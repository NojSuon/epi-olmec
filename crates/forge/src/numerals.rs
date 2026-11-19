//! Bar-and-dot numerals (1–19) and the placeholder sign, drawn directly as
//! vector parts so they share each style's pen.
//!
//! As on the La Mojarra stela, dots (ones) sit above the bars (fives),
//! and each bar is drawn as an outlined slab.

use crate::style::{self, Contour, Style};
use crate::vector::{Line, Shape};

const BAR_W: f32 = 620.0;
const BAR_H: f32 = 96.0;
const BAR_GAP: f32 = 46.0;
const DOT_R: f32 = 44.0;
const DOT_STEP: f32 = 150.0;

pub fn shape(n: u32, half: f32) -> Shape {
    let (bars, dots) = (n / 5, n % 5);
    let mut s = Shape { half, ..Default::default() };
    let mut y = 0.0;
    for _ in 0..bars {
        let (x0, x1) = (-BAR_W / 2.0, BAR_W / 2.0);
        let (y0, y1) = (y - BAR_H, y);
        s.strokes.push(Line { pts: vec![(x0, y0), (x0, y1), (x1, y1), (x1, y0)], closed: true });
        y -= BAR_H + BAR_GAP;
    }
    if dots > 0 {
        // Dots go on top of the bars.
        let top = if bars > 0 { DOT_R + BAR_GAP } else { 0.0 };
        let x0 = -(dots as f32 - 1.0) * DOT_STEP / 2.0;
        for i in 0..dots {
            s.dots.push(((x0 + i as f32 * DOT_STEP, top), DOT_R));
        }
    }
    s
}

/// A dotted circle (◌), shown for syllables without a known sign and for
/// Latin letters typed on their own.
pub fn placeholder(half: f32, st: Style) -> Vec<Contour> {
    let r = match st {
        Style::Playful => half * 2.2,
        _ => half * 1.8,
    };
    (0..10)
        .map(|i| {
            let a = i as f32 * std::f32::consts::TAU / 10.0;
            style::circle((230.0 * a.cos(), 230.0 * a.sin()), r, true)
        })
        .collect()
}
