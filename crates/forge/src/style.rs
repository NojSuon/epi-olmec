//! The three type styles. Each one redraws the same traced centre lines,
//! solid areas, dots and rings with its own pen:
//!
//! * **Sans** (Pal, "plain"): one even line weight, square ends and sharp
//!   corners, like a 黑体 / sans serif.
//! * **Playful** (Juego): heavier, rounded line with round ends, softened
//!   corners and plump dots, in the spirit of the brand's Fredoka.
//! * **Formal** (Estela, "stela"): thick verticals and thin horizontals,
//!   with a small triangular serif at the right end of free horizontals,
//!   like a 宋体 / Song typeface.
//!
//! All coordinates are font units with y up. Filled outlines run clockwise
//! (the TrueType convention) and holes counter-clockwise; overlapping pieces
//! are fine because fonts fill with the non-zero rule.

use crate::vector::{area, dist, Line, Shape, P};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Style {
    Sans,
    Playful,
    Formal,
}

impl Style {
    pub const ALL: [Style; 3] = [Style::Sans, Style::Playful, Style::Formal];

    pub fn id(self) -> &'static str {
        match self {
            Style::Sans => "sans",
            Style::Playful => "juego",
            Style::Formal => "estela",
        }
    }

    pub fn family(self) -> &'static str {
        match self {
            Style::Sans => "Epi-Olmec Pal",
            Style::Playful => "Epi-Olmec Juego",
            Style::Formal => "Epi-Olmec Estela",
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Pt {
    pub x: f32,
    pub y: f32,
    pub on: bool,
}

pub type Contour = Vec<Pt>;

fn poly(pts: &[P], clockwise: bool) -> Contour {
    let mut c: Vec<P> = pts.to_vec();
    let a = area(&c);
    if (a > 0.0) == clockwise {
        c.reverse();
    }
    c.into_iter().map(|(x, y)| Pt { x, y, on: true }).collect()
}

/// Circle from 8 quadratic arcs.
pub fn circle(c: P, r: f32, clockwise: bool) -> Contour {
    let k = 1.0 / (std::f32::consts::PI / 8.0).cos();
    let mut out = Vec::new();
    for i in 0..8 {
        let a = i as f32 * std::f32::consts::PI / 4.0;
        let b = a + std::f32::consts::PI / 8.0;
        out.push(Pt { x: c.0 + r * a.cos(), y: c.1 + r * a.sin(), on: true });
        out.push(Pt { x: c.0 + r * k * b.cos(), y: c.1 + r * k * b.sin(), on: false });
    }
    if clockwise {
        out.reverse();
    }
    out
}

/// Pen sizes, from the typical half width of the source lines (`half`).
struct Pen {
    style: Style,
    /// Width of vertical (and, except in Formal, all) lines.
    wv: f32,
    /// Width of horizontal lines.
    wh: f32,
}

impl Pen {
    fn new(style: Style, half: f32) -> Pen {
        let w = 2.0 * half;
        match style {
            Style::Sans => Pen { style, wv: w * 1.9, wh: w * 1.9 },
            Style::Playful => Pen { style, wv: w * 2.5, wh: w * 2.5 },
            Style::Formal => Pen { style, wv: w * 2.5, wh: w * 0.8 },
        }
    }

}

/// Chaikin corner cutting; open lines keep their end points.
fn chaikin(pts: &[P], closed: bool, rounds: usize) -> Vec<P> {
    let mut p = pts.to_vec();
    for _ in 0..rounds {
        if p.len() < 3 && !closed {
            return p;
        }
        let n = p.len();
        let mut q = Vec::with_capacity(n * 2);
        if !closed {
            q.push(p[0]);
        }
        let segs = if closed { n } else { n - 1 };
        for i in 0..segs {
            let (a, b) = (p[i], p[(i + 1) % n]);
            let first = !closed && i == 0;
            let last = !closed && i == segs - 1;
            if !first {
                q.push((0.75 * a.0 + 0.25 * b.0, 0.75 * a.1 + 0.25 * b.1));
            }
            if !last {
                q.push((0.25 * a.0 + 0.75 * b.0, 0.25 * a.1 + 0.75 * b.1));
            }
        }
        if !closed {
            q.push(p[n - 1]);
        }
        p = q;
    }
    p
}

/// Rounds only the curved parts of a line: corners between two diagonal
/// segments are cut, square corners stay sharp.
fn soften(pts: &[P], closed: bool, rounds: usize) -> Vec<P> {
    use crate::vector::{dir, Dir};
    let mut p = pts.to_vec();
    for _ in 0..rounds {
        let n = p.len();
        if n < 3 {
            return p;
        }
        let mut q = Vec::with_capacity(n * 2);
        for i in 0..n {
            let interior = closed || (i > 0 && i < n - 1);
            if interior {
                let (a, b, c) = (p[(i + n - 1) % n], p[i], p[(i + 1) % n]);
                if dir(a, b) == Dir::D && dir(b, c) == Dir::D && turn(a, b, c) < 1.3 {
                    q.push((0.75 * b.0 + 0.25 * a.0, 0.75 * b.1 + 0.25 * a.1));
                    q.push((0.75 * b.0 + 0.25 * c.0, 0.75 * b.1 + 0.25 * c.1));
                    continue;
                }
            }
            q.push(p[i]);
        }
        p = q;
    }
    p
}

fn turn(a: P, b: P, c: P) -> f32 {
    let (d1, d2) = ((b.0 - a.0, b.1 - a.1), (c.0 - b.0, c.1 - b.1));
    let cross = d1.0 * d2.1 - d1.1 * d2.0;
    let dot = d1.0 * d2.0 + d1.1 * d2.1;
    cross.atan2(dot).abs()
}

fn stroke(line: &Line, pen: &Pen, out: &mut Vec<Contour>) {
    // Very short segments point in random directions; merge close points.
    let min = pen.wv.max(pen.wh) * 0.3;
    let mut base: Vec<P> = Vec::with_capacity(line.pts.len());
    for (i, &p) in line.pts.iter().enumerate() {
        let last = i + 1 == line.pts.len();
        match base.last() {
            Some(&q) if dist(p, q) < min => {
                if last && !line.closed && base.len() > 1 {
                    *base.last_mut().unwrap() = p;
                }
            }
            _ => base.push(p),
        }
    }
    if line.closed && base.len() > 2 && dist(base[0], *base.last().unwrap()) < min {
        base.pop();
    }
    let closed = line.closed && base.len() > 2;
    let pts = if pen.style == Style::Playful { chaikin(&base, closed, 2) } else { soften(&base, closed, 2) };
    let n = pts.len();
    if n == 0 {
        return;
    }
    if n == 1 {
        out.push(circle(pts[0], pen.wv / 2.0, true));
        return;
    }
    let segs = if closed { n } else { n - 1 };
    match pen.style {
        Style::Sans => sans_stroke(&pts, closed, pen.wv, out),
        Style::Playful => {
            // A round pen: every segment is a capsule.
            for i in 0..segs {
                out.push(capsule(pts[i], pts[(i + 1) % n], pen.wv / 2.0));
            }
        }
        Style::Formal => {
            // A flat, wide pen held level: horizontals come out thin and
            // verticals thick, with square corners.
            for i in 0..segs {
                out.push(broad_nib(pts[i], pts[(i + 1) % n], pen.wv / 2.0, pen.wh / 2.0));
            }
            if !closed {
                // Triangular serif at the right end of a free horizontal end.
                for (end, inner) in [(pts[n - 1], pts[n - 2]), (pts[0], pts[1])] {
                    let (dx, dy) = (end.0 - inner.0, end.1 - inner.1);
                    let len = (dx * dx + dy * dy).sqrt();
                    if len < 1e-3 || dx / len < 0.92 {
                        continue;
                    }
                    let top = end.1 + pen.wh / 2.0;
                    let right = end.0 + pen.wv / 2.0;
                    let s = pen.wv * 0.85;
                    out.push(poly(&[(right - s * 1.6, top - 0.5), (right, top - 0.5), (right, top + s * 0.8)], true));
                }
            }
        }
    }
}

/// Hull of two discs: a segment drawn with a round pen.
fn capsule(a: P, b: P, r: f32) -> Contour {
    let len = dist(a, b);
    if len < 1e-3 {
        return circle(a, r, true);
    }
    let ang = (b.1 - a.1).atan2(b.0 - a.0);
    let k = 1.0 / (std::f32::consts::PI / 8.0).cos();
    let mut c = Vec::new();
    // Half circle around b (from +90° to −90° relative to the direction),
    // then around a, in 45° quadratic steps; counter-clockwise, reversed below.
    for (centre, from) in [(b, -std::f32::consts::FRAC_PI_2), (a, std::f32::consts::FRAC_PI_2)] {
        for s in 0..4 {
            let t = ang + from + s as f32 * std::f32::consts::FRAC_PI_4;
            let m = t + std::f32::consts::PI / 8.0;
            c.push(Pt { x: centre.0 + r * t.cos(), y: centre.1 + r * t.sin(), on: true });
            c.push(Pt { x: centre.0 + r * k * m.cos(), y: centre.1 + r * k * m.sin(), on: false });
        }
        let t = ang + from + std::f32::consts::PI;
        c.push(Pt { x: centre.0 + r * t.cos(), y: centre.1 + r * t.sin(), on: true });
    }
    c.reverse();
    c
}

/// A segment swept by an axis-aligned rectangular pen (half sizes hx, hy):
/// the convex hull of the pen at both ends.
fn broad_nib(a: P, b: P, hx: f32, hy: f32) -> Contour {
    let mut pts = Vec::with_capacity(8);
    for c in [a, b] {
        for (sx, sy) in [(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)] {
            pts.push((c.0 + sx * hx, c.1 + sy * hy));
        }
    }
    poly(&hull(pts), true)
}

fn hull(mut p: Vec<P>) -> Vec<P> {
    p.sort_by(|a, b| a.partial_cmp(b).unwrap());
    p.dedup();
    if p.len() < 3 {
        return p;
    }
    let cross = |o: P, a: P, b: P| (a.0 - o.0) * (b.1 - o.1) - (a.1 - o.1) * (b.0 - o.0);
    let mut lower: Vec<P> = Vec::new();
    for &q in &p {
        while lower.len() >= 2 && cross(lower[lower.len() - 2], lower[lower.len() - 1], q) <= 0.0 {
            lower.pop();
        }
        lower.push(q);
    }
    let mut upper: Vec<P> = Vec::new();
    for &q in p.iter().rev() {
        while upper.len() >= 2 && cross(upper[upper.len() - 2], upper[upper.len() - 1], q) <= 0.0 {
            upper.pop();
        }
        upper.push(q);
    }
    lower.pop();
    upper.pop();
    lower.extend(upper);
    lower
}

/// Even-width line with mitred corners and square ends (kurbo's stroker).
fn sans_stroke(pts: &[P], closed: bool, w: f32, out: &mut Vec<Contour>) {
    use kurbo::{stroke, BezPath, Cap, Join, PathEl, Stroke, StrokeOpts};
    let mut path = BezPath::new();
    path.move_to((pts[0].0 as f64, pts[0].1 as f64));
    for p in &pts[1..] {
        path.line_to((p.0 as f64, p.1 as f64));
    }
    if closed {
        path.close_path();
    }
    let st = Stroke::new(w as f64).with_join(Join::Miter).with_miter_limit(2.0).with_caps(Cap::Square);
    let outline = stroke(path, &st, &StrokeOpts::default(), 0.25);
    let mut subs: Vec<Vec<P>> = Vec::new();
    for el in outline.elements() {
        match *el {
            PathEl::MoveTo(p) => subs.push(vec![(p.x as f32, p.y as f32)]),
            PathEl::LineTo(p) => subs.last_mut().unwrap().push((p.x as f32, p.y as f32)),
            PathEl::QuadTo(_, p) | PathEl::CurveTo(_, _, p) => subs.last_mut().unwrap().push((p.x as f32, p.y as f32)),
            PathEl::ClosePath => {}
        }
    }
    // The biggest outline is the outside; for a closed line the other one is
    // the hole in the middle.
    let biggest = (0..subs.len()).max_by(|&a, &b| area(&subs[a]).abs().partial_cmp(&area(&subs[b]).abs()).unwrap());
    for (i, s) in subs.iter().enumerate() {
        if s.len() < 3 {
            continue;
        }
        let outer = !closed || Some(i) == biggest;
        out.push(poly(s, outer));
    }
}

/// Draws a traced shape (font units, y up) with the style's pen.
pub fn draw(shape: &Shape, style: Style, half: f32) -> Vec<Contour> {
    let pen = Pen::new(style, half);
    let mut out = Vec::new();
    for l in &shape.strokes {
        stroke(l, &pen, &mut out);
    }
    for b in &shape.blobs {
        // Blob outlines already have the right orientation (outer clockwise).
        let pts = if style == Style::Playful { chaikin(b, true, 2) } else { b.clone() };
        out.push(pts.into_iter().map(|(x, y)| Pt { x, y, on: true }).collect());
    }
    for &(c, r) in &shape.dots {
        let r = match style {
            Style::Sans => r.max(pen.wv * 0.72),
            Style::Playful => (r * 1.12).max(pen.wv * 0.8),
            Style::Formal => r.max(pen.wv * 0.66),
        };
        out.push(circle(c, r, true));
    }
    for &(c, r) in &shape.rings {
        let w = match style {
            Style::Formal => (pen.wv + pen.wh) / 2.0,
            _ => pen.wv,
        };
        out.push(circle(c, r + w / 2.0, true));
        if r - w / 2.0 > w * 0.25 {
            out.push(circle(c, r - w / 2.0, false));
        }
    }
    out
}

/// Bounding box of drawn outlines (x0, y0, x1, y1).
pub fn bounds(contours: &[Contour]) -> (f32, f32, f32, f32) {
    let mut b = (f32::MAX, f32::MAX, f32::MIN, f32::MIN);
    for c in contours {
        for p in c {
            b = (b.0.min(p.x), b.1.min(p.y), b.2.max(p.x), b.3.max(p.y));
        }
    }
    b
}

/// SVG path data, with y flipped so that `top` (font units) is y = 0.
pub fn svg_path(contours: &[Contour], top: f32) -> String {
    use std::fmt::Write;
    let mut s = String::new();
    let f = |v: f32| v.round() as i32;
    for c in contours {
        let Some(start) = c.iter().position(|p| p.on) else { continue };
        let n = c.len();
        let p0 = c[start];
        let _ = write!(s, "M{} {}", f(p0.x), f(top - p0.y));
        let mut i = 1;
        while i <= n {
            let p = c[(start + i) % n];
            if p.on {
                let _ = write!(s, "L{} {}", f(p.x), f(top - p.y));
                i += 1;
            } else {
                let q = c[(start + i + 1) % n];
                let end = if q.on { q } else { Pt { x: (p.x + q.x) / 2.0, y: (p.y + q.y) / 2.0, on: true } };
                let _ = write!(s, "Q{} {} {} {}", f(p.x), f(top - p.y), f(end.x), f(top - end.y));
                i += if q.on { 2 } else { 1 };
            }
        }
        s.push('Z');
    }
    s
}
