//! Turns a glyph mask into vector parts the styles can redraw:
//!
//! * **strokes**: centre lines of the drawn lines (from the skeleton);
//! * **blobs**: outlines of solid black areas, which a centre line would lose;
//! * **dots** and **rings**: small filled dots and small circles, redrawn as
//!   exact circles.
//!
//! Coordinates are in pixels of the (upscaled) mask, y pointing down.

use crate::raster::Mask;

pub type P = (f32, f32);

#[derive(Clone, Debug)]
pub struct Line {
    pub pts: Vec<P>,
    pub closed: bool,
}

#[derive(Clone, Debug, Default)]
pub struct Shape {
    pub strokes: Vec<Line>,
    /// Closed outlines; outer outlines and holes have opposite orientation.
    pub blobs: Vec<Vec<P>>,
    pub dots: Vec<(P, f32)>,
    pub rings: Vec<(P, f32)>,
    /// Typical half width of the drawn lines.
    pub half: f32,
}

impl Shape {
    pub fn map(&self, f: impl Fn(P) -> P, scale: f32) -> Shape {
        Shape {
            strokes: self.strokes.iter().map(|l| Line { pts: l.pts.iter().map(|&p| f(p)).collect(), closed: l.closed }).collect(),
            blobs: self.blobs.iter().map(|b| b.iter().map(|&p| f(p)).collect()).collect(),
            dots: self.dots.iter().map(|&(p, r)| (f(p), r * scale)).collect(),
            rings: self.rings.iter().map(|&(p, r)| (f(p), r * scale)).collect(),
            half: self.half * scale,
        }
    }

    pub fn bounds(&self) -> (f32, f32, f32, f32) {
        let mut b = (f32::MAX, f32::MAX, f32::MIN, f32::MIN);
        let mut add = |(x, y): P, r: f32| {
            b.0 = b.0.min(x - r);
            b.1 = b.1.min(y - r);
            b.2 = b.2.max(x + r);
            b.3 = b.3.max(y + r);
        };
        for l in &self.strokes {
            for &p in &l.pts {
                add(p, 0.0);
            }
        }
        for bl in &self.blobs {
            for &p in bl {
                add(p, 0.0);
            }
        }
        for &(p, r) in self.dots.iter().chain(&self.rings) {
            add(p, r);
        }
        b
    }
}

const N8: [(isize, isize); 8] = [(0, -1), (1, -1), (1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0), (-1, -1)];

/// Number of separate branches leaving a skeleton pixel.
fn branches(s: &Mask, x: isize, y: isize) -> usize {
    let p: Vec<bool> = N8.iter().map(|&(dx, dy)| s.at(x + dx, y + dy)).collect();
    (0..8).filter(|&i| !p[i] && p[(i + 1) % 8]).count().max(if p.iter().all(|&v| v) { 3 } else { 0 })
}

pub fn vectorize(mask: &Mask) -> Shape {
    let w = mask.w;
    let dist = mask.distance();
    let skel = mask.thin();

    // Typical half width: median distance on the skeleton.
    let mut ds: Vec<f32> = (0..skel.px.len()).filter(|&i| skel.px[i]).map(|i| dist[i]).collect();
    ds.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let half = ds.get(ds.len() / 2).copied().unwrap_or(1.0).max(1.0);

    let mut shape = Shape { half, ..Default::default() };
    let mut ignore = Mask::new(mask.w, mask.h);

    // Small filled components are dots.
    for c in mask.components() {
        let (cw, ch) = (c.width() as f32, c.height() as f32);
        let fill = c.pixels.len() as f32 / (cw * ch);
        if cw.max(ch) <= 5.5 * half && fill > 0.5 && cw.min(ch) >= 0.55 * cw.max(ch) {
            let (cx, cy) = c.centroid();
            shape.dots.push(((cx + 0.5, cy + 0.5), (cw + ch) / 4.0));
            for &(x, y) in &c.pixels {
                ignore.set(x, y, true);
            }
        }
    }

    // Solid areas: pixels far from the paper, grown back to their edges.
    let core_t = 2.3 * half;
    let mut region = Mask::new(mask.w, mask.h);
    for y in 0..mask.h {
        for x in 0..mask.w {
            let d = dist[y * w + x];
            if d > core_t && !ignore.at(x as isize, y as isize) {
                let r = d + 0.5;
                let ri = r.ceil() as isize;
                for dy in -ri..=ri {
                    for dx in -ri..=ri {
                        if ((dx * dx + dy * dy) as f32) <= r * r && mask.at(x as isize + dx, y as isize + dy) {
                            region.set((x as isize + dx) as usize, (y as isize + dy) as usize, true);
                        }
                    }
                }
            }
        }
    }
    // Ignore specks of "solid" area: a blob must be clearly thicker than a line.
    for c in region.clone().components() {
        if (c.pixels.len() as f32) < 14.0 * half * half {
            for &(x, y) in &c.pixels {
                region.set(x, y, false);
            }
        }
    }
    shape.blobs = outline(&region);

    // Skeleton outside dots and solid areas.
    let mut s = skel.clone();
    for i in 0..s.px.len() {
        if ignore.px[i] || region.px[i] {
            s.px[i] = false;
        }
    }
    let lines = trace(&s, half);
    let eps = (0.55 * half).max(0.8);
    for l in lines {
        let len = length(&l.pts, l.closed);
        if len < 1.2 * half {
            // A tiny mark: draw it as a dot.
            let n = l.pts.len() as f32;
            let c = l.pts.iter().fold((0.0, 0.0), |a, p| (a.0 + p.0 / n, a.1 + p.1 / n));
            shape.dots.push(((c.0 + 0.5, c.1 + 0.5), half * 1.1));
            continue;
        }
        if l.closed {
            if let Some(ring) = as_ring(&l.pts, half) {
                shape.rings.push(ring);
                continue;
            }
        }
        let mut pts: Vec<P> = simplify(&l.pts, eps, l.closed).into_iter().map(|(x, y)| (x + 0.5, y + 0.5)).collect();
        straighten(&mut pts, l.closed);
        shape.strokes.push(Line { pts, closed: l.closed });
    }
    align(&mut shape, 0.8 * half);
    shape
}

