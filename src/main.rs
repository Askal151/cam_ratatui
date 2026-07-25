mod faust_gen;
mod tracker;

use std::collections::VecDeque;
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

use tracker::Blob;

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
    pub grain_rate: f32,
    pub grain_size: f32,
    pub grain_amt: f32,
    pub grain_scat: f32,
    pub hand_x: f32,
    pub hand_y: f32,
    pub hand_size: f32,
    pub finger_count: f32,
    pub hand_openness: f32,
    pub head_x: f32,
    pub head_y: f32,
    pub has_hand: bool,
    pub has_head: bool,
    pub korg_drive: f32,
    pub korg_ring: f32,
    pub variant_label: &'static str,
}

impl Default for GestureParams {
    fn default() -> Self {
        Self {
            freq: 440.0,
            cutoff: 2000.0,
            gain: 0.3,
            mod_amt: 0.0,
            mod_freq: 3.0,
            osc_type: 0.0,
            pan: 0.5,
            detune: 1.001,
            filter_type: 0.0,
            reverb_mix: 0.15,
            grain_rate: 4.0,
            grain_size: 0.12,
            grain_amt: 0.3,
            grain_scat: 0.2,
            hand_x: 0.5,
            hand_y: 0.5,
            hand_size: 0.1,
            finger_count: 0.0,
            hand_openness: 0.0,
            head_x: 0.5,
            head_y: 0.5,
            has_hand: false,
            has_head: false,
            korg_drive: 0.0,
            korg_ring: 0.0,
            variant_label: "Sin",
        }
    }
}

// =========================== Sampler / Recorder ===========================

const MAX_SAMPLE_LEN: usize = 44100 * 10; // 10 seconds max

#[derive(Clone)]
struct SampleLayer {
    buffer: Vec<f32>,
}

#[derive(Clone)]
struct SamplerState {
    sample: Vec<f32>,
    layers: Vec<SampleLayer>,
    mixed_buffer: Vec<f32>,
    recording: bool,
    has_sample: bool,
    sample_version: u64,
}

impl SamplerState {
    fn new() -> Self {
        Self {
            sample: Vec::with_capacity(MAX_SAMPLE_LEN),
            layers: Vec::new(),
            mixed_buffer: Vec::new(),
            recording: false,
            has_sample: false,
            sample_version: 0,
        }
    }

    fn remix(&mut self) {
        if self.layers.is_empty() {
            self.has_sample = false;
            self.mixed_buffer.clear();
            return;
        }
        let max_len = self.layers.iter().map(|l| l.buffer.len()).max().unwrap_or(0);
        let mut out = vec![0.0f32; max_len];
        let g = 1.0 / self.layers.len() as f32;
        for layer in &self.layers {
            for (i, &s) in layer.buffer.iter().enumerate() {
                out[i] += s * g;
            }
        }
        let peak = out.iter().fold(0.0f32, |a, &b| a.max(b.abs()));
        if peak > 0.95 {
            let scale = 0.95 / peak;
            for s in &mut out { *s *= scale; }
        }
        self.mixed_buffer = out;
        self.has_sample = true;
        self.sample_version += 1;
    }
}

struct Sampler {
    state: Arc<Mutex<SamplerState>>,
    sample_rate: f32,
}

impl Sampler {
    fn new(state: Arc<Mutex<SamplerState>>, _params: Arc<Mutex<GestureParams>>, sr: f32) -> Self {
        Self { state, sample_rate: sr }
    }
}

// =========================== Audio engine ===========================

struct DelayLine {
    buf: Vec<f32>,
    pos: usize,
    max_len: usize,
    smooth_time: f32,
}

impl DelayLine {
    fn new(max_sec: f32, sample_rate: f32) -> Self {
        let max_len = (max_sec * sample_rate) as usize;
        Self { buf: vec![0.0f32; max_len], pos: 0, max_len, smooth_time: 0.3 }
    }

