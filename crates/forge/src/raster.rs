//! Small raster toolkit: grayscale images, masks, connected components,
//! distance transform and thinning.

use std::path::Path;

/// Ink amount per pixel, 0 (paper) to 1 (ink).
#[derive(Clone)]
pub struct Gray {
    pub w: usize,
    pub h: usize,
    pub px: Vec<f32>,
}

#[derive(Clone)]
pub struct Mask {
    pub w: usize,
    pub h: usize,
    pub px: Vec<bool>,
}

#[derive(Clone, Debug)]
pub struct Component {
    pub pixels: Vec<(usize, usize)>,
    pub x0: usize,
    pub y0: usize,
    pub x1: usize,
    pub y1: usize,
}

impl Component {
    pub fn width(&self) -> usize {
        self.x1 - self.x0 + 1
    }
    pub fn height(&self) -> usize {
        self.y1 - self.y0 + 1
    }
    pub fn centroid(&self) -> (f32, f32) {
        let n = self.pixels.len() as f32;
        let (sx, sy) = self.pixels.iter().fold((0.0, 0.0), |(a, b), &(x, y)| (a + x as f32, b + y as f32));
        (sx / n, sy / n)
    }
}

impl Gray {
    pub fn load(path: &Path) -> Gray {
        let img = image::open(path).unwrap_or_else(|e| panic!("{}: {e}", path.display())).to_luma8();
        let (w, h) = (img.width() as usize, img.height() as usize);
        let px = img.pixels().map(|p| 1.0 - p.0[0] as f32 / 255.0).collect();
        Gray { w, h, px }
    }

    pub fn get(&self, x: isize, y: isize) -> f32 {
        if x < 0 || y < 0 || x >= self.w as isize || y >= self.h as isize {
            0.0
        } else {
            self.px[y as usize * self.w + x as usize]
        }
    }

    pub fn crop(&self, x0: usize, y0: usize, x1: usize, y1: usize) -> Gray {
        let (w, h) = (x1 - x0, y1 - y0);
        let mut px = vec![0.0; w * h];
        for y in 0..h {
            for x in 0..w {
                px[y * w + x] = self.get((x0 + x) as isize, (y0 + y) as isize);
            }
        }
        Gray { w, h, px }
    }

    pub fn threshold(&self, t: f32) -> Mask {
        Mask { w: self.w, h: self.h, px: self.px.iter().map(|&v| v >= t).collect() }
    }

    /// Bilinear upscaling, sampling pixel centres.
    pub fn upscale(&self, f: usize) -> Gray {
        let (w, h) = (self.w * f, self.h * f);
        let mut px = vec![0.0; w * h];
        for y in 0..h {
            let sy = (y as f32 + 0.5) / f as f32 - 0.5;
            let y0 = sy.floor();
            let ty = sy - y0;
            for x in 0..w {
                let sx = (x as f32 + 0.5) / f as f32 - 0.5;
                let x0 = sx.floor();
                let tx = sx - x0;
                let (xi, yi) = (x0 as isize, y0 as isize);
                let a = self.get(xi, yi) * (1.0 - tx) + self.get(xi + 1, yi) * tx;
                let b = self.get(xi, yi + 1) * (1.0 - tx) + self.get(xi + 1, yi + 1) * tx;
                px[y * w + x] = a * (1.0 - ty) + b * ty;
            }
        }
        Gray { w, h, px }
    }

    /// Box blur with the given radius (used to soften 1-bit scans).
    pub fn blur(&self, r: isize) -> Gray {
        let mut px = vec![0.0; self.w * self.h];
        let n = ((2 * r + 1) * (2 * r + 1)) as f32;
        for y in 0..self.h as isize {
            for x in 0..self.w as isize {
                let mut s = 0.0;
                for dy in -r..=r {
                    for dx in -r..=r {
                        s += self.get(x + dx, y + dy);
                    }
                }
                px[y as usize * self.w + x as usize] = s / n;
            }
        }
        Gray { w: self.w, h: self.h, px }
    }

    /// Adds `m` pixels of empty margin on every side.
    pub fn pad(&self, m: usize) -> Gray {
        let (w, h) = (self.w + 2 * m, self.h + 2 * m);
        let mut px = vec![0.0; w * h];
        for y in 0..self.h {
            for x in 0..self.w {
                px[(y + m) * w + x + m] = self.px[y * self.w + x];
            }
        }
        Gray { w, h, px }
    }
}

impl Mask {
    pub fn new(w: usize, h: usize) -> Mask {
        Mask { w, h, px: vec![false; w * h] }
    }

    pub fn at(&self, x: isize, y: isize) -> bool {
        x >= 0 && y >= 0 && x < self.w as isize && y < self.h as isize && self.px[y as usize * self.w + x as usize]
    }

