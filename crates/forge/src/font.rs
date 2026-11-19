//! Writes a TrueType font: outlines, horizontal and vertical metrics, a
//! character map (Latin letters for typing, Private Use Area code points for
//! every sign) and ligatures that turn typed Latin into signs.

use kurbo::BezPath;
use write_fonts::{
    tables::{
        cmap::Cmap,
        glyf::{GlyfLocaBuilder, Glyph, SimpleGlyph},
        gsub::{Gsub, Ligature, LigatureSet, LigatureSubstFormat1, SingleSubst, SubstitutionLookup, SubstitutionLookupList},
        head::{Flags, Head, MacStyle},
        hhea::Hhea,
        hmtx::Hmtx,
        layout::{CoverageTable, Feature, FeatureList, FeatureRecord, LangSys, Lookup, LookupFlag, Script, ScriptList, ScriptRecord},
        maxp::Maxp,
        name::{Name, NameRecord},
        os2::{Os2, SelectionFlags},
        post::Post,
        vhea::Vhea,
        vmtx::{LongMetric, Vmtx},
    },
    types::{FWord, Fixed, GlyphId, GlyphId16, LongDateTime, NameId, Tag, UfWord},
    FontBuilder, OffsetMarker,
};

use crate::style::Contour;

pub const UPM: u16 = 1000;
pub const ASCENT: i16 = 880;
pub const DESCENT: i16 = -120;

pub struct GlyphOut {
    pub name: String,
    pub contours: Vec<Contour>,
    pub advance: u16,
    /// Vertical advance (height of the glyph's slot in a column).
    pub vadvance: u16,
    /// Top of the vertical slot (y of the vertical origin).
    pub vtop: i16,
}

pub struct FontSpec<'a> {
    pub family: &'a str,
    pub version: &'a str,
    pub glyphs: Vec<GlyphOut>,
    pub cmap: Vec<(char, usize)>,
    /// (component glyphs, ligature glyph)
    pub ligatures: Vec<(Vec<usize>, usize)>,
    /// Stylistic set 1: (base, alternate).
    pub ss01: Vec<(usize, usize)>,
}

fn bezpath(contours: &[Contour]) -> BezPath {
    let mut p = BezPath::new();
    let r = |v: f32| v.round() as f64;
    for c in contours {
        let Some(start) = c.iter().position(|q| q.on) else { continue };
        let n = c.len();
        p.move_to((r(c[start].x), r(c[start].y)));
        let mut i = 1;
        while i <= n {
            let q = c[(start + i) % n];
            if q.on {
                if i < n {
                    p.line_to((r(q.x), r(q.y)));
                }
                i += 1;
            } else {
                let nx = c[(start + i + 1) % n];
                let (ex, ey) = if nx.on { (nx.x, nx.y) } else { ((q.x + nx.x) / 2.0, (q.y + nx.y) / 2.0) };
                p.quad_to((r(q.x), r(q.y)), (r(ex), r(ey)));
                i += if nx.on { 2 } else { 1 };
            }
        }
        p.close_path();
    }
    p
}

fn gid(i: usize) -> GlyphId16 {
    GlyphId16::new(i as u16)
}