    fn process(&mut self, left: &mut [f32], right: &mut [f32], time_s: f32, feedback: f32, mix: f32, sr: f32) {
        if mix <= 0.0 { return; }
        self.smooth_time += (time_s.clamp(0.05, 1.2) - self.smooth_time) * 0.05;
        let delay_samples = (self.smooth_time * sr).clamp(1.0, self.max_len as f32 - 1.0);
        let fb = (feedback * 0.95).clamp(0.0, 0.92);
        let wet = mix.clamp(0.0, 1.0);
        let dry = 1.0 - wet;

        for i in 0..left.len() {
            let dry_in_l = left[i];
            let dry_in_r = right[i];

            let read_pos = (self.pos as f32 + self.max_len as f32 - delay_samples) % self.max_len as f32;
            let idx = read_pos as usize;
            let frac = read_pos - idx as f32;
            let next = if idx + 1 < self.max_len { idx + 1 } else { 0 };
            let delayed = self.buf[idx] + frac * (self.buf[next] - self.buf[idx]);

            let wet_out = delayed * wet;
            left[i] = dry_in_l * dry + wet_out;
            right[i] = dry_in_r * dry + wet_out;

            let write = (dry_in_l + dry_in_r) * 0.5 + delayed * fb;
            self.buf[self.pos] = write;
            self.pos = (self.pos + 1) % self.max_len;
        }
    }
}

struct AudioEngine {
    dsp: faust_gen::mydsp,
    cacher: Arc<Mutex<GestureParams>>,
    sampler: Sampler,
    mic_buffer: Arc<Mutex<VecDeque<f32>>>,
    mic_channels: usize,
    cached: GestureParams,
    delay: DelayLine,
    smooth_freq: f32,
    smooth_cutoff: f32,
    smooth_pan: f32,
    smooth_gain: f32,
    smooth_reverb: f32,
    smooth_mod_freq: f32,
    smooth_mod_amt: f32,
    smooth_detune: f32,
    smooth_rate: f32,
    smooth_grain_amt: f32,
    smooth_grain_scat: f32,
    last_sample_version: u64,
    dsp_left: Vec<f32>,
    dsp_right: Vec<f32>,
    dsp_input: Vec<f32>,
}

impl AudioEngine {
    fn new(
        params: Arc<Mutex<GestureParams>>,
        sampler_state: Arc<Mutex<SamplerState>>,
        mic_buffer: Arc<Mutex<VecDeque<f32>>>,
        mic_channels: usize,
    ) -> Self {
        let mut dsp = faust_gen::mydsp::new();
        dsp.init(44100);
        let sampler = Sampler::new(sampler_state, params.clone(), 44100.0);
        let delay = DelayLine::new(1.5, 44100.0);
        let d = GestureParams::default();
        Self {
            dsp, cacher: params, sampler, mic_buffer, mic_channels, cached: GestureParams::default(), delay,
            smooth_freq: d.freq, smooth_cutoff: d.cutoff, smooth_pan: d.pan, smooth_gain: 0.0,
            smooth_reverb: d.reverb_mix, smooth_mod_freq: d.mod_freq, smooth_mod_amt: d.mod_amt,
            smooth_detune: d.detune, smooth_rate: d.grain_rate, smooth_grain_amt: d.grain_amt,
            smooth_grain_scat: d.grain_scat, last_sample_version: 0,
            dsp_left: Vec::new(), dsp_right: Vec::new(), dsp_input: Vec::new(),
        }
    }

