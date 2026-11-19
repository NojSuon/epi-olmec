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
    if n == 0 {
        return shell(half);
    }
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

/// Zero as a shell, the Mesoamerican sign for completion. No zero is
/// attested in the surviving Epi-Olmec texts, so this follows the general
/// Mesoamerican convention: a shell seen from the side, with its lip and ribs.
fn shell(half: f32) -> Shape {
    let mut s = Shape { half, ..Default::default() };
    // Body: a wide oval, slightly flattened on top.
    let body: Vec<(f32, f32)> = (0..36)
        .map(|i| {
            let a = i as f32 * std::f32::consts::TAU / 36.0;
            let y = if a.sin() > 0.0 { 150.0 * a.sin() } else { 190.0 * a.sin() };
            (310.0 * a.cos(), y)
        })
        .collect();
    s.strokes.push(Line { pts: body, closed: true });
    // Lip: an arc across the top third.
    let lip: Vec<(f32, f32)> = (0..=12)
        .map(|i| {
            let t = i as f32 / 12.0;
            let x = -255.0 + 510.0 * t;
            (x, 40.0 + 60.0 * (1.0 - (2.0 * t - 1.0).powi(2)))
        })
        .collect();
    s.strokes.push(Line { pts: lip, closed: false });
    // Ribs fanning down from the lip.
    for (x0, x1) in [(-130.0, -170.0), (0.0, 0.0), (130.0, 170.0)] {
        s.strokes.push(Line { pts: vec![(x0, 40.0), (x1, -120.0)], closed: false });
    }
    // Two dots above the lip.
    s.dots.push(((-80.0, 105.0), DOT_R * 0.55));
    s.dots.push(((80.0, 105.0), DOT_R * 0.55));
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
