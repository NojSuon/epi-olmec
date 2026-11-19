//! `forge`: vectorizes the Epi-Olmec signs and builds the fonts.
//!
//! ```text
//! cargo run -p epi-olmec-forge --release            # fonts + site data + previews
//! ```
//!
//! Inputs (in `sources/`):
//! * `syllabary-chart.png`: the syllabary chart, 11 consonant rows × 6 vowel
//!   columns;
//! * `logograms/<name>.png`: one logogram per file, cropped from Kaufman &
//!   Justeson (2001).
//!
//! Outputs: `fonts/*.ttf`, `site/data/glyphs.json`, `previews/*.png`.

mod font;
mod neography;
mod numerals;
mod preview;
mod raster;
mod style;
mod vector;

use std::{collections::BTreeMap, fs, path::Path};

use epi_olmec_core as core;
use raster::Gray;
use style::{Contour, Style};
use vector::Shape;

/// Font units per pixel of the chart. Chart signs are ~85 px wide, which
/// gives signs of about 770 units on a 1000-unit em.
const CHART_SCALE: f32 = 9.0;
/// Upscaling before tracing, for smoother centre lines.
const UPSCALE: usize = 4;
/// Vertical centre of the signs (the em box runs from −120 to 880).
const CENTER_Y: f32 = 380.0;
/// Space above and below each sign in a column.
const V_GAP: f32 = 70.0;
/// Width that logograms are scaled to.
const LOGOGRAM_WIDTH: f32 = 780.0;
pub const VERSION: &str = "1.000";

// Centres of the chart's columns (i e ü a u o) and rows (p t ts k h s j m n w y).
const COLS: [f32; 6] = [195.0, 333.0, 465.0, 597.0, 731.0, 863.0];
const ROWS: [f32; 11] = [160.0, 285.0, 420.0, 550.0, 685.0, 820.0, 950.0, 1085.0, 1220.0, 1350.0, 1470.0];

pub struct Sign {
    pub name: String,
    /// Traced shape in font units, y up (not yet placed).
    pub shape: Shape,
}

