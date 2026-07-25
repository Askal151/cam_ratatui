# cam_Granular noise modulator

Turn your webcam into an ASCII-art terminal visualizer that doubles as a gesture-controlled synthesizer. Motion tracking on your hand/head/fingers maps live to a Faust-generated synth engine, with a built-in looper for layering recorded mic samples on top.

## Features

- **Webcam → ASCII art**, rendered live in the terminal (`ratatui` + `crossterm`), with optional 24-bit color and per-object bounding-box overlays.
- **Motion-based multi-object tracking** (frame-difference + connected components — no ML model, no color calibration) classifying blobs into `hand` / `head` / `finger` / `body`.
- **Gesture → sound mapping**:
  - Hand X → stereo pan
  - Hand Y → pitch (bottom = low, top = high)
  - Hand size → filter cutoff (small = dark, big = bright)
  - Hand velocity → gain burst + slight detune (silent when the hand is still)
  - Finger count → synth "variant" preset (Bass / Lead / Pad / Keys / Haze)
  - Head position → reverb mix + delay time/feedback
- **Looper/sampler**: record the mic into layered loops that get mixed into the synth output.
- **Live HUD** with gauges for pitch, cutoff, gain, pan, reverb, delay time/feedback, and grain amount.

## Controls

| Key     | Action                                                                 |
|---------|-------------------------------------------------------------------------|
| `q`     | Quit                                                                     |
| `c`     | Toggle 24-bit color rendering                                            |
| `Space` | Start/stop recording a sample layer (stopping commits the layer; starting again with none recording clears previous layers) |

## Requirements

- A webcam (opens index `0` — i.e. `/dev/video0` — by default)
- An audio output device, and a microphone if you want to use the looper
- System libraries, needed to build:
  - `libasound2-dev` (ALSA headers, for audio via `cpal`)
  - `libclang-dev` (for `bindgen`, used by the V4L2 camera bindings)

  ```sh
  sudo apt install libasound2-dev libclang-dev pkg-config
  ```

## Build & run

```sh
cargo build --release
./run.sh
```

or directly:

```sh
cargo run --release
```

`run.sh` also exports `LIBCLANG_PATH` / `LD_LIBRARY_PATH` / `PKG_CONFIG_PATH` pointing at `/tmp/alsa_dev` and `/tmp/libclang_extract`. Those are leftovers from an earlier setup where the ALSA/libclang dev files were manually extracted into `/tmp` instead of installed via `apt` — since `/tmp` is wiped on reboot, that workaround needs to be redone after every reboot. If you install the packages above normally (as shown), you generally don't need those paths.

## Project layout

- `src/main.rs` — camera capture loop, gesture-to-parameter mapping, ASCII rendering, audio engine glue
- `src/tracker.rs` — frame-difference motion tracker + connected-component blob detection
- `src/faust_gen.rs` — the synth DSP engine (Rust code generated from a Faust `.dsp` source)
- `dsp/gest_synth.dsp` — Faust source sketch for the synth

**Note:** `dsp/gest_synth.dsp` is currently a simplified sketch and is *not* kept in sync with the DSP graph actually running in `src/faust_gen.rs` (which is considerably more elaborate — multi-tap reverb, allpass diffusion, etc.). Regenerating `faust_gen.rs` from the checked-in `.dsp` file will produce a different-sounding engine.
