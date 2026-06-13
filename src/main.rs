mod faust_gen;

use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use nokhwa::{
    pixel_format::RgbFormat,
    utils::{CameraIndex, RequestedFormat, RequestedFormatType},
    Camera,
};
use ratatui::{
    backend::CrosstermBackend,
    layout::Rect,
    style::Color,
    widgets::{Block, Borders, Paragraph, Widget},
    Terminal,
};

// =========================== FAUST types ===========================

type F32 = f32;
type F64 = f64;

#[derive(Copy, Clone)]
pub struct ParamIndex(pub i32);

pub trait FaustDsp {
    type T;
    fn new() -> Self
    where
        Self: Sized;
    fn metadata(&self, m: &mut dyn Meta);
    fn get_sample_rate(&self) -> i32;
    fn get_num_inputs(&self) -> i32;
    fn get_num_outputs(&self) -> i32;
    fn class_init(sample_rate: i32)
    where
        Self: Sized;
    fn instance_reset_params(&mut self);
    fn instance_clear(&mut self);
    fn instance_constants(&mut self, sample_rate: i32);
    fn instance_init(&mut self, sample_rate: i32);
    fn init(&mut self, sample_rate: i32);
    fn build_user_interface(&self, ui_interface: &mut dyn UI<Self::T>);
    fn build_user_interface_static(ui_interface: &mut dyn UI<Self::T>)
    where
        Self: Sized;
    fn get_param(&self, param: ParamIndex) -> Option<Self::T>;
    fn set_param(&mut self, param: ParamIndex, value: Self::T);
    fn compute(&mut self, count: i32, inputs: &[&[Self::T]], outputs: &mut [&mut [Self::T]]);
}

pub trait Meta {
    fn declare(&mut self, key: &str, value: &str);
}

pub trait UI<T> {
    fn open_tab_box(&mut self, label: &str);
    fn open_horizontal_box(&mut self, label: &str);
    fn open_vertical_box(&mut self, label: &str);
    fn close_box(&mut self);
    fn add_button(&mut self, label: &str, param: ParamIndex);
    fn add_check_button(&mut self, label: &str, param: ParamIndex);
    fn add_vertical_slider(
        &mut self,
        label: &str,
        param: ParamIndex,
        init: T,
        min: T,
        max: T,
        step: T,
    );
    fn add_horizontal_slider(
        &mut self,
        label: &str,
        param: ParamIndex,
        init: T,
        min: T,
        max: T,
        step: T,
    );
    fn add_num_entry(&mut self, label: &str, param: ParamIndex, init: T, min: T, max: T, step: T);
    fn add_horizontal_bargraph(&mut self, label: &str, param: ParamIndex, min: T, max: T);
    fn add_vertical_bargraph(&mut self, label: &str, param: ParamIndex, min: T, max: T);
    fn declare(&mut self, param: Option<ParamIndex>, key: &str, value: &str);
}

// =========================== Gesture params ===========================

#[derive(Debug, Clone, Copy)]
pub struct GestureParams {
    pub freq: f32,
    pub cutoff: f32,
    pub gain: f32,
    pub mod_amt: f32,
    pub mod_freq: f32,
    pub osc_type: f32,
    pub pan: f32,
    pub detune: f32,
    pub filter_type: f32,
    pub reverb_mix: f32,
    pub hand_x: f32,
    pub hand_y: f32,
    pub hand_size: f32,
    pub finger_count: f32,
    pub hand_openness: f32,
    pub head_x: f32,
    pub head_y: f32,
    pub has_hand: bool,
    pub has_head: bool,
}

impl Default for GestureParams {
    fn default() -> Self {
        Self {
            freq: 440.0,
            cutoff: 2000.0,
            gain: 0.3,
            mod_amt: 0.05,
            mod_freq: 3.0,
            osc_type: 0.0,
            pan: 0.5,
            detune: 1.001,
            filter_type: 0.0,
            reverb_mix: 0.15,
            hand_x: 0.5,
            hand_y: 0.5,
            hand_size: 0.1,
            finger_count: 0.0,
            hand_openness: 0.0,
            head_x: 0.5,
            head_y: 0.5,
            has_hand: false,
            has_head: false,
        }
    }
}