fn main() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let root = root.canonicalize().unwrap();
    let sources = root.join("sources");

    let mut signs = chart_signs(&sources.join("syllabary-chart.png"));
    let mut halves: Vec<f32> = signs.iter().map(|s| s.shape.half).collect();
    halves.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let half = halves[halves.len() / 2];
    println!("{} syllable signs, line half width {:.1} units", signs.len(), half);

    for l in core::LOGOGRAMS.iter() {
        let path = sources.join("logograms").join(format!("{}.png", l.name));
        if !path.exists() {
            eprintln!("warning: missing {}", path.display());
            continue;
        }
        signs.push(Sign { name: l.name.to_string(), shape: logogram_shape(&path) });
    }
    // Closed-syllable signs (kak, pak, pük, yaj), drawn in K&J 2001 §1.2.
    for s in core::CVC.iter() {
        let path = sources.join("cvc").join(format!("{}.png", s.name));
        signs.push(Sign { name: s.name.to_string(), shape: logogram_shape(&path) });
    }
    // Neographies: modern proposals built from real signs (not attested).
    for (c, v) in core::neography::all() {
        let ((dc, dv), side) = core::neography::donor(c);
        let donor = &signs.iter().find(|s| s.name == core::syllable_name(dc, dv, false)).expect("donor").shape;
        let shape = neography::compose(donor, v, side, half);
        signs.push(Sign { name: core::neography::name(c, v), shape });
    }
    for n in 0..=19 {
        signs.push(Sign { name: core::numeral_name(n), shape: numerals::shape(n, half) });
    }

    let out_fonts = root.join("fonts");
    let out_prev = root.join("previews");
    let out_data = root.join("site/data");
    for d in [&out_fonts, &out_prev, &out_data] {
        fs::create_dir_all(d).unwrap();
    }

    let mut json_styles = BTreeMap::new();
    let mut compare = Vec::new();
    for st in Style::ALL {
        let drawn: Vec<(String, Vec<Contour>)> = signs
            .iter()
            .map(|s| (s.name.clone(), place(style::draw(&s.shape, st, half))))
            .collect();
        let placeholder = place(numerals::placeholder(half, st));

        let spec = font_spec(st, &drawn, &placeholder);
        let bytes = font::build(&spec);
        let file = out_fonts.join(format!("EpiOlmec-{}.ttf", capitalize(st.id())));
        fs::write(&file, &bytes).unwrap();
        println!("{} ({} glyphs, {} KB)", file.display(), spec.glyphs.len(), bytes.len() / 1024);

        preview::chart(&drawn, &out_prev.join(format!("{}-syllabary.png", st.id())));
        preview::extras(&drawn, &out_prev.join(format!("{}-extras.png", st.id())));
        compare.push(drawn.clone());

        let mut glyphs = BTreeMap::new();
        for g in spec.glyphs.iter().filter(|g| drawn.iter().any(|d| d.0 == g.name) || g.name == "placeholder") {
            glyphs.insert(
                g.name.clone(),
                serde_json::json!({
                    "d": style::svg_path(&g.contours, font::ASCENT as f32),
                    "vadv": g.vadvance,
                    "vtop": font::ASCENT - g.vtop,
                }),
            );
        }
        json_styles.insert(st.id(), serde_json::json!({ "family": st.family(), "glyphs": glyphs }));
    }

    preview::compare(&compare, &["ki"], &out_prev.join("zoom.png"));
    preview::neo(&compare, &out_prev.join("neographies.png"));
    preview::compare(&compare, &["pi", "ki", "sa", "ja", "kak", "pvk", "sky", "num0", "num13"], &out_prev.join("compare.png"));

    let meta = serde_json::json!({
        "version": VERSION,
        "em": { "ascent": font::ASCENT, "descent": font::DESCENT, "advance": 1000 },
        "styles": json_styles,
        "syllables": core::syllables().iter().map(|&(c, v, alt)| serde_json::json!({
            "name": core::syllable_name(c, v, alt),
            "label": core::syllable_label(c, v),
            "c": c, "v": v, "alt": alt,
            "cp": format!("{:04X}", core::syllable_codepoint(c, v, alt) as u32),
        })).collect::<Vec<_>>(),
        "logograms": core::LOGOGRAMS.iter().enumerate().map(|(i, l)| serde_json::json!({
            "name": l.name, "es": l.es, "en": l.en, "reading": l.reading,
            "cp": format!("{:04X}", core::logogram_codepoint(i) as u32),
        })).collect::<Vec<_>>(),
        "neo": core::neography::all().iter().map(|&(c, v)| serde_json::json!({
            "name": core::neography::name(c, v), "label": core::syllable_label(c, v), "c": c, "v": v,
            "cp": format!("{:04X}", core::neography::codepoint(c, v) as u32),
        })).collect::<Vec<_>>(),
        "cvc": core::CVC.iter().enumerate().map(|(i, s)| serde_json::json!({
            "name": s.name, "label": s.label,
            "cp": format!("{:04X}", core::cvc_codepoint(i) as u32),
        })).collect::<Vec<_>>(),
    });
    fs::write(out_data.join("glyphs.json"), serde_json::to_string(&meta).unwrap()).unwrap();
    println!("site/data/glyphs.json written");
}

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    c.next().map(|f| f.to_uppercase().collect::<String>() + c.as_str()).unwrap_or_default()
}

/// Centres a drawn sign in the em box, shrinking it if it is too big.
fn place(mut contours: Vec<Contour>) -> Vec<Contour> {
    if contours.is_empty() {
        return contours;
    }
    let (x0, y0, x1, y1) = style::bounds(&contours);
    let (w, h) = (x1 - x0, y1 - y0);
    let k = (960.0 / w).min(1000.0 / h).min(1.0);
    let (cx, cy) = ((x0 + x1) / 2.0, (y0 + y1) / 2.0);
    for c in &mut contours {
        for p in c {
            p.x = (p.x - cx) * k + 500.0;
            p.y = (p.y - cy) * k + CENTER_Y;
        }
    }
    contours
}

/// Splits the chart into cells and traces every sign.
fn chart_signs(path: &Path) -> Vec<Sign> {
    let img = Gray::load(path);
    let mask = img.threshold(0.45);
    let comps: Vec<_> = mask
        .components()
        .into_iter()
        .filter(|c| c.width() < 400 && c.height() < 400) // drop the frame
        .filter(|c| {
            let (x, y) = c.centroid();
            x > 115.0 && y > 85.0 // drop the row and column labels
        })
        .collect();

    let mut cells: BTreeMap<(usize, usize), Vec<usize>> = BTreeMap::new();
    for (i, c) in comps.iter().enumerate() {
        let (x, y) = c.centroid();
        let col = (0..6).min_by(|&a, &b| (COLS[a] - x).abs().partial_cmp(&(COLS[b] - x).abs()).unwrap()).unwrap();
        let row = (0..11).min_by(|&a, &b| (ROWS[a] - y).abs().partial_cmp(&(ROWS[b] - y).abs()).unwrap()).unwrap();
        cells.entry((row, col)).or_default().push(i);
    }

    let mut out = Vec::new();
    for (c, v, alt) in core::syllables() {
        let Some(ids) = cells.get(&(c, v)) else {
            eprintln!("warning: no sign found for {}", core::syllable_label(c, v));
            continue;
        };
        let group: Vec<usize> = if core::has_alternate(c, v) {
            let (top, bottom) = split_rows(ids, &comps);
            if alt { bottom } else { top }
        } else {
            ids.clone()
        };
        let name = core::syllable_name(c, v, alt);
        let shape = trace_group(&img, &comps, &group);
        out.push(Sign { name, shape });
    }
    out
}

