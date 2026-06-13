import("stdfaust.lib");

freq        = hslider("freq", 440, 40, 2000, 0.01);
cutoff      = hslider("cutoff", 2000, 30, 10000, 0.01);
gain        = hslider("gain", 0.3, 0, 1, 0.01);
mod_amt     = hslider("mod_amt", 0.05, 0, 0.6, 0.01);
mod_freq    = hslider("mod_freq", 3, 0.1, 30, 0.01);
osc_type    = hslider("osc_type", 0, 0, 3, 1);
pan         = hslider("pan", 0.5, 0, 1, 0.01);
detune      = hslider("detune", 1.001, 0.99, 1.05, 0.001);
filter_type = hslider("filter_type", 0, 0, 2, 1);
reverb_mix  = hslider("reverb_mix", 0.15, 0, 0.8, 0.01);

grain_rate  = hslider("grain_rate", 4, 0.5, 20, 0.1);
grain_size  = hslider("grain_size", 0.12, 0.01, 0.5, 0.01);
grain_amt   = hslider("grain_amt", 0.3, 0, 1, 0.01);
grain_scat  = hslider("grain_scat", 0.2, 0, 1, 0.01);

saw_osc=os.sawtooth;
tri_osc=os.triangle;
sqr_osc=os.square;

select_osc(f, t) = select2(t > 0,
    select2(t > 1,
        select2(t > 2, tri_osc(f), sqr_osc(f)),
        saw_osc(f)),
    os.osc(f));

carrier = select_osc(freq + os.osc(mod_freq) * mod_amt * freq * 0.5, osc_type);
sub     = select_osc(freq * 0.5, osc_type) * 0.25;
detuned = select_osc(freq * detune, osc_type) * 0.3;
mix = carrier + sub + detuned + no.noise * 0.04;

lpf = fi.lowpass(3, cutoff, mix);
hpf = fi.highpass(2, cutoff, mix);
bpf = fi.highpass(2, cutoff, fi.lowpass(2, cutoff * 1.4, mix));
ft = filter_type;
slpf = 1.0 - (ft > 0.0) - (ft < 0.0);
sbpf = (ft > 0.0) - (ft > 1.0);
shpf = (ft > 1.0) - (ft > 2.0);
filtered = lpf * slpf + bpf * sbpf + hpf * shpf;

rev = re.mono_freeverb(0.5, 0.3, 0.3, 23);
wet = reverb_mix * (filtered : rev);
dry = (1.0 - reverb_mix) * filtered;
out = (dry + wet) * gain;

// Granular section
grain_phase = os.phasor(grain_rate);
grain_size_norm = grain_size * grain_rate;
grain_size_clamped = min(grain_size_norm, 1.0);
grain_active = grain_phase < grain_size_clamped;
grain_pos = grain_phase / max(grain_size_clamped, 0.001);
grain_env = select2(grain_active, pow(sin(grain_pos * 3.14159), 2), 0.0);
amp_jitter = 1.0 + grain_scat * no.noise * 0.6;
pan_jitter = (no.noise + 1.0) * 0.5;
grain_wet = out * grain_env * amp_jitter * grain_amt;
grain_L = grain_wet * pan_jitter;
grain_R = grain_wet * (1.0 - pan_jitter);

process = (out * pan + grain_L), (out * (1.0 - pan) + grain_R);