    fn process(&mut self, output: &mut [f32], channels: usize) {
        // Update param cache without blocking
        if let Ok(p) = self.cacher.try_lock() {
            self.cached = *p;
        }
        let p = &self.cached;

        let sr: f32 = 44100.0;
        let sk = (1.0 / (sr * 0.015)).min(1.0_f32);
        let ss = (1.0 / (sr * 0.06)).min(1.0_f32);

        self.smooth_freq += (p.freq - self.smooth_freq) * sk;
        self.smooth_cutoff += (p.cutoff - self.smooth_cutoff) * sk;
        self.smooth_pan += (p.pan - self.smooth_pan) * sk;
        self.smooth_gain += (p.gain - self.smooth_gain) * sk;
        self.smooth_reverb += (p.reverb_mix - self.smooth_reverb) * ss;
        self.smooth_mod_freq += (p.mod_freq - self.smooth_mod_freq) * ss;
        self.smooth_mod_amt += (p.mod_amt - self.smooth_mod_amt) * ss;
        self.smooth_detune += (p.detune - self.smooth_detune) * sk;
        self.smooth_rate += (p.grain_rate - self.smooth_rate) * sk;
        self.smooth_grain_amt += (p.grain_amt - self.smooth_grain_amt) * ss;
        self.smooth_grain_scat += (p.grain_scat - self.smooth_grain_scat) * sk;

        self.dsp.set_param(ParamIndex(0), self.smooth_cutoff);
        self.dsp.set_param(ParamIndex(1), self.smooth_detune);
        self.dsp.set_param(ParamIndex(2), p.filter_type);
        self.dsp.set_param(ParamIndex(3), self.smooth_freq);
        self.dsp.set_param(ParamIndex(4), self.smooth_gain);
        self.dsp.set_param(ParamIndex(5), self.smooth_grain_amt);
        self.dsp.set_param(ParamIndex(6), self.smooth_rate);
        self.dsp.set_param(ParamIndex(7), self.smooth_grain_scat);
        self.dsp.set_param(ParamIndex(8), p.grain_size);
        self.dsp.set_param(ParamIndex(9), self.smooth_mod_amt);
        self.dsp.set_param(ParamIndex(10), self.smooth_mod_freq);
        self.dsp.set_param(ParamIndex(11), p.osc_type);
        self.dsp.set_param(ParamIndex(12), self.smooth_pan);
        self.dsp.set_param(ParamIndex(13), self.smooth_reverb);
        self.dsp.korg_drive = p.korg_drive;
        self.dsp.korg_ring = p.korg_ring;

        let num_frames = output.len() / channels;

        self.dsp_left.resize(num_frames, 0.0);
        self.dsp_right.resize(num_frames, 0.0);
        self.dsp_input.resize(num_frames, 0.0);
        let outputs = &mut [self.dsp_left.as_mut_slice(), self.dsp_right.as_mut_slice()];
        let inputs: [&[f32]; 4] = [&self.dsp_input, &self.dsp_input, &self.dsp_input, &self.dsp_input];
        self.dsp.compute(num_frames as i32, &inputs, outputs);

        // Apply delay only while synth is active — prevents echo tail sustaining after gesture ends
        if self.smooth_gain > 0.002 {
            self.delay.process(&mut self.dsp_left, &mut self.dsp_right, p.head_x * 0.8 + 0.1, p.head_y * 0.55 + 0.05, 0.35, 44100.0);
        }

        // Record mic input (non-blocking) — fills the sample buffer
        if let Ok(mut st) = self.sampler.state.try_lock() {
            if st.recording && st.sample.len() < MAX_SAMPLE_LEN {
                if let Ok(mut mic) = self.mic_buffer.try_lock() {
                    let space = MAX_SAMPLE_LEN - st.sample.len();
                    let to_record = num_frames.min(space);
                    for _ in 0..to_record {
                        let mut frame_sum = 0.0f32;
                        let mut got = 0usize;
                        for _ in 0..self.mic_channels {
                            if let Some(s) = mic.pop_front() {
                                frame_sum += s;
                                got += 1;
                            } else {
                                break;
                            }
                        }
                        if got > 0 {
                            st.sample.push(frame_sum / got as f32);
                        } else {
                            break;
                        }
                    }
                }
            }
            if st.has_sample && !st.mixed_buffer.is_empty() {
                if st.sample_version != self.last_sample_version {
                    self.dsp.sample_buffer = st.mixed_buffer.clone();
                    self.last_sample_version = st.sample_version;
                }
                self.dsp.sample_amp = (p.gain * 2.5).min(1.5);
                self.dsp.sample_pitch = if p.has_hand { 0.5 + p.finger_count * 0.8 } else { 1.0 };
            } else {
                self.dsp.sample_amp = 0.0;
            }
        }

        // Copy DSP to output
        for i in 0..num_frames {
            if channels == 2 {
                output[i * 2] = self.dsp_left[i];
                output[i * 2 + 1] = self.dsp_right[i];
            } else {
                output[i] = self.dsp_left[i];
            }
        }
    }
}

