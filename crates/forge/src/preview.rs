//! PNG previews of the drawn signs, to check the vectorization by eye.

use std::path::Path;

use tiny_skia::{FillRule, Paint, PathBuilder, Pixmap, Transform};

use crate::style::Contour;

const CELL: f32 = 150.0;

fn fill(pm: &mut Pixmap, contours: &[Contour], x: f32, y: f32, scale: f32) {
    let mut pb = PathBuilder::new();
    for c in contours {
        let Some(start) = c.iter().position(|p| p.on) else { continue };
        let n = c.len();
        let tr = |px: f32, py: f32| (x + px * scale, y + (880.0 - py) * scale);
        let (sx, sy) = tr(c[start].x, c[start].y);
        pb.move_to(sx, sy);
        let mut i = 1;
        while i <= n {
            let p = c[(start + i) % n];
            if p.on {
                let (px, py) = tr(p.x, p.y);
                pb.line_to(px, py);
                i += 1;
            } else {
                let q = c[(start + i + 1) % n];
                let (ex, ey) = if q.on { (q.x, q.y) } else { ((p.x + q.x) / 2.0, (p.y + q.y) / 2.0) };
                let (cx, cy) = tr(p.x, p.y);
                let (ex, ey) = tr(ex, ey);
                pb.quad_to(cx, cy, ex, ey);
                i += if q.on { 2 } else { 1 };
            }
        }
        pb.close();
    }
    if let Some(path) = pb.finish() {
        let mut paint = Paint::default();
        paint.set_color_rgba8(43, 29, 26, 255);
        paint.anti_alias = true;
        pm.fill_path(&path, &paint, FillRule::Winding, Transform::identity(), None);
    }
}

fn grid(items: &[Option<&Vec<Contour>>], cols: usize, path: &Path) {
    let rows = items.len().div_ceil(cols).max(1);
    let mut pm = Pixmap::new((cols as f32 * CELL) as u32, (rows as f32 * CELL) as u32).unwrap();
    pm.fill(tiny_skia::Color::from_rgba8(255, 249, 245, 255));
    let scale = (CELL - 20.0) / 1000.0;
    for (i, item) in items.iter().enumerate() {
        if let Some(c) = item {
            let (gx, gy) = ((i % cols) as f32 * CELL, (i / cols) as f32 * CELL);
            fill(&mut pm, c, gx + 10.0, gy + 10.0, scale);
        }
    }
    pm.save_png(path).unwrap();
}

/// The syllabary laid out like the chart (rows p…y, columns i…ü…o).
pub fn chart(drawn: &[(String, Vec<Contour>)], path: &Path) {
    use epi_olmec_core as core;
    let mut items = Vec::new();
    for c in 0..11 {
        for v in 0..6 {
            let name = core::syllable_name(c, v, false);
            items.push(drawn.iter().find(|d| d.0 == name).map(|d| &d.1));
        }
    }
    grid(&items, 6, path);
}

/// Alternates, logograms and numerals.
pub fn extras(drawn: &[(String, Vec<Contour>)], path: &Path) {
    let items: Vec<_> = drawn
        .iter()
        .filter(|d| d.0.ends_with(".alt") || !d.0.chars().any(|c| c.is_ascii_digit()) && (d.0.len() > 3 || ["kak", "pak", "pvk", "yaj"].contains(&d.0.as_str())) || d.0.starts_with("num"))
        .map(|d| Some(&d.1))
        .collect();
    grid(&items, 8, path);
}

/// The same signs in every style, one style per row, at a larger size.
pub fn compare(styles: &[Vec<(String, Vec<Contour>)>], names: &[&str], path: &Path) {
    let cell = if names.len() <= 2 { 800.0 } else { 300.0 };
    let mut pm = Pixmap::new((names.len() as f32 * cell) as u32, (styles.len() as f32 * cell) as u32).unwrap();
    pm.fill(tiny_skia::Color::from_rgba8(255, 249, 245, 255));
    let scale = (cell - 30.0) / 1000.0;
    for (r, drawn) in styles.iter().enumerate() {
        for (c, name) in names.iter().enumerate() {
            if let Some(d) = drawn.iter().find(|d| d.0 == *name) {
                fill(&mut pm, &d.1, c as f32 * cell + 15.0, r as f32 * cell + 15.0, scale);
            }
        }
    }
    pm.save_png(path).unwrap();
}