/// Splits a cell's pieces at the widest horizontal gap (top form, bottom form).
fn split_rows(ids: &[usize], comps: &[raster::Component]) -> (Vec<usize>, Vec<usize>) {
    let mut spans: Vec<(usize, usize)> = ids.iter().map(|&i| (comps[i].y0, comps[i].y1)).collect();
    spans.sort();
    let mut best = (0usize, 0isize);
    let mut reach = spans[0].1;
    for s in &spans[1..] {
        let gap = s.0 as isize - reach as isize;
        if gap > best.1 {
            best = (s.0, gap);
        }
        reach = reach.max(s.1);
    }
    let cut = best.0;
    let top = ids.iter().copied().filter(|&i| comps[i].y0 < cut).collect();
    let bottom = ids.iter().copied().filter(|&i| comps[i].y0 >= cut).collect();
    (top, bottom)
}

fn trace_group(img: &Gray, comps: &[raster::Component], group: &[usize]) -> Shape {
    let m = 4;
    let x0 = group.iter().map(|&i| comps[i].x0).min().unwrap().saturating_sub(m);
    let y0 = group.iter().map(|&i| comps[i].y0).min().unwrap().saturating_sub(m);
    let x1 = group.iter().map(|&i| comps[i].x1).max().unwrap() + m + 1;
    let y1 = group.iter().map(|&i| comps[i].y1).max().unwrap() + m + 1;
    // Keep only this sign's ink (and its anti-aliased edge).
    let mut keep = raster::Mask::new(x1 - x0, y1 - y0);
    for &i in group {
        for &(x, y) in &comps[i].pixels {
            for dy in -1isize..=1 {
                for dx in -1isize..=1 {
                    let (kx, ky) = (x as isize - x0 as isize + dx, y as isize - y0 as isize + dy);
                    if kx >= 0 && ky >= 0 && (kx as usize) < keep.w && (ky as usize) < keep.h {
                        keep.set(kx as usize, ky as usize, true);
                    }
                }
            }
        }
    }
    let mut crop = img.crop(x0, y0, x1, y1);
    for i in 0..crop.px.len() {
        if !keep.px[i] {
            crop.px[i] = 0.0;
        }
    }
    let big = crop.upscale(UPSCALE);
    let shape = vector::vectorize(&big.threshold(0.5));
    let k = CHART_SCALE / UPSCALE as f32;
    shape.map(|(x, y)| (x * k, -y * k), k)
}

/// Traces a logogram cropped from a 1-bit scan.
fn logogram_shape(path: &Path) -> Shape {
    let img = Gray::load(path).pad(4).blur(1);
    let f = 6;
    let big = img.upscale(f);
    let shape = vector::vectorize(&big.threshold(0.42));
    let (x0, _, x1, _) = shape.bounds();
    let k = LOGOGRAM_WIDTH / (x1 - x0).max(1.0);
    shape.map(|(x, y)| (x * k, -y * k), k)
}

