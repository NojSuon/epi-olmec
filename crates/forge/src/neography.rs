//! Builds the neographies (modern proposals, not attested; see
//! `epi_olmec_core::neography` for the rules) from the traced shapes of real
//! signs, so each style's pen draws them like everything else.

use epi_olmec_core::neography::Side;

use crate::vector::{Line, Shape};

/// Height of the donor sign and of the vowel band below it (font units).
const DONOR_W: f32 = 640.0;
const DONOR_H: f32 = 500.0;
const GAP: f32 = 80.0;
const MARK: f32 = 150.0;

/// Donor sign on top, vowel mark below, side stroke for l and r.
pub fn compose(donor: &Shape, v: usize, side: Side, half: f32) -> Shape {
    let (x0, y0, x1, y1) = donor.bounds();
    let k = (DONOR_W / (x1 - x0)).min(DONOR_H / (y1 - y0));
    let (cx, top) = ((x0 + x1) / 2.0, y1);
    // Donor: centred on x = 0, top at y = 0.
    let mut s = donor.map(|(x, y)| ((x - cx) * k, (y - top) * k), k);
    s.half = half;
    let h = (y1 - y0) * k;
    let w = (x1 - x0) * k;
    let ym = -h - GAP - MARK / 2.0; // centre of the vowel band
    let line = |pts: Vec<(f32, f32)>, closed| Line { pts, closed };
    let r = MARK / 2.0;
    match v {
        // i: one vertical stroke
        0 => s.strokes.push(line(vec![(0.0, ym + r), (0.0, ym - r)], false)),
        // e: two vertical strokes
        1 => {
            for x in [-55.0, 55.0] {
                s.strokes.push(line(vec![(x, ym + r), (x, ym - r)], false));
            }
        }
        // ü: a ring
        2 => s.rings.push(((0.0, ym), r * 0.75)),
        // a: a horizontal bar
        3 => s.strokes.push(line(vec![(-130.0, ym), (130.0, ym)], false)),
        // u: a cup
        4 => s.strokes.push(line(
            vec![(-85.0, ym + r), (-85.0, ym), (-62.0, ym - 50.0), (0.0, ym - r), (62.0, ym - 50.0), (85.0, ym), (85.0, ym + r)],
            false,
        )),
        // o: a diamond
        5 => s.strokes.push(line(vec![(0.0, ym + r), (r, ym), (0.0, ym - r), (-r, ym)], true)),
        _ => unreachable!(),
    }
    // Foreign sounds: a stroke down the left side.
    let xs = -w / 2.0 - 110.0;
    let (ya, yb) = (0.0, ym - r);
    match side {
        Side::None => {}
        Side::Line => s.strokes.push(line(vec![(xs, ya), (xs, yb)], false)),
        Side::Zigzag => {
            let n = 7;
            let pts = (0..=n)
                .map(|i| {
                    let t = i as f32 / n as f32;
                    (xs + if i % 2 == 0 { -32.0 } else { 32.0 }, ya + (yb - ya) * t)
                })
                .collect();
            s.strokes.push(line(pts, false));
        }
    }
    s
}