// =========================== Audio engine ===========================

struct AudioEngine {
    dsp: faust_gen::mydsp,
    params: Arc<Mutex<GestureParams>>,
}

impl AudioEngine {
    fn new(params: Arc<Mutex<GestureParams>>) -> Self {
        let mut dsp = faust_gen::mydsp::new();
        dsp.init(44100);
        Self { dsp, params }
    }

    fn process(&mut self, output: &mut [f32], channels: usize) {
        if let Ok(params) = self.params.lock() {
            self.dsp.set_param(ParamIndex(0), params.cutoff);
            self.dsp.set_param(ParamIndex(1), params.detune);
            self.dsp.set_param(ParamIndex(2), params.filter_type);
            self.dsp.set_param(ParamIndex(3), params.freq);
            self.dsp.set_param(ParamIndex(4), params.gain);
            self.dsp.set_param(ParamIndex(5), params.mod_amt);
            self.dsp.set_param(ParamIndex(6), params.mod_freq);
            self.dsp.set_param(ParamIndex(7), params.osc_type);
            self.dsp.set_param(ParamIndex(8), params.pan);
            self.dsp.set_param(ParamIndex(9), params.reverb_mix);
        }

        let num_frames = output.len() / channels;
        let mut left_out = vec![0.0f32; num_frames];
        let mut right_out = vec![0.0f32; num_frames];
        let outputs = &mut [left_out.as_mut_slice(), right_out.as_mut_slice()];
        self.dsp.compute(num_frames as i32, &[], outputs);

        for i in 0..num_frames {
            if channels == 2 {
                output[i * 2] = left_out[i];
                output[i * 2 + 1] = right_out[i];
            } else {
                output[i] = left_out[i];
            }
        }
    }
}

fn start_audio(
    params: Arc<Mutex<GestureParams>>,
) -> Result<cpal::Stream, Box<dyn std::error::Error>> {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .ok_or("no audio output device")?;
    let config = device.default_output_config()?;
    let sample_rate = config.sample_rate().0;
    let channels = config.channels() as usize;

    println!(
        "Audio: {} Hz, {} ch, device: {}",
        sample_rate,
        channels,
        device.name().unwrap_or_default()
    );

    let mut engine = AudioEngine::new(params);
    engine.dsp.init(sample_rate as i32);

    let stream = device.build_output_stream(
        &config.into(),
        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            engine.process(data, channels);
        },
        |err| eprintln!("audio error: {}", err),
        None,
    )?;

    stream.play()?;
    Ok(stream)
}

// =========================== ASCII rendering ===========================

const RAMP: &[u8] = b" .'`^\",:;Il!i><~+_-?][}{1)(|/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$";
const RAMP_LEN: usize = RAMP.len();

const BAYER: [[f32; 4]; 4] = [
    [0.0, 8.0, 2.0, 10.0],
    [12.0, 4.0, 14.0, 6.0],
    [3.0, 11.0, 1.0, 9.0],
    [15.0, 7.0, 13.0, 5.0],
];

const GAMMA: f32 = 1.0 / 1.4;

fn gamma_correct(v: u8) -> u8 {
    let f = v as f32 / 255.0;
    (f.powf(GAMMA) * 255.0) as u8
}

fn rgb_to_gray(r: u8, g: u8, b: u8) -> u8 {
    gamma_correct(((r as u32 * 77 + g as u32 * 150 + b as u32 * 29) >> 8) as u8)
}