pub fn build(spec: &FontSpec) -> Vec<u8> {
    let n = spec.glyphs.len();

    // Outlines and bounds.
    let mut glyf = GlyfLocaBuilder::new();
    let mut bounds = Vec::with_capacity(n);
    let (mut max_points, mut max_contours) = (0u16, 0u16);
    for g in &spec.glyphs {
        let path = bezpath(&g.contours);
        if g.contours.is_empty() {
            glyf.add_glyph(&Glyph::Empty).unwrap();
            bounds.push(None);
        } else {
            let sg = SimpleGlyph::from_bezpath(&path).unwrap_or_else(|e| panic!("{}: {e:?}", g.name));
            let b = sg.bbox;
            max_points = max_points.max(sg.contours.iter().map(|c| c.len()).sum::<usize>() as u16);
            max_contours = max_contours.max(sg.contours.len() as u16);
            bounds.push(Some((b.x_min, b.y_min, b.x_max, b.y_max)));
            glyf.add_glyph(&sg).unwrap();
        }
    }
    let (glyf, loca, loca_format) = glyf.build();

    let all = bounds.iter().flatten();
    let x_min = all.clone().map(|b| b.0).min().unwrap_or(0);
    let y_min = all.clone().map(|b| b.1).min().unwrap_or(0);
    let x_max = all.clone().map(|b| b.2).max().unwrap_or(0);
    let y_max = all.clone().map(|b| b.3).max().unwrap_or(0);

    let head = Head::new(
        Fixed::from_f64(1.0),
        0,
        Flags::BASELINE_AT_Y_0 | Flags::LSB_AT_X_0,
        UPM,
        LongDateTime::new(0),
        LongDateTime::new(0),
        x_min,
        y_min,
        x_max,
        y_max,
        MacStyle::empty(),
        8,
        loca_format as i16,
    );

    // Horizontal metrics.
    let h_metrics: Vec<LongMetric> = spec
        .glyphs
        .iter()
        .zip(&bounds)
        .map(|(g, b)| LongMetric { advance: g.advance, side_bearing: b.map(|b| b.0).unwrap_or(0) })
        .collect();
    let max_adv = spec.glyphs.iter().map(|g| g.advance).max().unwrap_or(0);
    let hhea = Hhea::new(
        FWord::new(ASCENT),
        FWord::new(DESCENT),
        FWord::new(200),
        UfWord::new(max_adv),
        FWord::new(bounds.iter().flatten().map(|b| b.0).min().unwrap_or(0)),
        FWord::new(spec.glyphs.iter().zip(&bounds).filter_map(|(g, b)| b.map(|b| g.advance as i16 - b.2)).min().unwrap_or(0)),
        FWord::new(x_max),
        1,
        0,
        0,
        n as u16,
    );
    let hmtx = Hmtx::new(h_metrics, vec![]);

    // Vertical metrics: each sign takes as much of the column as it needs.
    let v_metrics: Vec<LongMetric> = spec
        .glyphs
        .iter()
        .zip(&bounds)
        .map(|(g, b)| LongMetric { advance: g.vadvance, side_bearing: b.map(|b| g.vtop - b.3).unwrap_or(0) })
        .collect();
    let max_vadv = spec.glyphs.iter().map(|g| g.vadvance).max().unwrap_or(0);
    let vhea = Vhea::new(
        FWord::new(500),
        FWord::new(-500),
        FWord::new(0),
        UfWord::new(max_vadv),
        FWord::new(v_metrics.iter().map(|m| m.side_bearing).min().unwrap_or(0)),
        FWord::new(0),
        FWord::new(max_vadv as i16),
        0,
        1,
        0,
        n as u16,
    );
    let vmtx = Vmtx::new(v_metrics, vec![]);

    let mut maxp = Maxp::new(n as u16);
    maxp.max_points = Some(max_points);
    maxp.max_contours = Some(max_contours);
    maxp.max_composite_points = Some(0);
    maxp.max_composite_contours = Some(0);
    maxp.max_zones = Some(2);
    maxp.max_twilight_points = Some(0);
    maxp.max_storage = Some(0);
    maxp.max_function_defs = Some(0);
    maxp.max_instruction_defs = Some(0);
    maxp.max_stack_elements = Some(0);
    maxp.max_size_of_instructions = Some(0);
    maxp.max_component_elements = Some(0);
    maxp.max_component_depth = Some(0);

    let cmap = Cmap::from_mappings(spec.cmap.iter().map(|&(c, g)| (c, GlyphId::new(g as u32)))).expect("cmap");
    let first = spec.cmap.iter().map(|c| c.0 as u32).min().unwrap_or(0x20);
    let last = spec.cmap.iter().map(|c| c.0 as u32).max().unwrap_or(0x20);

    let os2 = Os2 {
        x_avg_char_width: 1000,
        us_weight_class: 400,
        us_width_class: 5,
        fs_type: 0,
        y_subscript_x_size: 650,
        y_subscript_y_size: 600,
        y_subscript_y_offset: 75,
        y_superscript_x_size: 650,
        y_superscript_y_size: 600,
        y_superscript_y_offset: 350,
        y_strikeout_size: 50,
        y_strikeout_position: 300,
        ach_vend_id: Tag::new(b"MATI"),
        fs_selection: SelectionFlags::REGULAR | SelectionFlags::USE_TYPO_METRICS,
        us_first_char_index: first.min(0xFFFF) as u16,
        us_last_char_index: last.min(0xFFFF) as u16,
        s_typo_ascender: ASCENT,
        s_typo_descender: DESCENT,
        s_typo_line_gap: 200,
        us_win_ascent: (y_max.max(ASCENT)) as u16,
        us_win_descent: (-y_min.min(DESCENT)) as u16,
        ul_code_page_range_1: Some(1),
        ul_code_page_range_2: Some(0),
        sx_height: Some(500),
        s_cap_height: Some(700),
        us_default_char: Some(0),
        us_break_char: Some(32),
        us_max_context: Some(spec.ligatures.iter().map(|l| l.0.len() as u16).max().unwrap_or(1)),
        // Private Use Area bit.
        ul_unicode_range_2: 1 << (60 - 32),
        ..Default::default()
    };

    let post = Post::new_v2(spec.glyphs.iter().map(|g| g.name.as_str()));

    let ps_name: String = spec.family.chars().filter(|c| c.is_ascii_alphanumeric() || *c == '-').collect();
    let names = [
        (0, "Glyphs vectorized by Mati Mati from the Epi-Olmec syllabary (Kaufman & Justeson 2001)."),
        (1, spec.family),
        (2, "Regular"),
        (3, &format!("{};{};MATI", spec.version, ps_name)),
        (4, spec.family),
        (5, &format!("Version {}", spec.version)),
        (6, &format!("{ps_name}-Regular")),
        (13, "This Font Software is licensed under the SIL Open Font License, Version 1.1."),
        (14, "https://openfontlicense.org"),
    ];
    let mut records: Vec<NameRecord> = names
        .iter()
        .map(|(id, s)| NameRecord::new(3, 1, 0x409, NameId::new(*id), OffsetMarker::new(s.to_string())))
        .collect();
    records.sort_by_key(|r| (r.platform_id, r.encoding_id, r.language_id, r.name_id));
    let name = Name::new(records);

    let gsub = gsub(spec);

    let mut fb = FontBuilder::new();
    fb.add_table(&head).unwrap();
    fb.add_table(&hhea).unwrap();
    fb.add_table(&hmtx).unwrap();
    fb.add_table(&vhea).unwrap();
    fb.add_table(&vmtx).unwrap();
    fb.add_table(&maxp).unwrap();
    fb.add_table(&os2).unwrap();
    fb.add_table(&cmap).unwrap();
    fb.add_table(&name).unwrap();
    fb.add_table(&post).unwrap();
    fb.add_table(&glyf).unwrap();
    fb.add_table(&loca).unwrap();
    fb.add_table(&gsub).unwrap();
    fb.build()
}