fn length(pts: &[P], closed: bool) -> f32 {
    let mut s: f32 = pts.windows(2).map(|w| dist(w[0], w[1])).sum();
    if closed && pts.len() > 2 {
        s += dist(pts[0], pts[pts.len() - 1]);
    }
    s
}

pub fn dist(a: P, b: P) -> f32 {
    ((a.0 - b.0).powi(2) + (a.1 - b.1).powi(2)).sqrt()
}

/// A closed line that is nearly a circle.
fn as_ring(pts: &[P], half: f32) -> Option<(P, f32)> {
    let n = pts.len() as f32;
    if n < 8.0 {
        return None;
    }
    let c = pts.iter().fold((0.0, 0.0), |a, p| (a.0 + p.0 / n, a.1 + p.1 / n));
    let rs: Vec<f32> = pts.iter().map(|&p| dist(p, c)).collect();
    let mean = rs.iter().sum::<f32>() / n;
    let sd = (rs.iter().map(|r| (r - mean).powi(2)).sum::<f32>() / n).sqrt();
    (sd / mean < 0.13 && mean < 9.0 * half).then_some(((c.0 + 0.5, c.1 + 0.5), mean))
}

/// Follows the skeleton into polylines. Each junction (a clump of branching
/// pixels) becomes one point; lines that merely pass through a two-way joint
/// are joined, and short spurs left by thinning are dropped.
fn trace(s: &Mask, half: f32) -> Vec<Line> {
    let (w, h) = (s.w as isize, s.h as isize);
    let idx = |x: isize, y: isize| (y * w + x) as usize;
    let mut node = vec![false; s.px.len()];
    for y in 0..h {
        for x in 0..w {
            if s.at(x, y) {
                node[idx(x, y)] = branches(s, x, y) != 2;
            }
        }
    }
    // Clusters of touching node pixels.
    let mut cluster = vec![usize::MAX; s.px.len()];
    let mut centres: Vec<P> = Vec::new();
    for y in 0..h {
        for x in 0..w {
            if !node[idx(x, y)] || cluster[idx(x, y)] != usize::MAX {
                continue;
            }
            let id = centres.len();
            let mut stack = vec![(x, y)];
            cluster[idx(x, y)] = id;
            let (mut sx, mut sy, mut n) = (0.0, 0.0, 0.0);
            while let Some((cx, cy)) = stack.pop() {
                sx += cx as f32;
                sy += cy as f32;
                n += 1.0;
                for (dx, dy) in N8 {
                    let (nx, ny) = (cx + dx, cy + dy);
                    if s.at(nx, ny) && node[idx(nx, ny)] && cluster[idx(nx, ny)] == usize::MAX {
                        cluster[idx(nx, ny)] = id;
                        stack.push((nx, ny));
                    }
                }
            }
            centres.push((sx / n, sy / n));
        }
    }

    let neighbours = |x: isize, y: isize| -> Vec<(isize, isize)> {
        [0, 2, 4, 6, 1, 3, 5, 7].iter().map(|&i| (x + N8[i].0, y + N8[i].1)).filter(|&(nx, ny)| s.at(nx, ny)).collect()
    };

    // (points, cluster at start, cluster at end, closed)
    let mut lines: Vec<(Vec<P>, Option<usize>, Option<usize>, bool)> = Vec::new();
    let mut visited = vec![false; s.px.len()];
    let walk = |start_cluster: Option<usize>, first: (isize, isize), prev0: (isize, isize), visited: &mut Vec<bool>| {
        let mut pts = vec![(first.0 as f32, first.1 as f32)];
        visited[idx(first.0, first.1)] = true;
        let (mut prev, mut cur) = (prev0, first);
        let mut end = None;
        loop {
            let ns = neighbours(cur.0, cur.1);
            let node_next = ns.iter().copied().find(|&p| {
                p != prev && node[idx(p.0, p.1)] && (Some(cluster[idx(p.0, p.1)]) != start_cluster || pts.len() > 2)
            });
            if let Some(p) = node_next {
                end = Some(cluster[idx(p.0, p.1)]);
                break;
            }
            match ns.iter().copied().find(|&p| p != prev && !node[idx(p.0, p.1)] && !visited[idx(p.0, p.1)]) {
                Some(p) => {
                    visited[idx(p.0, p.1)] = true;
                    pts.push((p.0 as f32, p.1 as f32));
                    prev = cur;
                    cur = p;
                }
                None => break,
            }
        }
        (pts, end)
    };

    for y in 0..h {
        for x in 0..w {
            if !s.at(x, y) || !node[idx(x, y)] {
                continue;
            }
            let c = cluster[idx(x, y)];
            for (nx, ny) in neighbours(x, y) {
                if node[idx(nx, ny)] || visited[idx(nx, ny)] {
                    continue;
                }
                let (mut pts, end) = walk(Some(c), (nx, ny), (x, y), &mut visited);
                pts.insert(0, centres[c]);
                if let Some(e) = end {
                    pts.push(centres[e]);
                }
                lines.push((pts, Some(c), end, false));
            }
        }
    }
    // Loops with no junction at all.
    for y in 0..h {
        for x in 0..w {
            if s.at(x, y) && !node[idx(x, y)] && !visited[idx(x, y)] {
                let ns = neighbours(x, y);
                let Some(&prev) = ns.first() else { continue };
                let (pts, _) = walk(None, (x, y), prev, &mut visited);
                let closed = pts.len() > 3;
                lines.push((pts, None, None, closed));
            }
        }
    }
    // Clusters with no line at all (isolated pixels or clumps).
    let mut used = vec![false; centres.len()];
    for l in &lines {
        for c in [l.1, l.2].into_iter().flatten() {
            used[c] = true;
        }
    }
    for (c, &u) in used.iter().enumerate() {
        if !u {
            lines.push((vec![centres[c]], None, None, false));
        }
    }

    let degrees = |lines: &[(Vec<P>, Option<usize>, Option<usize>, bool)]| {
        let mut d = vec![0usize; centres.len()];
        for l in lines {
            for c in [l.1, l.2].into_iter().flatten() {
                d[c] += 1;
            }
        }
        d
    };
    // Drop short spurs: a free end on one side, a real junction on the other.
    let d = degrees(&lines);
    lines.retain(|l| {
        let free = |c: Option<usize>| c.map(|c| d[c] == 1).unwrap_or(true);
        let junction = |c: Option<usize>| c.map(|c| d[c] >= 3).unwrap_or(false);
        let spur = (free(l.1) && junction(l.2)) || (free(l.2) && junction(l.1));
        !(spur && length(&l.0, false) < 2.4 * half)
    });

    // Join lines through two-way joints.
    loop {
        let d = degrees(&lines);
        let Some(c) = (0..centres.len()).find(|&c| {
            d[c] == 2 && lines.iter().filter(|l| !l.3 && (l.1 == Some(c) || l.2 == Some(c))).count() >= 1
        }) else {
            break;
        };
        let ids: Vec<usize> = (0..lines.len()).filter(|&i| !lines[i].3 && (lines[i].1 == Some(c) || lines[i].2 == Some(c))).collect();
        if ids.len() == 1 {
            // Both ends of one line meet here: it is a loop.
            let l = &mut lines[ids[0]];
            if l.1 == Some(c) && l.2 == Some(c) {
                l.0.pop();
                l.1 = None;
                l.2 = None;
                l.3 = true;
                continue;
            }
            break;
        }
        let (i, j) = (ids[0], ids[1]);
        let mut b = lines.remove(j);
        let a = &mut lines[if j < i { i - 1 } else { i }];
        // Orient a to end at c and b to start at c.
        if a.2 != Some(c) {
            a.0.reverse();
            std::mem::swap(&mut a.1, &mut a.2);
        }
        if b.1 != Some(c) {
            b.0.reverse();
            std::mem::swap(&mut b.1, &mut b.2);
        }
        a.0.extend(b.0.into_iter().skip(1));
        a.2 = b.2;
    }

    lines.into_iter().map(|(pts, _, _, closed)| Line { pts, closed }).collect()
}