/// Glyph order, character map and ligatures of one font.
fn font_spec<'a>(st: Style, drawn: &[(String, Vec<Contour>)], placeholder: &[Contour]) -> font::FontSpec<'a> {
    use font::GlyphOut;
    let mut glyphs: Vec<GlyphOut> = Vec::new();
    let mut index: BTreeMap<String, usize> = BTreeMap::new();
    let mut push = |glyphs: &mut Vec<GlyphOut>, name: &str, contours: Vec<Contour>, adv: u16| {
        let (vadv, vtop) = if contours.is_empty() {
            (500, (CENTER_Y + 250.0) as i16)
        } else {
            let (_, y0, _, y1) = style::bounds(&contours);
            ((y1 - y0 + 2.0 * V_GAP).round() as u16, (y1 + V_GAP).round() as i16)
        };
        index.insert(name.to_string(), glyphs.len());
        glyphs.push(GlyphOut { name: name.to_string(), contours, advance: adv, vadvance: vadv, vtop });
    };

    // .notdef: an empty box.
    let bx = |x0: f32, y0: f32, x1: f32, y1: f32, cw: bool| -> Contour {
        let mut c = vec![(x0, y0), (x0, y1), (x1, y1), (x1, y0)];
        if !cw {
            c.reverse();
        }
        c.into_iter().map(|(x, y)| style::Pt { x, y, on: true }).collect()
    };
    push(&mut glyphs, ".notdef", vec![bx(150.0, 0.0, 850.0, 760.0, true), bx(200.0, 50.0, 800.0, 710.0, false)], 1000);
    push(&mut glyphs, "space", vec![], 500);
    push(&mut glyphs, "placeholder", placeholder.to_vec(), 1000);

    // Latin letters typed by the user: each needs its own glyph so that
    // ligatures can tell them apart. On their own they show the placeholder.
    let typed: Vec<char> = "abcdefghijklmnopqrstuvwxyz0123456789üʉɨ'’ʼʔ~[]#".chars().collect();
    for &ch in &typed {
        push(&mut glyphs, &format!("in.{:04X}", ch as u32), placeholder.to_vec(), 1000);
    }
    for (name, contours) in drawn {
        push(&mut glyphs, name, contours.clone(), 1000);
    }

    let g = |name: &str| *index.get(name).unwrap_or_else(|| panic!("no glyph {name}"));
    let gin = |ch: char| g(&format!("in.{:04X}", ch as u32));

    let mut cmap = vec![(' ', g("space")), ('\u{3000}', g("space")), ('\u{25CC}', g("placeholder"))];
    for &ch in &typed {
        cmap.push((ch, gin(ch)));
        for up in ch.to_uppercase() {
            if up != ch {
                cmap.push((up, gin(ch)));
            }
        }
    }

    let mut ligatures: Vec<(Vec<usize>, usize)> = Vec::new();
    let mut ss01 = Vec::new();
    let consonant_spellings: [&[&str]; 13] = [
        // The glottal-stop row is the bare vowel; h 7 ' ʔ before it are silent spellings. tz = ts.
        &["p"], &["t"], &["ts", "tz"], &["k"], &["", "h", "7", "'", "’", "ʼ", "ʔ"], &["s"], &["j"], &["m"], &["n"], &["w"], &["y"], &["l"], &["r"],
    ];
    let vowel_spellings: [&[&str]; 6] = [&["i"], &["e"], &["ü", "v", "ʉ", "ɨ"], &["a"], &["u"], &["o"]];
    let seq = |s: &str| -> Vec<usize> { s.chars().map(gin).collect() };
    for (c, v, alt) in core::syllables() {
        let name = core::syllable_name(c, v, alt);
        let Some(&gl) = index.get(&name) else { continue };
        cmap.push((core::syllable_codepoint(c, v, alt), gl));
        for cs in consonant_spellings[c] {
            for vs in vowel_spellings[v] {
                let typed = format!("{cs}{vs}{}", if alt { "~" } else { "" });
                ligatures.push((seq(&typed), gl));
            }
        }
        if alt {
            ss01.push((g(&core::syllable_name(c, v, false)), gl));
        }
    }
    // Neographies (modern proposals): typed like any syllable, e.g. ka, la.
    for (c, v) in core::neography::all() {
        let gl = g(&core::neography::name(c, v));
        cmap.push((core::neography::codepoint(c, v), gl));
        for cs in consonant_spellings[c] {
            for vs in vowel_spellings[v] {
                ligatures.push((seq(&format!("{cs}{vs}")), gl));
            }
        }
    }
    for (i, l) in core::LOGOGRAMS.iter().enumerate() {
        let Some(&gl) = index.get(l.name) else { continue };
        cmap.push((core::logogram_codepoint(i), gl));
        for key in [l.name, l.es] {
            ligatures.push((seq(&format!("[{key}]")), gl));
        }
    }
    for (i, s) in core::CVC.iter().enumerate() {
        let gl = g(s.name);
        cmap.push((core::cvc_codepoint(i), gl));
        // [kak]; pük also as [pvk].
        for key in [s.label, s.name] {
            ligatures.push((seq(&format!("[{key}]")), gl));
        }
    }
    for n in 0..=19u32 {
        let gl = g(&core::numeral_name(n));
        cmap.push((core::numeral_codepoint(n), gl));
        ligatures.push((seq(&format!("#{n}")), gl));
    }

    font::FontSpec { family: st.family(), version: VERSION, glyphs, cmap, ligatures, ss01 }
}