fn gsub(spec: &FontSpec) -> Gsub {
    use std::collections::BTreeMap;
    // Ligatures grouped by first glyph, longest first.
    let mut by_first: BTreeMap<usize, Vec<(Vec<usize>, usize)>> = BTreeMap::new();
    for (comps, lig) in &spec.ligatures {
        by_first.entry(comps[0]).or_default().push((comps[1..].to_vec(), *lig));
    }
    let coverage: Vec<GlyphId16> = by_first.keys().map(|&g| gid(g)).collect();
    let sets: Vec<LigatureSet> = by_first
        .into_values()
        .map(|mut ligs| {
            ligs.sort_by(|a, b| b.0.len().cmp(&a.0.len()));
            LigatureSet::new(ligs.into_iter().map(|(rest, lig)| Ligature::new(gid(lig), rest.into_iter().map(gid).collect())).collect())
        })
        .collect();
    let liga = LigatureSubstFormat1::new(CoverageTable::from(coverage), sets);
    let lookup0 = SubstitutionLookup::Ligature(Lookup::new(LookupFlag::empty(), vec![liga.into()]));

    let mut ss = spec.ss01.clone();
    ss.sort();
    let single = write_fonts::tables::gsub::SingleSubstFormat2::new(
        CoverageTable::from(ss.iter().map(|s| gid(s.0)).collect::<Vec<_>>()),
        ss.iter().map(|s| gid(s.1)).collect(),
    );
    let lookup1 = SubstitutionLookup::Single(Lookup::new(LookupFlag::empty(), vec![SingleSubst::Format2(single)]));

    let features = FeatureList::new(vec![
        FeatureRecord::new(Tag::new(b"liga"), Feature::new(None, vec![0])),
        FeatureRecord::new(Tag::new(b"rlig"), Feature::new(None, vec![0])),
        FeatureRecord::new(Tag::new(b"ss01"), Feature::new(None, vec![1])),
    ]);
    let lang = || LangSys::new(vec![0, 1, 2]);
    let scripts = ScriptList::new(vec![
        ScriptRecord::new(Tag::new(b"DFLT"), Script::new(Some(lang()), vec![])),
        ScriptRecord::new(Tag::new(b"latn"), Script::new(Some(lang()), vec![])),
    ]);
    Gsub::new(scripts, features, SubstitutionLookupList::new(vec![lookup0, lookup1]))
}
