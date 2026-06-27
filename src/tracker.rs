pub struct Blob {
    pub label: u32,
    pub pixels: usize,
    pub cx: f32,
    pub cy: f32,
    pub min_x: usize,
    pub max_x: usize,
    pub min_y: usize,
    pub max_y: usize,
}

pub struct MotionTracker {
    prev_gray: Vec<u8>,
    initialized: bool,
}

impl MotionTracker {
    pub fn new() -> Self {
        Self { prev_gray: Vec::new(), initialized: false }
    }

    pub fn detect(&mut self, rgb: &[u8], w: usize, h: usize, step: usize) -> Vec<Blob> {
        let size = w * h;
        if self.prev_gray.len() != size {
            self.prev_gray = vec![0u8; size];
            self.initialized = false;
        }

        let mut gray = vec![0u8; size];
        for i in 0..size {
            let j = i * 3;
            let r = rgb[j] as u32;
            let g = rgb[j + 1] as u32;
            let b = rgb[j + 2] as u32;
            gray[i] = ((r * 77 + g * 150 + b * 29) >> 8) as u8;
        }

        if !self.initialized {
            self.prev_gray.copy_from_slice(&gray);
            self.initialized = true;
            return Vec::new();
        }

        let step = step.max(2);
        let sw = w / step;
        let sh = h / step;
        if sw < 2 || sh < 2 { return vec![]; }

        let mut mask = vec![false; sw * sh];
        let threshold = 25u8;

        for y in 0..sh {
            for x in 0..sw {
                let px = x * step;
                let py = y * step;
                let idx = py * w + px;
                let diff = (gray[idx] as i16 - self.prev_gray[idx] as i16).abs();
                mask[y * sw + x] = diff > threshold as i16;
            }
        }

        self.prev_gray.copy_from_slice(&gray);

        // Dilate mask to merge nearby motion regions
        let mut dilated = vec![false; sw * sh];
        for y in 0..sh {
            for x in 0..sw {
                if mask[y * sw + x] {
                    for dy in -1..=1 {
                        for dx in -1..=1 {
                            let ny = y as i32 + dy;
                            let nx = x as i32 + dx;
                            if nx >= 0 && nx < sw as i32 && ny >= 0 && ny < sh as i32 {
                                dilated[ny as usize * sw + nx as usize] = true;
                            }
                        }
                    }
                }
            }
        }

        let total_size = (sw * sh) as f32;

        // Connected component labeling (same algorithm as before)
        let mut labels = vec![0u32; sw * sh];
        let mut next_label = 1u32;
        let mut eq: Vec<(u32, u32)> = Vec::new();

        for y in 0..sh {
            for x in 0..sw {
                if !dilated[y * sw + x] { continue; }
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

        let mut parent: Vec<u32> = (0..total).collect();
        fn resolve(parent: &mut [u32], x: u32) -> u32 {
            let xi = x as usize;
            if parent[xi] != x {
                parent[xi] = resolve(parent, parent[xi]);
            }
            parent[xi]
        }
        for &(a, b) in &eq {
            let ra = resolve(&mut parent, a);
            let rb = resolve(&mut parent, b);
            if ra != rb { parent[rb as usize] = ra; }
        }
        for i in 0..total as usize { let pi = parent[i]; parent[i] = resolve(&mut parent, pi); }

        let mut label_map: Vec<u32> = vec![0; total as usize];
        let mut unique_count = 0u32;
        for i in 1..total as usize {
            let p = parent[i] as usize;
            if label_map[p] == 0 { unique_count += 1; label_map[p] = unique_count; }
        }

        let mut blobs: Vec<Blob> = (0..unique_count).map(|_| Blob {
            label: 0, pixels: 0, cx: 0.0, cy: 0.0,
            min_x: sw, max_x: 0, min_y: sh, max_y: 0,
        }).collect();

        for y in 0..sh {
            for x in 0..sw {
                if !dilated[y * sw + x] { continue; }
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

        for b in &mut blobs {
            b.cx = b.cx / b.pixels as f32 * step as f32;
            b.cy = b.cy / b.pixels as f32 * step as f32;
            b.min_x *= step;
            b.max_x = (b.max_x + 1) * step;
            b.min_y *= step;
            b.max_y = (b.max_y + 1) * step;
        }

        blobs.retain(|b| b.pixels >= 3);
        blobs.sort_by(|a, b| b.pixels.cmp(&a.pixels));
        blobs
    }
}
