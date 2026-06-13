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
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Gauge, Paragraph, Widget},
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

fn overlay_bbox_color(
    text: &mut [Vec<char>],
    colors: &mut [Vec<(u8, u8, u8)>],
    fw: usize, fh: usize,
    bbox: (usize, usize, usize, usize),
    ascii_cols: usize, ascii_rows: usize,
    col: (u8, u8, u8),
    _label: &str,
) {
    let (x1, y1, x2, y2) = bbox;
    if x2 <= x1 || y2 <= y1 { return; }

    let ax1 = (x1 * ascii_cols / fw).clamp(0, ascii_cols.saturating_sub(1));
    let ax2 = (x2 * ascii_cols / fw).clamp(0, ascii_cols.saturating_sub(1));
    let ay1 = (y1 * ascii_rows / fh).clamp(0, ascii_rows.saturating_sub(1));
    let ay2 = (y2 * ascii_rows / fh).clamp(0, ascii_rows.saturating_sub(1));

    if ax2 - ax1 < 2 || ay2 - ay1 < 2 { return; }

    for x in ax1..=ax2 {
        if x < ascii_cols {
            if ay1 < ascii_rows { text[ay1][x] = '─'; colors[ay1][x] = col; }
            if ay2 < ascii_rows { text[ay2][x] = '─'; colors[ay2][x] = col; }
        }
    }
    for y in ay1..=ay2 {
        if y < ascii_rows {
            if ax1 < ascii_cols { text[y][ax1] = '│'; colors[y][ax1] = col; }
            if ax2 < ascii_cols { text[y][ax2] = '│'; colors[y][ax2] = col; }
        }
    }
    if ay1 < ascii_rows && ax1 < ascii_cols { text[ay1][ax1] = '┌'; colors[ay1][ax1] = col; }
    if ay1 < ascii_rows && ax2 < ascii_cols { text[ay1][ax2] = '┐'; colors[ay1][ax2] = col; }
    if ay2 < ascii_rows && ax1 < ascii_cols { text[ay2][ax1] = '└'; colors[ay2][ax1] = col; }
    if ay2 < ascii_rows && ax2 < ascii_cols { text[ay2][ax2] = '┘'; colors[ay2][ax2] = col; }
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

// =========================== Multi-object tracking ===========================

fn is_skin(r: u8, g: u8, b: u8) -> bool {
    let r = r as i32;
    let g = g as i32;
    let b = b as i32;
    r > 60 && g > 30 && b > 15 && r > g && r > b && (r - g).abs() > 12
}

struct Blob {
    label: u32,
    pixels: usize,
    cx: f32,
    cy: f32,
    min_x: usize,
    max_x: usize,
    min_y: usize,
    max_y: usize,
}

/// Two-pass connected component labeling on a stepped skin mask.
fn find_blobs(rgb_data: &[u8], w: usize, h: usize, step: usize) -> Vec<Blob> {
    let step = step.max(2);
    let sw = w / step;
    let sh = h / step;
    if sw < 2 || sh < 2 { return vec![]; }

    // Build low-res mask
    let mut mask = vec![false; sw * sh];
    for y in 0..sh {
        for x in 0..sw {
            let i = (y * step * w + x * step) * 3;
            mask[y * sw + x] = is_skin(rgb_data[i], rgb_data[i + 1], rgb_data[i + 2]);
        }
    }

    // First pass: provisional labels
    let mut labels = vec![0u32; sw * sh];
    let mut next_label = 1u32;
    let mut eq: Vec<(u32, u32)> = Vec::new();

    for y in 0..sh {
        for x in 0..sw {
            if !mask[y * sw + x] { continue; }
            let l = if x > 0 { labels[y * sw + x - 1] } else { 0 };
            let u = if y > 0 { labels[(y - 1) * sw + x] } else { 0 };
            if l == 0 && u == 0 {
                labels[y * sw + x] = next_label;
                next_label += 1;
            } else if l != 0 && u != 0 && l != u {
                let (a, b) = (l.min(u), l.max(u));
                labels[y * sw + x] = a;
                eq.push((a, b));
            } else {
                labels[y * sw + x] = if l != 0 { l } else { u };
            }
        }
    }

    let total = next_label;
    if total == 1 { return vec![]; }

    // Resolve equivalences (DSU)
    let mut parent: Vec<u32> = (0..total).collect();
    fn find(parent: &mut [u32], x: u32) -> u32 {
        let xi = x as usize;
        if parent[xi] != x {
            parent[xi] = find(parent, parent[xi]);
        }
        parent[xi]
    }
    for &(a, b) in &eq {
        let ra = find(&mut parent, a);
        let rb = find(&mut parent, b);
        if ra != rb { parent[rb as usize] = ra; }
    }
    for i in 0..total as usize { let pi = parent[i]; parent[i] = find(&mut parent, pi); }

    // Second pass: rewrite labels
    let mut label_map: Vec<u32> = vec![0; total as usize];
    let mut unique_count = 0u32;
    for i in 1..total as usize {
        let p = parent[i] as usize;
        if label_map[p] == 0 { unique_count += 1; label_map[p] = unique_count; }
    }

    // Accumulate component stats
    let mut blobs: Vec<Blob> = (0..unique_count).map(|_| Blob {
        label: 0, pixels: 0, cx: 0.0, cy: 0.0,
        min_x: sw, max_x: 0, min_y: sh, max_y: 0,
    }).collect();

    for y in 0..sh {
        for x in 0..sw {
            if !mask[y * sw + x] { continue; }
            let orig = labels[y * sw + x];
            let final_label = label_map[parent[orig as usize] as usize] - 1;
            let b = &mut blobs[final_label as usize];
            b.label = final_label;
            b.pixels += 1;
            b.cx += x as f32;
            b.cy += y as f32;
            b.min_x = b.min_x.min(x);
            b.max_x = b.max_x.max(x);
            b.min_y = b.min_y.min(y);
            b.max_y = b.max_y.max(y);
        }
    }

    // Convert from stepped coords to pixel coords, compute centroids
    for b in &mut blobs {
        b.cx = b.cx / b.pixels as f32 * step as f32;
        b.cy = b.cy / b.pixels as f32 * step as f32;
        b.min_x *= step;
        b.max_x = (b.max_x + 1) * step;
        b.min_y *= step;
        b.max_y = (b.max_y + 1) * step;
    }

    // Filter small blobs
    blobs.retain(|b| b.pixels >= 5);
    blobs.sort_by(|a, b| b.pixels.cmp(&a.pixels));
    blobs
}

/// Classify blobs into named body parts by position & size.
fn classify_blobs(blobs: &[Blob], w: usize, h: usize) -> Vec<(&'static str, (usize, usize, usize, usize), f32, f32)> {
    let mut result: Vec<(&str, (usize, usize, usize, usize), f32, f32)> = Vec::new();
    let fw = w as f32;
    let fh = h as f32;

    for b in blobs {
        let bw = (b.max_x - b.min_x) as f32 / fw;
        let bh = (b.max_y - b.min_y) as f32 / fh;
        let nx = b.cx / fw;
        let ny = b.cy / fh;
        let area = bw * bh;
        let bbox = (b.min_x, b.min_y, b.max_x, b.max_y);

        // Head: high in frame, moderate size, roughly circular
        if ny < 0.45 && area < 0.25 && bh < 0.35 && bw * 1.5 > bh {
            result.push(("head", bbox, nx, ny));
        // Fingers: small, elongated, at edges of hand region
        } else if area < 0.06 && (bw < 0.15 || bh < 0.15) {
            result.push(("finger", bbox, nx, ny));
        // Hand: medium, mid-frame
        } else if area < 0.3 && ny < 0.75 && ny > 0.1 {
            result.push(("hand", bbox, nx, ny));
        // Body: large, lower frame
        } else if area > 0.08 && ny > 0.3 {
            result.push(("body", bbox, nx, ny));
        // Everything else
        } else {
            result.push(("obj", bbox, nx, ny));
        }
    }
    result
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

    let mut prev_hx = 0.5f32;
    let mut prev_hy = 0.5f32;

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

        // Multi-object tracking (find & classify skin blobs)
        let step = 4usize.max(fw / 80);
        let blobs = find_blobs(&rgb, fw, fh, step);
        let objects = classify_blobs(&blobs, fw, fh);
        let bboxes: Vec<(usize, usize, usize, usize)> = objects.iter().map(|(_, b, _, _)| *b).collect();

        // Per-object audio mapping
        let mut hand_bbox = (0, 0, 0, 0);
        let mut head_nx = 0.5f32;
        let mut head_ny = 0.5f32;
        let mut has_hand = false;
        let mut has_head = false;
        let mut finger_count = 0usize;
        let mut body_nx = 0.5f32;

        for (label, bbox, nx, ny) in &objects {
            let (x1, y1, x2, y2) = bbox;
            let bw = (x2 - x1) as f32 / fw as f32;
            let bh = (y2 - y1) as f32 / fh as f32;
            match *label {
                "hand"  => { hand_bbox = *bbox; has_hand = true; }
                "head"  => { head_nx = *nx; head_ny = *ny; has_head = true; }
                "finger"=> { finger_count += 1; }
                "body"  => { body_nx = *nx; }
                _       => {}
            }
        }

        if let Ok(mut p) = gesture_params.lock() {
            p.has_hand = has_hand;
            p.has_head = has_head;
            p.finger_count = finger_count as f32;

            if has_hand {
                let (x1, y1, x2, y2) = hand_bbox;
                let hx = (x1 + x2) as f32 / (2.0 * fw as f32);
                let hy = (y1 + y2) as f32 / (2.0 * fh as f32);
                let hs = ((x2 - x1) * (y2 - y1)) as f32 / (fw * fh) as f32;

                // Velocity — movement speed triggers percussive bursts
                let dx = hx - prev_hx;
                let dy = hy - prev_hy;
                let vel = (dx * dx + dy * dy).sqrt() * 12.0;
                let v_boost = vel.min(1.0);
                prev_hx = hx;
                prev_hy = hy;

                p.hand_x = hx;
                p.hand_y = hy;
                p.hand_size = hs.min(1.0);

                // ---- Hand X → pan (full range) ----
                p.pan = hx.clamp(0.0, 1.0);

                // ---- Velocity → detune wobble (speed = pitch shimmy) ----
                p.detune = 1.0 + v_boost * 0.04;

                // ---- Hand Y → freq & cutoff (linear, instant) ----
                p.freq = 40.0 + (1.0 - hy) * 1960.0;
                p.cutoff = 30.0 + (1.0 - hy) * 9970.0;

                // ---- Hand size → gain & modulation ----
                p.mod_amt = ((1.0 - hs) * 0.6).min(0.5);
                p.mod_freq = 0.5 + hs * 29.5;
                p.gain = (0.04 + hs * 0.8).max(0.04).min(0.9);

                // ---- Velocity → gain burst + mod sweep ----
                if vel > 0.05 {
                    p.gain = (p.gain + v_boost * 0.3).min(0.9);
                    p.mod_freq = (p.mod_freq + v_boost * 30.0).min(30.0);
                    p.mod_amt = (p.mod_amt + v_boost * 0.3).min(0.6);
                    // Direction: moving up = pitch sweep
                    if dy < 0.0 {
                        p.freq = (p.freq + v_boost * 500.0).min(2000.0);
                    }
                }

                // ---- Fingers → osc & filter combos ----
                let fc = finger_count.min(5);
                let osc_table = [0.0, 1.0, 0.0, 2.0, 3.0, 2.0];
                let flt_table = [0.0, 0.0, 1.0, 1.0, 2.0, 2.0];
                p.osc_type = osc_table[fc];
                p.filter_type = flt_table[fc];

                // ---- Head → reverb mix ----
                if has_head {
                    p.reverb_mix = (0.7 - head_ny * 0.6).clamp(0.0, 0.7);
                    p.head_x = head_nx;
                    p.head_y = head_ny;
                }
            } else {
                // No hand → instant silence
                p.gain = 0.0;
                p.mod_freq = 1.0;
                p.detune = 1.0;
                prev_hx = 0.5;
                prev_hy = 0.5;

                if has_head {
                    p.reverb_mix = (0.5 - head_ny * 0.4).clamp(0.0, 0.5);
                    p.freq = 80.0 + (1.0 - head_ny) * 400.0;
                    p.cutoff = 100.0 + (1.0 - head_ny) * 2000.0;
                    p.gain = 0.08;
                }
            }
        }

        let Ok((cols, rows)) = crossterm::terminal::size() else {
            continue;
        };
        let cols = cols as usize;
        let rows = rows as usize;

        if cols > 0 && rows > 0 {
            let info_rows = 4usize;
            if rows > info_rows + 5 {
                let ascii_rows = rows - info_rows;
                let (mut text, mut colors) = build_ascii_frame(&rgb, fw, fh, cols, ascii_rows, use_color);
                // Draw all detected object bounding boxes (each with different color)
                let bbox_colors = [(0,255,0),(255,0,0),(0,255,255),(255,255,0),(255,128,0)];
                for (idx, &(ref label, bbox, _, _)) in objects.iter().enumerate() {
                    let col = bbox_colors[idx % bbox_colors.len()];
                    overlay_bbox_color(&mut text, &mut colors, fw, fh, bbox, cols, ascii_rows, col, label);
                }

                if let Err(e) = terminal.draw(|f| {
                    let area = f.area();
                    let ascii_area = Rect::new(0, 0, area.width, area.height - info_rows as u16);
                    f.render_widget(AsciiWidget { text, colors }, ascii_area);

                    // Parameter box overlay
                    let info_rows = 4usize;
                    let info_area = Rect::new(0, area.height - info_rows as u16, area.width, info_rows as u16);
                    let gs = gesture_params.lock().unwrap();
                    let osc_names = ["Sin", "Saw", "Sq ", "Tri"];
                    let flt_names = ["LPF", "BPF", "HPF"];
                    let oidx = gs.osc_type as usize;
                    let fidx = gs.filter_type as usize;
                    let stat = if gs.has_hand { "HAND" } else if gs.has_head { "FACE" } else { "--" };
                    let fc = gs.finger_count as usize;
                    let hd = if gs.has_head { format!("Hd{:.2}", gs.head_y) } else { "---".to_string() };

                    let chunks = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints([Constraint::Length(1), Constraint::Length(1), Constraint::Length(1), Constraint::Length(1)].as_ref())
                        .split(info_area);

                    // Row 0: status
                    let status_line = format!(
                        " {}  {}:{}  {}F  {}  Pan:{:.2}  [q]uit [c]olor({})",
                        stat, osc_names[oidx.min(3)], flt_names[fidx.min(2)],
                        fc, hd, gs.pan,
                        if use_color { "on" } else { "off" },
                    );
                    let block = Block::default().borders(Borders::TOP).title("Synth");
                    f.render_widget(Paragraph::new(status_line).block(block), chunks[0]);

                    // Rows 1-3: parameter bars in 2 columns
                    let bar_chunks = Layout::default()
                        .direction(Direction::Horizontal)
                        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                        .split(chunks[1]);

                    let freq_norm = ((gs.freq - 40.0) / 1960.0).clamp(0.0, 1.0);
                    let cut_norm = ((gs.cutoff - 30.0) / 9970.0).clamp(0.0, 1.0);
                    let gain_norm = gs.gain;
                    let mod_norm = gs.mod_amt / 0.5;
                    let rev_norm = gs.reverb_mix / 0.7;
                    let det_norm = ((gs.detune - 0.99) / 0.06).clamp(0.0, 1.0);

                    f.render_widget(
                        Gauge::default()
                            .block(Block::default().borders(Borders::NONE))
                            .gauge_style(Style::default().fg(Color::Cyan))
                            .percent((freq_norm * 100.0) as u16)
                            .label(format!("Freq {:.0}Hz", gs.freq)),
                        bar_chunks[0],
                    );
                    f.render_widget(
                        Gauge::default()
                            .block(Block::default().borders(Borders::NONE))
                            .gauge_style(Style::default().fg(Color::Magenta))
                            .percent((cut_norm * 100.0) as u16)
                            .label(format!("Cut {:.0}Hz", gs.cutoff)),
                        bar_chunks[1],
                    );

                    let bar_chunks2 = Layout::default()
                        .direction(Direction::Horizontal)
                        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                        .split(chunks[2]);

                    f.render_widget(
                        Gauge::default()
                            .block(Block::default().borders(Borders::NONE))
                            .gauge_style(Style::default().fg(Color::Green))
                            .percent((gain_norm * 100.0) as u16)
                            .label(format!("Gain {:.2}", gs.gain)),
                        bar_chunks2[0],
                    );
                    f.render_widget(
                        Gauge::default()
                            .block(Block::default().borders(Borders::NONE))
                            .gauge_style(Style::default().fg(Color::Yellow))
                            .percent((mod_norm * 100.0) as u16)
                            .label(format!("Mod {:.2}", gs.mod_amt)),
                        bar_chunks2[1],
                    );

                    let bar_chunks3 = Layout::default()
                        .direction(Direction::Horizontal)
                        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                        .split(chunks[3]);

                    f.render_widget(
                        Gauge::default()
                            .block(Block::default().borders(Borders::NONE))
                            .gauge_style(Style::default().fg(Color::Red))
                            .percent((rev_norm * 100.0) as u16)
                            .label(format!("Rev {:.2}", gs.reverb_mix)),
                        bar_chunks3[0],
                    );
                    f.render_widget(
                        Gauge::default()
                            .block(Block::default().borders(Borders::NONE))
                            .gauge_style(Style::default().fg(Color::Blue))
                            .percent((det_norm * 100.0) as u16)
                            .label(format!("Det {:.3}", gs.detune)),
                        bar_chunks3[1],
                    );
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