fn start_audio(
    params: Arc<Mutex<GestureParams>>,
    sampler_state: Arc<Mutex<SamplerState>>,
    mic_buffer: Arc<Mutex<VecDeque<f32>>>,
    mic_channels: usize,
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

    let mut engine = AudioEngine::new(params, sampler_state, mic_buffer, mic_channels);
    engine.dsp.init(sample_rate as i32);
    engine.sampler.sample_rate = sample_rate as f32;

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

fn start_mic_input(
    buffer: Arc<Mutex<VecDeque<f32>>>,
) -> Result<(cpal::Stream, usize), Box<dyn std::error::Error>> {
    let host = cpal::default_host();
    let device = host
        .default_input_device()
        .ok_or("no microphone input device")?;
    let config = device.default_input_config()?;
    let channels = config.channels() as usize;

    println!(
        "Mic: {} Hz, {} ch, device: {}",
        config.sample_rate().0,
        channels,
        device.name().unwrap_or_default()
    );

    let max_samples = MAX_SAMPLE_LEN * channels;
    let stream = device.build_input_stream(
        &config.into(),
        move |data: &[f32], _: &cpal::InputCallbackInfo| {
            let mut buf = buffer.lock().unwrap();
            let amp = 3.0f32;
            for &sample in data {
                let s = (sample * amp).clamp(-1.0, 1.0);
                buf.push_back(s);
                if buf.len() > max_samples {
                    buf.pop_front();
                }
            }
        },
        |err| eprintln!("mic input error: {}", err),
        None,
    )?;

    stream.play()?;
    Ok((stream, channels))
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

// =========================== HUD snapshot ===========================

struct HUDData {
    stat: &'static str,
    osc: &'static str,
    variant: &'static str,
    fingers: usize,
    freq: f32,
    cutoff: f32,
    gain: f32,
    pan: f32,
    reverb_mix: f32,
    detune: f32,
    grain_amt: f32,
    delay_time: f32,
    delay_fb: f32,
}

// =========================== Main ===========================

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut camera = Camera::new(
        CameraIndex::Index(0),
        RequestedFormat::new::<RgbFormat>(RequestedFormatType::None),
    )?;
    camera.open_stream()?;

    let gesture_params = Arc::new(Mutex::new(GestureParams::default()));
    let sampler_state = Arc::new(Mutex::new(SamplerState::new()));
    let mic_buffer: Arc<Mutex<VecDeque<f32>>> = Arc::new(Mutex::new(VecDeque::new()));
    let (_mic_stream, mic_channels) = start_mic_input(mic_buffer.clone())?;
    let _audio_stream = start_audio(gesture_params.clone(), sampler_state.clone(), mic_buffer.clone(), mic_channels)?;

    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    let mut use_color =
        std::env::var("COLORTERM").map_or(false, |v| v == "truecolor" || v == "24bit");
    let frame_time = Duration::from_secs_f64(1.0 / 30.0);

    let mut motion_tracker = tracker::MotionTracker::new();
    let mut prev_hx = 0.5f32;
    let mut prev_hy = 0.5f32;
    let mut hand_hold = 0.0f32;
    let mut cam_errors = 0u32;
    let mut last_good_frame = Instant::now();
    const CAM_STALL_TIMEOUT: Duration = Duration::from_secs(10);
    const CAM_RESTART_EVERY: u32 = 30;

    let result = loop {
        let frame_start = Instant::now();

        let buffer = match camera.frame() {
            Ok(b) => {
                cam_errors = 0;
                last_good_frame = frame_start;
                b
            }
            Err(e) => {
                cam_errors += 1;
                eprintln!("camera frame error #{}: {}", cam_errors, e);

                // Transient failures (e.g. system briefly overloaded) can produce
                // bursts of errors; only give up once the camera has been silent
                // for a while, and try restarting the stream before then.
                if cam_errors % CAM_RESTART_EVERY == 0 {
                    eprintln!("camera stalled for {} errors, attempting to restart stream", cam_errors);
                    if let Err(re) = camera.open_stream() {
                        eprintln!("camera restart failed: {}", re);
                    }
                }
                if last_good_frame.elapsed() > CAM_STALL_TIMEOUT {
                    break Err(Box::new(std::io::Error::new(std::io::ErrorKind::TimedOut, "camera disconnected")) as Box<dyn std::error::Error>);
                }

                let elapsed = frame_start.elapsed();
                if elapsed < frame_time {
                    std::thread::sleep(frame_time - elapsed);
                }
                continue;
            }
        };
        let rgb = buffer.decode_image::<RgbFormat>()?;
        let res = buffer.resolution();
        let fw = res.width_x as usize;
        let fh = res.height_y as usize;

        // Multi-object tracking (motion-based, position not color)
        let step = 4usize.max(fw / 80);
        let blobs = motion_tracker.detect(&rgb, fw, fh, step);
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
                "hand"  => { hand_bbox = *bbox; has_hand = true; hand_hold = 0.5; }
                "head"  => { head_nx = *nx; head_ny = *ny; has_head = true; }
                "finger"=> { finger_count += 1; }
                "body"  => { body_nx = *nx; }
                _       => {}
            }
        }
        // Persist hand detection for 0.5s after motion stops
        if !has_hand {
            hand_hold = (hand_hold - 1.0 / 30.0).max(0.0);
            if hand_hold > 0.0 { has_hand = true; }
        }

        // Snapshot params, then drop lock immediately
        {
            let mut p = gesture_params.lock().unwrap();
            p.has_hand = has_hand;
            p.has_head = has_head;
            p.finger_count = finger_count as f32;

            if has_hand {
                let (x1, y1, x2, y2) = hand_bbox;
                let hx = (x1 + x2) as f32 / (2.0 * fw as f32);
                let hy = (y1 + y2) as f32 / (2.0 * fh as f32);
                let hs = ((x2 - x1) * (y2 - y1)) as f32 / (fw * fh) as f32;

                let dx = hx - prev_hx;
                let dy = hy - prev_hy;
                let vel = (dx * dx + dy * dy).sqrt() * 12.0;
                let v_boost = vel.min(1.0);
                prev_hx = hx;
                prev_hy = hy;

                p.hand_x = hx;
                p.hand_y = hy;
                p.hand_size = hs.min(1.0);

                // ▸ Hand X → Pan
                p.pan = hx.clamp(0.0, 1.0);

                // ▸ Hand Y → Freq (bottom=low, top=high)
                p.freq = 40.0 + (1.0 - hy) * 1960.0;

                // ▸ Hand Size → Cutoff (small=dark, big=bright)
                p.cutoff = 30.0 + hs * 9970.0;

                // ▸ Velocity → Gain burst + slight detune (silent when still)
                if vel > 0.05 {
                    p.gain = (0.15 + v_boost * 0.5).min(0.75);
                    p.detune = 1.0 + v_boost * 0.008;
                } else {
                    p.gain = 0.0;
                    p.detune = 1.001;
                }
                p.grain_scat = v_boost * 0.3;

                // ▸ Fingers → Synth variant (orchestration presets)
                let fc = finger_count.min(4);
                let (osc_val, grain, flt, det, lbl) = match fc {
                    0 => (0.0, 0.0,   0.0, 1.000, "Bass"),
                    1 => (1.0, 0.08,  0.0, 1.003, "Lead"),
                    2 => (3.0, 0.15,  1.0, 1.002, "Pad"),
                    3 => (2.0, 0.0,   0.0, 1.000, "Keys"),
                    _ => (1.0, 0.35,  2.0, 1.005, "Haze"),
                };
                p.osc_type = osc_val;
                p.grain_amt = grain;
                p.filter_type = flt;
                p.detune = det;
                p.korg_drive = 0.0;
                p.korg_ring = 0.0;
                p.variant_label = lbl;

                // ▸ Head → Reverb mix + subtle vibrato (not siren)
                if has_head {
                    p.reverb_mix = (0.5 - head_ny * 0.4).clamp(0.0, 0.5);
                    p.mod_freq = 2.0 + head_nx * 6.0;
                    p.mod_amt = 0.0;
                    p.head_x = head_nx;
                    p.head_y = head_ny;
                } else {
                    p.reverb_mix = 0.1;
                    p.mod_freq = 2.0;
                    p.mod_amt = 0.0;
                }
                // Grain rate from hand size + velocity
                p.grain_rate = 8.0 + hs * 8.0 + v_boost * 6.0;
                p.grain_size = 0.02 + hs * 0.2;

            } else {
                p.gain = 0.0;
                p.freq = 440.0;
                prev_hx = 0.5;
                prev_hy = 0.5;
            }
        }

        let Ok((cols, rows)) = crossterm::terminal::size() else {
            continue;
        };
        let cols = cols as usize;
        let rows = rows as usize;

        if cols > 0 && rows > 0 {
            let info_rows = 5usize;
            if rows > info_rows + 5 {
                let ascii_rows = rows - info_rows;
                let (mut text, mut colors) = build_ascii_frame(&rgb, fw, fh, cols, ascii_rows, use_color);
                // Draw all detected object bounding boxes (each with different color)
                let bbox_colors = [(0,255,0),(255,0,0),(0,255,255),(255,255,0),(255,128,0)];
                for (idx, &(ref label, bbox, _, _)) in objects.iter().enumerate() {
                    let col = bbox_colors[idx % bbox_colors.len()];
                    overlay_bbox_color(&mut text, &mut colors, fw, fh, bbox, cols, ascii_rows, col, label);
                }

                let (rec_status, st_has_sample) = match sampler_state.try_lock() {
                    Ok(st) => {
                        let status = if st.recording { "REC".to_string() }
                        else if st.has_sample { format!("{}L", st.layers.len()) }
                        else { "---".to_string() };
                        (status, st.has_sample)
                    }
                    Err(_) => ("??".to_string(), false),
                };

                // Snapshot params for HUD (drop lock before draw)
                let hud = {
                    let gs = gesture_params.lock().unwrap();
                    HUDData {
                        stat: if gs.has_hand { "HAND" } else if gs.has_head { "FACE" } else { "--" },
                        osc: ["Sin", "Saw", "Sq ", "Tri"][(gs.osc_type as usize).min(3)],
                        variant: gs.variant_label,
                        fingers: gs.finger_count as usize,
                        freq: gs.freq,
                        cutoff: gs.cutoff,
                        gain: gs.gain,
                        pan: gs.pan,
                        reverb_mix: gs.reverb_mix,
                        detune: gs.detune,
                        grain_amt: gs.grain_amt,
                        delay_time: gs.head_x * 0.8 + 0.1,
                        delay_fb: gs.head_y * 0.55 + 0.05,
                    }
                };

                if let Err(e) = terminal.draw(|f| {
                    let area = f.area();
                    let ascii_area = Rect::new(0, 0, area.width, area.height - info_rows as u16);
                    f.render_widget(AsciiWidget { text, colors }, ascii_area);

                    let info_area = Rect::new(0, area.height - info_rows as u16, area.width, info_rows as u16);
                    let chunks = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints([Constraint::Length(1), Constraint::Length(1), Constraint::Length(1), Constraint::Length(1), Constraint::Length(1)].as_ref())
                        .split(info_area);

                    let status_line = format!(
                        " {} {} {} F:{}  Pan:{:.2}  Rev:{:.2}  [q] [c]({})",
                        hud.stat, hud.variant, hud.osc, hud.fingers, hud.pan, hud.reverb_mix,
                        if use_color { "on" } else { "off" },
                    );
                    f.render_widget(Paragraph::new(status_line).block(Block::default().borders(Borders::TOP).title("Synth")), chunks[0]);

                    let r1 = Layout::default()
                        .direction(Direction::Horizontal)
                        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                        .split(chunks[1]);
                    let freq_norm = ((hud.freq - 40.0) / 1960.0).clamp(0.0, 1.0);
                    let cut_norm = ((hud.cutoff - 30.0) / 9970.0).clamp(0.0, 1.0);
                    f.render_widget(Gauge::default().gauge_style(Style::default().fg(Color::Cyan)).percent((freq_norm * 100.0) as u16).label(format!("Pitch {}Hz \u{2190} Y", hud.freq as u32)), r1[0]);
                    f.render_widget(Gauge::default().gauge_style(Style::default().fg(Color::Magenta)).percent((cut_norm * 100.0) as u16).label(format!("Cut {}k \u{2190} Size", (hud.cutoff / 100.0) as u32)), r1[1]);

                    let r2 = Layout::default()
                        .direction(Direction::Horizontal)
                        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                        .split(chunks[2]);
                    f.render_widget(Gauge::default().gauge_style(Style::default().fg(Color::Green)).percent((hud.gain * 100.0) as u16).label(format!("Gain {:.2} \u{2190} Vel", hud.gain)), r2[0]);
                    f.render_widget(Gauge::default().gauge_style(Style::default().fg(Color::Yellow)).percent((hud.pan * 100.0) as u16).label(format!("Pan {:.2} \u{2190} X", hud.pan)), r2[1]);

                    let r3 = Layout::default()
                        .direction(Direction::Horizontal)
                        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                        .split(chunks[3]);
                    let rev_norm = (hud.reverb_mix / 0.7).clamp(0.0, 1.0);
                    let det_norm = ((hud.detune - 0.99) / 0.06).clamp(0.0, 1.0);
                    f.render_widget(Gauge::default().gauge_style(Style::default().fg(Color::Red)).percent((rev_norm * 100.0) as u16).label(format!("Rev {:.2}", hud.reverb_mix)), r3[0]);
                    f.render_widget(Gauge::default().gauge_style(Style::default().fg(Color::Blue)).percent((det_norm * 100.0) as u16).label(format!("Dly {:.2}s Fb{:.2} \u{2190} Head", hud.delay_time, hud.delay_fb)), r3[1]);

                    let r4 = Layout::default()
                        .direction(Direction::Horizontal)
                        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                        .split(chunks[4]);
                    let rec_style = if rec_status == "REC" {
                        Style::default().fg(Color::Red)
                    } else if st_has_sample {
                        Style::default().fg(Color::Green)
                    } else {
                        Style::default().fg(Color::DarkGray)
                    };
                    f.render_widget(Paragraph::new(format!(" {}  [Space]", rec_status)).style(rec_style), r4[0]);
                    f.render_widget(Paragraph::new(format!("Grain {:.2} \u{2190} Fingers", hud.grain_amt)).style(Style::default().fg(Color::Rgb(180, 120, 255))), r4[1]);
                }) {
                    break Err(Box::new(e) as Box<dyn std::error::Error>);
                }
            }
        }

        if event::poll(Duration::from_millis(1))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break Ok(()),
                    KeyCode::Char('c') => use_color = !use_color,
                    KeyCode::Char(' ') => {
                        if let Ok(mut st) = sampler_state.try_lock() {
                            if let Ok(mut mic) = mic_buffer.try_lock() {
                                if st.recording {
                                    st.recording = false;
                                    if !st.sample.is_empty() {
                                        let new_buf = st.sample.clone();
                                        st.layers.push(SampleLayer { buffer: new_buf });
                                    }
                                    st.sample.clear();
                                    mic.clear();
                                    st.remix();
                                } else {
                                    st.layers.clear();
                                    st.sample.clear();
                                    mic.clear();
                                    st.recording = true;
                                    st.has_sample = false;
                                }
                            }
                        }
                    }
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