fn sharpen_3x3(src: &[u8], w: usize, h: usize) -> Vec<u8> {
    let mut dst = src.to_vec();
    if w < 3 || h < 3 {
        return dst;
    }
    for y in 1..h - 1 {
        for x in 1..w - 1 {
            let i = y * w + x;
            let v = src[i] as i32 * 5 - src[(y - 1) * w + x] as i32
                - src[(y + 1) * w + x] as i32
                - src[y * w + x - 1] as i32
                - src[y * w + x + 1] as i32;
            dst[i] = v.clamp(0, 255) as u8;
        }
    }
    dst
}

fn resize_nearest(src: &[u8], sw: usize, sh: usize, dw: usize, dh: usize) -> Vec<u8> {
    if dw == 0 || dh == 0 {
        return vec![];
    }
    let mut dst = vec![0u8; dw * dh];
    for dy in 0..dh {
        let sy = dy * sh / dh;
        for dx in 0..dw {
            let sx = dx * sw / dw;
            dst[dy * dw + dx] = src[sy * sw + sx];
        }
    }
    dst
}

fn build_ascii_frame(
    rgb_data: &[u8],
    frame_w: usize,
    frame_h: usize,
    cols: usize,
    rows: usize,
    use_color: bool,
) -> (Vec<Vec<char>>, Vec<Vec<(u8, u8, u8)>>) {
    let mut gray = Vec::with_capacity(frame_w * frame_h);
    let mut rgb_small_colors: Vec<(u8, u8, u8)> = Vec::new();

    for y in 0..frame_h {
        for x in 0..frame_w {
            let i = (y * frame_w + x) * 3;
            let r = rgb_data[i];
            let g = rgb_data[i + 1];
            let b = rgb_data[i + 2];
            gray.push(rgb_to_gray(r, g, b));
        }
    }

    let gray = sharpen_3x3(&gray, frame_w, frame_h);
    let gray = resize_nearest(&gray, frame_w, frame_h, cols, rows);

    if use_color {
        rgb_small_colors.reserve(cols * rows);
        for dy in 0..rows {
            let sy = dy * frame_h / rows;
            for dx in 0..cols {
                let sx = dx * frame_w / cols;
                let i = (sy * frame_w + sx) * 3;
                rgb_small_colors.push((rgb_data[i], rgb_data[i + 1], rgb_data[i + 2]));
            }
        }
    }

    let mut text = Vec::with_capacity(rows);
    let mut fg_colors = Vec::with_capacity(if use_color { rows } else { 0 });

    for y in 0..rows {
        let mut text_row = Vec::with_capacity(cols);
        let mut color_row = Vec::with_capacity(if use_color { cols } else { 0 });
        for x in 0..cols {
            let g = gray[y * cols + x];
            let bayer_val = BAYER[y % 4][x % 4] / 16.0;
            let idx = ((g as f32 / 255.0 * RAMP_LEN as f32) + bayer_val)
                .clamp(0.0, (RAMP_LEN - 1) as f32) as usize;
            text_row.push(RAMP[idx] as char);
            if use_color {
                color_row.push(rgb_small_colors[y * cols + x]);
            }
        }
        text.push(text_row);
        if use_color {
            fg_colors.push(color_row);
        }
    }

    (text, fg_colors)
}

struct AsciiWidget {
    text: Vec<Vec<char>>,
    colors: Vec<Vec<(u8, u8, u8)>>,
}

impl Widget for AsciiWidget {
    fn render(self, area: Rect, buf: &mut ratatui::buffer::Buffer) {
        for (y, row) in self.text.iter().enumerate() {
            if y >= area.height as usize {
                break;
            }
            for (x, &ch) in row.iter().enumerate() {
                if x >= area.width as usize {
                    break;
                }
                let cell = &mut buf[(area.x + x as u16, area.y + y as u16)];
                let mut s = [0u8; 4];
                let symbol = ch.encode_utf8(&mut s);
                cell.set_symbol(symbol);
                if let Some(colors) = self.colors.get(y) {
                    if let Some(&(r, g, b)) = colors.get(x) {
                        cell.set_fg(Color::Rgb(r, g, b));
                    }
                }
            }
        }
    }
}

// =========================== Gesture tracking ===========================