    pub fn set(&mut self, x: usize, y: usize, v: bool) {
        self.px[y * self.w + x] = v;
    }

    /// 8-connected components.
    pub fn components(&self) -> Vec<Component> {
        let mut seen = vec![false; self.w * self.h];
        let mut out = Vec::new();
        for start in 0..self.px.len() {
            if !self.px[start] || seen[start] {
                continue;
            }
            seen[start] = true;
            let mut stack = vec![start];
            let mut c = Component { pixels: Vec::new(), x0: usize::MAX, y0: usize::MAX, x1: 0, y1: 0 };
            while let Some(i) = stack.pop() {
                let (x, y) = (i % self.w, i / self.w);
                c.pixels.push((x, y));
                c.x0 = c.x0.min(x);
                c.y0 = c.y0.min(y);
                c.x1 = c.x1.max(x);
                c.y1 = c.y1.max(y);
                for dy in -1isize..=1 {
                    for dx in -1isize..=1 {
                        let (nx, ny) = (x as isize + dx, y as isize + dy);
                        if self.at(nx, ny) {
                            let j = ny as usize * self.w + nx as usize;
                            if !seen[j] {
                                seen[j] = true;
                                stack.push(j);
                            }
                        }
                    }
                }
            }
            out.push(c);
        }
        out
    }

    /// Distance from each ink pixel to the nearest paper pixel (two-pass
    /// chamfer 3-4, divided by 3 so that it is in pixels).
    pub fn distance(&self) -> Vec<f32> {
        let (w, h) = (self.w, self.h);
        let big = 1e9f32;
        let mut d: Vec<f32> = self.px.iter().map(|&b| if b { big } else { 0.0 }).collect();
        let idx = |x: usize, y: usize| y * w + x;
        for y in 0..h {
            for x in 0..w {
                if d[idx(x, y)] == 0.0 {
                    continue;
                }
                let mut v = d[idx(x, y)];
                if x > 0 { v = v.min(d[idx(x - 1, y)] + 3.0) } else { v = v.min(3.0) }
                if y > 0 { v = v.min(d[idx(x, y - 1)] + 3.0) } else { v = v.min(3.0) }
                if x > 0 && y > 0 { v = v.min(d[idx(x - 1, y - 1)] + 4.0) }
                if x + 1 < w && y > 0 { v = v.min(d[idx(x + 1, y - 1)] + 4.0) }
                d[idx(x, y)] = v;
            }
        }
        for y in (0..h).rev() {
            for x in (0..w).rev() {
                if d[idx(x, y)] == 0.0 {
                    continue;
                }
                let mut v = d[idx(x, y)];
                if x + 1 < w { v = v.min(d[idx(x + 1, y)] + 3.0) } else { v = v.min(3.0) }
                if y + 1 < h { v = v.min(d[idx(x, y + 1)] + 3.0) } else { v = v.min(3.0) }
                if x + 1 < w && y + 1 < h { v = v.min(d[idx(x + 1, y + 1)] + 4.0) }
                if x > 0 && y + 1 < h { v = v.min(d[idx(x - 1, y + 1)] + 4.0) }
                d[idx(x, y)] = v;
            }
        }
        d.iter().map(|v| v / 3.0).collect()
    }

    /// Zhang–Suen thinning to a one-pixel skeleton.
    pub fn thin(&self) -> Mask {
        let mut m = self.clone();
        let (w, h) = (m.w as isize, m.h as isize);
        loop {
            let mut changed = false;
            for step in 0..2 {
                let mut del = Vec::new();
                for y in 0..h {
                    for x in 0..w {
                        if !m.at(x, y) {
                            continue;
                        }
                        let p = [
                            m.at(x, y - 1),
                            m.at(x + 1, y - 1),
                            m.at(x + 1, y),
                            m.at(x + 1, y + 1),
                            m.at(x, y + 1),
                            m.at(x - 1, y + 1),
                            m.at(x - 1, y),
                            m.at(x - 1, y - 1),
                        ];
                        let b = p.iter().filter(|&&v| v).count();
                        if !(2..=6).contains(&b) {
                            continue;
                        }
                        let a = (0..8).filter(|&i| !p[i] && p[(i + 1) % 8]).count();
                        if a != 1 {
                            continue;
                        }
                        let (p2, p4, p6, p8) = (p[0], p[2], p[4], p[6]);
                        let ok = if step == 0 {
                            !(p2 && p4 && p6) && !(p4 && p6 && p8)
                        } else {
                            !(p2 && p4 && p8) && !(p2 && p6 && p8)
                        };
                        if ok {
                            del.push((x as usize, y as usize));
                        }
                    }
                }
                if !del.is_empty() {
                    changed = true;
                }
                for (x, y) in del {
                    m.set(x, y, false);
                }
            }
            if !changed {
                break;
            }
        }
        m
    }
}