/// Ramer–Douglas–Peucker simplification.
pub fn simplify(pts: &[P], eps: f32, closed: bool) -> Vec<P> {
    if pts.len() < 3 {
        return pts.to_vec();
    }
    if closed {
        // Split at the point farthest from the first one.
        let far = (1..pts.len()).max_by(|&a, &b| dist(pts[0], pts[a]).partial_cmp(&dist(pts[0], pts[b])).unwrap()).unwrap();
        let mut a = simplify(&pts[..=far], eps, false);
        let mut ring: Vec<P> = pts[far..].to_vec();
        ring.push(pts[0]);
        let b = simplify(&ring, eps, false);
        a.pop();
        a.extend(b);
        a.pop();
        return a;
    }
    let (a, b) = (pts[0], pts[pts.len() - 1]);
    let mut best = (0, 0.0f32);
    for (i, &p) in pts.iter().enumerate().take(pts.len() - 1).skip(1) {
        let d = seg_dist(p, a, b);
        if d > best.1 {
            best = (i, d);
        }
    }
    if best.1 > eps {
        let mut left = simplify(&pts[..=best.0], eps, false);
        let right = simplify(&pts[best.0..], eps, false);
        left.pop();
        left.extend(right);
        left
    } else {
        vec![a, b]
    }
}