fn is_skin(r: u8, g: u8, b: u8) -> bool {
    let r = r as i32;
    let g = g as i32;
    let b = b as i32;
    r > 60 && g > 30 && b > 15 && r > g && r > b && (r - g).abs() > 12
}

struct HandResult {
    hand_x: f32,
    hand_y: f32,
    hand_bbox_size: f32,
    finger_count: usize,
    hand_openness: f32,
    head_x: f32,
    head_y: f32,
    has_hand: bool,
    has_head: bool,
}

fn track_hand(rgb_data: &[u8], w: usize, h: usize) -> HandResult {
    let step = 4usize.max(w / 80);

    // Pass 1: collect skin pixels and compute overall centroid/bbox
    let mut skin_pixels: Vec<(usize, usize)> = Vec::with_capacity((w * h) / (step * step));
    let mut total_x: f32 = 0.0;
    let mut total_y: f32 = 0.0;
    let mut min_x = w as f32;
    let mut max_x = 0.0f32;
    let mut min_y = h as f32;
    let mut max_y = 0.0f32;

    for y in (0..h).step_by(step) {
        for x in (0..w).step_by(step) {
            let i = (y * w + x) * 3;
            if is_skin(rgb_data[i], rgb_data[i + 1], rgb_data[i + 2]) {
                skin_pixels.push((x, y));
                total_x += x as f32;
                total_y += y as f32;
                if (x as f32) < min_x { min_x = x as f32; }
                if (x as f32) > max_x { max_x = x as f32; }
                if (y as f32) < min_y { min_y = y as f32; }
                if (y as f32) > max_y { max_y = y as f32; }
            }
        }
    }

    if skin_pixels.len() < 10 {
        return HandResult {
            hand_x: 0.5, hand_y: 0.5, hand_bbox_size: 0.1,
            finger_count: 0, hand_openness: 0.0,
            head_x: 0.5, head_y: 0.5,
            has_hand: false, has_head: false,
        };
    }

    let n = skin_pixels.len() as f32;
    let cx = total_x / n;
    let cy = total_y / n;

    // Normalize to 0-1
    let hand_x = cx / w as f32;
    let hand_y = cy / h as f32;
    let bbox_w = (max_x - min_x) / w as f32;
    let bbox_h = (max_y - min_y) / h as f32;
    let hand_bbox_size = (bbox_w * bbox_h).min(1.0);

    // Pass 2: radial profile from centroid for finger counting
    let num_angles = 36;
    let angle_step = (std::f32::consts::TAU / num_angles as f32);
    let mut radial_dist = vec![0.0f32; num_angles];

    for &(px, py) in &skin_pixels {
        let dx = px as f32 - cx;
        let dy = py as f32 - cy;
        let dist = (dx * dx + dy * dy).sqrt();
        let angle = dy.atan2(dx) + std::f32::consts::PI; // 0..TAU
        let idx = (angle / angle_step) as usize % num_angles;
        if dist > radial_dist[idx] {
            radial_dist[idx] = dist;
        }
    }

    // Smooth radial profile (3-point moving average)
    let mut smoothed = radial_dist.clone();
    for i in 0..num_angles {
        let prev = radial_dist[(i + num_angles - 1) % num_angles];
        let next = radial_dist[(i + 1) % num_angles];
        smoothed[i] = (prev + radial_dist[i] + next) / 3.0;
    }

    // Find mean distance to normalize
    let mean_dist: f32 = smoothed.iter().sum::<f32>() / num_angles as f32;
    if mean_dist < 2.0 {
        return HandResult {
            hand_x, hand_y, hand_bbox_size,
            finger_count: 0, hand_openness: 0.0,
            head_x: 0.5, head_y: 0.5,
            has_hand: true, has_head: false,
        };
    }

    // Count peaks in upper half (angles 9..27 correspond to top of hand in image coords)
    // In image coords: angle 0 = right, angle 9 = bottom, angle 18 = left, angle 27 = top
    // Fingers extend upward => angles around 27 (top) to 9 (bottom wrapping)
    let peak_angle_start = 18 + 4;  // slightly left of top
    let peak_angle_end = 36 - 4;    // slightly right of top
    let peak_threshold = 1.25;

    let mut finger_count = 0usize;
    for i in peak_angle_start..peak_angle_end {
        let ii = i % num_angles;
        let prev = smoothed[(ii + num_angles - 1) % num_angles];
        let next = smoothed[(ii + 1) % num_angles];
        if smoothed[ii] > prev && smoothed[ii] > next && smoothed[ii] > mean_dist * peak_threshold {
            finger_count += 1;
        }
    }

    // Hand openness: coefficient of variation of radial distances
    let variance: f32 = smoothed.iter().map(|d| (d - mean_dist).powi(2)).sum::<f32>() / num_angles as f32;
    let std_dev = variance.sqrt();
    let hand_openness = (std_dev / mean_dist).min(1.0);

    // Head tracking: find skin pixels in upper-center region of frame
    let head_roi_top = 0usize;
    let head_roi_bottom = (h as f32 * 0.4) as usize;
    let head_roi_left = (w as f32 * 0.15) as usize;
    let head_roi_right = (w as f32 * 0.85) as usize;
    let mut head_total_x = 0.0f32;
    let mut head_total_y = 0.0f32;
    let mut head_count = 0.0f32;

    for y in (head_roi_top..head_roi_bottom).step_by(step) {
        for x in (head_roi_left..head_roi_right).step_by(step) {
            let i = (y * w + x) * 3;
            if is_skin(rgb_data[i], rgb_data[i + 1], rgb_data[i + 2]) {
                // Only count if it's not part of the hand (above the hand)
                if (y as f32) < cy - (max_y - min_y) * 0.3 {
                    head_total_x += x as f32;
                    head_total_y += y as f32;
                    head_count += 1.0;
                }
            }
        }
    }

    let has_head = head_count > 5.0;
    let head_x = if has_head { head_total_x / head_count / w as f32 } else { 0.5 };
    let head_y = if has_head { head_total_y / head_count / h as f32 } else { 0.5 };

    HandResult {
        hand_x, hand_y, hand_bbox_size,
        finger_count: finger_count.min(5),
        hand_openness,
        head_x, head_y,
        has_hand: true, has_head,
    }
}

// =========================== Main ===========================

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut camera = Camera::new(
        CameraIndex::Index(0),
        RequestedFormat::new::<RgbFormat>(RequestedFormatType::None),
    )?;
    camera.open_stream()?;

    let gesture_params = Arc::new(Mutex::new(GestureParams::default()));
    let _audio_stream = start_audio(gesture_params.clone())?;

    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    let mut use_color =
        std::env::var("COLORTERM").map_or(false, |v| v == "truecolor" || v == "24bit");
    let frame_time = Duration::from_secs_f64(1.0 / 30.0);

    let result = loop {
        let frame_start = Instant::now();

        let buffer = match camera.frame() {
            Ok(b) => b,
            Err(e) => break Err(Box::new(e) as Box<dyn std::error::Error>),
        };
        let rgb = buffer.decode_image::<RgbFormat>()?;
        let res = buffer.resolution();
        let fw = res.width_x as usize;
        let fh = res.height_y as usize;

        // Gesture tracking
        let hr = track_hand(&rgb, fw, fh);

        // Map gesture to audio params
        if let Ok(mut p) = gesture_params.lock() {
            p.has_hand = hr.has_hand;
            p.has_head = hr.has_head;
            p.hand_x = hr.hand_x;
            p.hand_y = hr.hand_y;
            p.hand_size = hr.hand_bbox_size;
            p.finger_count = hr.finger_count as f32;
            p.hand_openness = hr.hand_openness;
            p.head_x = hr.head_x;
            p.head_y = hr.head_y;

            if hr.has_hand {
                let hx = hr.hand_x;
                let hy = hr.hand_y;
                let hs = hr.hand_bbox_size;
                let fc = hr.finger_count;

                // Fingers → oscillator type & filter type
                p.osc_type = (fc as f32 / 5.0 * 3.5).floor().clamp(0.0, 3.0);
                p.filter_type = (fc as f32 / 5.0 * 2.5).floor().clamp(0.0, 2.0);

                // X → pan & detune
                p.pan = hx.clamp(0.0, 1.0);
                p.detune = 0.99 + hx * 0.06;

                // Y → freq & cutoff
                p.freq = 40.0 + (1.0 - hy) * 1960.0;
                p.cutoff = 30.0 + (1.0 - hy) * 9970.0;

                // Hand size → gain & modulation
                p.mod_amt = (hs * 0.6).min(0.5);
                p.mod_freq = 0.5 + (1.0 - hs) * 29.5;

                // Head Y → reverb mix (head higher = more reverb)
                if hr.has_head {
                    p.reverb_mix = (0.7 - hr.head_y * 0.6).clamp(0.0, 0.7);
                }

                // Openness → gain (open palm = louder)
                p.gain = (0.04 + hr.hand_openness * 0.5).max(0.04).min(0.9);
            }
        }

        let Ok((cols, rows)) = crossterm::terminal::size() else {
            continue;
        };
        let cols = cols as usize;
        let rows = rows as usize;

        if cols > 0 && rows > 0 {
            let info_rows = 3usize;
            if rows > info_rows + 5 {
                let ascii_rows = rows - info_rows;
                let (text, colors) = build_ascii_frame(&rgb, fw, fh, cols, ascii_rows, use_color);

                if let Err(e) = terminal.draw(|f| {
                    let area = f.area();
                    let ascii_area = Rect::new(0, 0, area.width, area.height - info_rows as u16);
                    f.render_widget(AsciiWidget { text, colors }, ascii_area);

                    // Info overlay
                    let info_area = Rect::new(0, area.height - info_rows as u16, area.width, info_rows as u16);
                    let gs = gesture_params.lock().unwrap();
                    let status = if gs.has_hand { "HAND" } else { "NONE" };
                    let osc_names = ["Sine", "Saw", "Sqre", "Tri "];
                    let flt_names = ["LPF", "BPF", "HPF"];
                    let oidx = gs.osc_type as usize;
                    let fidx = gs.filter_type as usize;
                    let fc = gs.finger_count as usize;
                    let open_bar = "░".repeat((gs.hand_openness * 8.0) as usize);
                    let info = format!(
                        " {} | {}:{}F {:.0}Hz/{:.0}ct | G:{:.2} M:{:.2}/{:.1} D:{:.3} R:{:.2} | P:{:.2}\n\
                         Fingers:{} | Open:{} ({:.2}) | Head:{:.2},{:.2} | [q]uit [c]olor({})",
                        status,
                        osc_names[oidx.min(3)], flt_names[fidx.min(2)],
                        gs.freq, gs.cutoff, gs.gain,
                        gs.mod_amt, gs.mod_freq, gs.detune, gs.reverb_mix,
                        gs.pan,
                        fc, open_bar, gs.hand_openness,
                        gs.head_x, gs.head_y,
                        if use_color { "on" } else { "off" }
                    );
                    let block = Block::default().borders(Borders::TOP).title("Synth");
                    f.render_widget(Paragraph::new(info).block(block), info_area);
                }) {
                    break Err(Box::new(e));
                }
            }
        }

        if event::poll(Duration::from_millis(1))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break Ok(()),
                    KeyCode::Char('c') => use_color = !use_color,
                    _ => {}
                }
            }
        }

        let elapsed = frame_start.elapsed();
        if elapsed < frame_time {
            std::thread::sleep(frame_time - elapsed);
        }
    };

    terminal.show_cursor()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    disable_raw_mode()?;

    drop(camera);
    println!("Camera to ASCII stopped.");

    result
}