fn seg_dist(p: P, a: P, b: P) -> f32 {
    let (dx, dy) = (b.0 - a.0, b.1 - a.1);
    let l2 = dx * dx + dy * dy;
    if l2 == 0.0 {
        return dist(p, a);
    }
    let t = (((p.0 - a.0) * dx + (p.1 - a.1) * dy) / l2).clamp(0.0, 1.0);
    dist(p, (a.0 + t * dx, a.1 + t * dy))
}

/// Outlines of a mask along pixel edges, simplified. Outer outlines and holes
/// come out with opposite orientation.
fn outline(m: &Mask) -> Vec<Vec<P>> {
    use std::collections::HashMap;
    let mut next: HashMap<(i32, i32), Vec<(i32, i32)>> = HashMap::new();
    for y in 0..m.h as i32 {
        for x in 0..m.w as i32 {
            if !m.at(x as isize, y as isize) {
                continue;
            }
            let empty = |dx: i32, dy: i32| !m.at((x + dx) as isize, (y + dy) as isize);
            let mut add = |a: (i32, i32), b: (i32, i32)| next.entry(a).or_default().push(b);
            if empty(0, -1) {
                add((x, y), (x + 1, y));
            }
            if empty(1, 0) {
                add((x + 1, y), (x + 1, y + 1));
            }
            if empty(0, 1) {
                add((x + 1, y + 1), (x, y + 1));
            }
            if empty(-1, 0) {
                add((x, y + 1), (x, y));
            }
        }
    }
    let mut loops = Vec::new();
    loop {
        let Some(&start) = next.iter().find(|(_, v)| !v.is_empty()).map(|(k, _)| k) else { break };
        let mut pts = vec![start];
        let mut cur = start;
        loop {
            let Some(list) = next.get_mut(&cur) else { break };
            let Some(n) = list.pop() else { break };
            if n == start {
                break;
            }
            pts.push(n);
            cur = n;
        }
        if pts.len() >= 4 {
            let f: Vec<P> = pts.iter().map(|&(x, y)| (x as f32, y as f32)).collect();
            // Smooth the pixel staircase: midpoints, then simplify.
            let mids: Vec<P> = (0..f.len()).map(|i| {
                let (a, b) = (f[i], f[(i + 1) % f.len()]);
                ((a.0 + b.0) / 2.0, (a.1 + b.1) / 2.0)
            }).collect();
            let s = simplify(&mids, 0.9, true);
            let mut s = s;
            straighten(&mut s, true);
            if s.len() >= 3 {
                loops.push(s);
            }
        }
    }
    loops
}

/// Signed area (positive = counter-clockwise in y-up coordinates).
pub fn area(pts: &[P]) -> f32 {
    let n = pts.len();
    (0..n).map(|i| {
        let (a, b) = (pts[i], pts[(i + 1) % n]);
        a.0 * b.1 - b.0 * a.1
    }).sum::<f32>() / 2.0
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Dir {
    H,
    V,
    D,
}

/// Direction class of a segment: within 12° of horizontal or vertical, or diagonal.
pub fn dir(a: P, b: P) -> Dir {
    let (dx, dy) = ((b.0 - a.0).abs(), (b.1 - a.1).abs());
    let t = 0.21; // tan 12°
    if dy <= t * dx {
        Dir::H
    } else if dx <= t * dy {
        Dir::V
    } else {
        Dir::D
    }
}

/// Straightens a traced line: near-horizontal and near-vertical segments
/// become exactly horizontal and vertical, so boxes and bars come out crisp.
pub fn straighten(pts: &mut [P], closed: bool) {
    let n = pts.len();
    if n < 2 {
        return;
    }
    let segs = if closed { n } else { n - 1 };
    let dirs: Vec<Dir> = (0..segs).map(|i| dir(pts[i], pts[(i + 1) % n])).collect();
    let orig = pts.to_vec();
    for (i, p) in pts.iter_mut().enumerate() {
        let before = if closed || i > 0 { Some((i + segs - 1) % segs) } else { None };
        let after = if closed || i < segs { Some(i % segs) } else { None };
        let mut ys = Vec::new();
        let mut xs = Vec::new();
        for s in [before, after].into_iter().flatten() {
            let (a, b) = (orig[s], orig[(s + 1) % n]);
            match dirs[s] {
                Dir::H => ys.push((a.1 + b.1) / 2.0),
                Dir::V => xs.push((a.0 + b.0) / 2.0),
                Dir::D => {}
            }
        }
        if !ys.is_empty() {
            p.1 = ys.iter().sum::<f32>() / ys.len() as f32;
        }
        if !xs.is_empty() {
            p.0 = xs.iter().sum::<f32>() / xs.len() as f32;
        }
    }
}

/// Lines up horizontal edges that sit at almost the same height (and vertical
/// edges at almost the same x) across the whole sign.
pub fn align(shape: &mut Shape, tol: f32) {
    fn clusters(mut v: Vec<f32>, tol: f32) -> Vec<(f32, f32, f32)> {
        v.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let mut out: Vec<(f32, f32, f32)> = Vec::new(); // (min, max, mean)
        let mut cur: Vec<f32> = Vec::new();
        for x in v {
            if let Some(&last) = cur.last() {
                if x - last > tol {
                    out.push((cur[0], *cur.last().unwrap(), cur.iter().sum::<f32>() / cur.len() as f32));
                    cur.clear();
                }
            }
            cur.push(x);
        }
        if !cur.is_empty() {
            out.push((cur[0], *cur.last().unwrap(), cur.iter().sum::<f32>() / cur.len() as f32));
        }
        out
    }
    let mut hs = Vec::new();
    let mut vs = Vec::new();
    let lines: Vec<(&Vec<P>, bool)> = shape.strokes.iter().map(|l| (&l.pts, l.closed)).chain(shape.blobs.iter().map(|b| (b, true))).collect();
    for (pts, closed) in &lines {
        let n = pts.len();
        let segs = if *closed { n } else { n.saturating_sub(1) };
        for i in 0..segs {
            let (a, b) = (pts[i], pts[(i + 1) % n]);
            if a.1 == b.1 && a.0 != b.0 {
                hs.push(a.1);
            }
            if a.0 == b.0 && a.1 != b.1 {
                vs.push(a.0);
            }
        }
    }
    let hc = clusters(hs, tol);
    let vc = clusters(vs, tol);
    let snap = |v: f32, cs: &[(f32, f32, f32)]| cs.iter().find(|c| v >= c.0 - 1e-3 && v <= c.1 + 1e-3).map(|c| c.2).unwrap_or(v);
    let fix = |pts: &mut Vec<P>, closed: bool| {
        let n = pts.len();
        let segs = if closed { n } else { n.saturating_sub(1) };
        let orig = pts.clone();
        for i in 0..segs {
            let (a, b) = (orig[i], orig[(i + 1) % n]);
            if a.1 == b.1 && a.0 != b.0 {
                let y = snap(a.1, &hc);
                pts[i].1 = y;
                pts[(i + 1) % n].1 = y;
            }
            if a.0 == b.0 && a.1 != b.1 {
                let x = snap(a.0, &vc);
                pts[i].0 = x;
                pts[(i + 1) % n].0 = x;
            }
        }
    };
    for l in &mut shape.strokes {
        fix(&mut l.pts, l.closed);
    }
    for b in &mut shape.blobs {
        fix(b, true);
    }
}
