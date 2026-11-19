//! Epi-Olmec syllabary inventory and the rules that turn Latin text into
//! a sequence of glyphs.
//!
//! The syllabary follows Kaufman & Justeson, *Epi-Olmec Hieroglyphic Writing
//! and Texts* (2001), figure 4: 11 consonants × 6 vowels, of which the signs for
//! 53 syllables are known.

pub mod logograms;
pub mod neography;
pub mod spell;

pub use logograms::{Logogram, LOGOGRAMS};
pub use spell::{spell, spell_with, Mode, Token};

/// Consonants in chart order. `name` is ASCII and used in glyph names,
/// `label` is how the consonant is written in the practical orthography.
pub struct Consonant {
    pub name: &'static str,
    pub label: &'static str,
    /// Weak consonants (h = glottal stop, j, w, y) are not written when no vowel follows.
    pub weak: bool,
}

/// The first 11 are the consonants of the language and of the chart. The
/// last two, l and r, did not exist in Epi-Olmec; they are only written with
/// neographies (see `neography`).
pub const CONSONANTS: [Consonant; 13] = [
    Consonant { name: "p", label: "p", weak: false },
    Consonant { name: "t", label: "t", weak: false },
    Consonant { name: "ts", label: "ts", weak: false },
    Consonant { name: "k", label: "k", weak: false },
    // The glottal stop row: written as the bare vowel (a, e, i…). h, 7, ' and ʔ
    // are accepted as silent spellings of it, like the Spanish h.
    Consonant { name: "", label: "", weak: true },
    Consonant { name: "s", label: "s", weak: false },
    Consonant { name: "j", label: "j", weak: true },
    Consonant { name: "m", label: "m", weak: false },
    Consonant { name: "n", label: "n", weak: false },
    Consonant { name: "w", label: "w", weak: true },
    Consonant { name: "y", label: "y", weak: true },
    Consonant { name: "l", label: "l", weak: false },
    Consonant { name: "r", label: "r", weak: false },
];

/// Number of consonants the script actually had.
pub const NATIVE_CONSONANTS: usize = 11;

/// Vowels in chart order. `ü` is the high central vowel /ɨ/.
pub struct Vowel {
    pub name: &'static str,
    pub label: &'static str,
}

pub const VOWELS: [Vowel; 6] = [
    Vowel { name: "i", label: "i" },
    Vowel { name: "e", label: "e" },
    Vowel { name: "v", label: "ü" },
    Vowel { name: "a", label: "a" },
    Vowel { name: "u", label: "u" },
    Vowel { name: "o", label: "o" },
];

/// Which syllables have a known sign, row by row (i e ü a u o).
const KNOWN: [[bool; 6]; 13] = [
    [true, true, true, true, true, true],     // p
    [true, true, true, true, true, true],     // t
    [true, true, true, true, true, false],    // ts
    [true, true, true, false, true, true],    // k
    [true, false, true, true, true, true],    // bare vowels (glottal stop)
    [true, false, true, true, true, false],   // s
    [true, true, false, true, true, true],    // j
    [true, true, true, true, false, false],   // m
    [true, true, true, true, true, false],    // n
    [true, true, true, true, false, true],    // w
    [false, true, true, true, false, false],  // y
    [false; 6],                               // l (not in the language)
    [false; 6],                               // r (not in the language)
];

/// Syllables drawn with two forms in the chart; the second is the alternate
/// (`ja~`, or the `ss01` feature in the fonts).
pub const WITH_ALTERNATE: [(usize, usize); 6] = [(0, 3), (2, 3), (3, 2), (4, 0), (5, 0), (6, 3)];

pub fn known(c: usize, v: usize) -> bool {
    KNOWN[c][v]
}

pub fn has_alternate(c: usize, v: usize) -> bool {
    WITH_ALTERNATE.contains(&(c, v))
}

/// Glyph name of a syllable: `pa`, `tsi`, `ho`, `kv` (kü); `.alt` for the
/// second form.
pub fn syllable_name(c: usize, v: usize, alt: bool) -> String {
    format!("{}{}{}", CONSONANTS[c].name, VOWELS[v].name, if alt { ".alt" } else { "" })
}

/// Syllable as written in the practical orthography: `pa`, `ü` (glottal row).
pub fn syllable_label(c: usize, v: usize) -> String {
    format!("{}{}", CONSONANTS[c].label, VOWELS[v].label)
}

/// Every known syllable with its forms, in chart order.
pub fn syllables() -> Vec<(usize, usize, bool)> {
    let mut out = Vec::new();
    for c in 0..CONSONANTS.len() {
        for v in 0..VOWELS.len() {
            if known(c, v) {
                out.push((c, v, false));
                if has_alternate(c, v) {
                    out.push((c, v, true));
                }
            }
        }
    }
    out
}

/// Signs for whole closed syllables (consonant–vowel–consonant). Kaufman &
/// Justeson (2001, §1.2) recognize these four: kak, pak, pük and yaj.
pub struct Cvc {
    pub name: &'static str,
    pub label: &'static str,
    pub c1: usize,
    pub v: usize,
    pub c2: usize,
}

pub const CVC: [Cvc; 4] = [
    Cvc { name: "kak", label: "kak", c1: 3, v: 3, c2: 3 },
    Cvc { name: "pak", label: "pak", c1: 0, v: 3, c2: 3 },
    Cvc { name: "pvk", label: "pük", c1: 0, v: 2, c2: 3 },
    Cvc { name: "yaj", label: "yaj", c1: 10, v: 3, c2: 6 },
];

pub fn cvc_index(c1: usize, v: usize, c2: usize) -> Option<usize> {
    CVC.iter().position(|s| s.c1 == c1 && s.v == v && s.c2 == c2)
}

pub fn cvc_codepoint(index: usize) -> char {
    char::from_u32(0xE100 + index as u32).unwrap()
}

/// Private Use Area code point of each glyph, stable across versions so text
/// copied as code points keeps working: syllables from U+E000 (row × 16 +
/// vowel × 2 + alternate), closed syllables from U+E100, logograms from
/// U+E200, numerals 0–19 from U+E300.
pub fn syllable_codepoint(c: usize, v: usize, alt: bool) -> char {
    char::from_u32(0xE000 + (c as u32) * 16 + (v as u32) * 2 + alt as u32).unwrap()
}

pub fn logogram_codepoint(index: usize) -> char {
    char::from_u32(0xE200 + index as u32).unwrap()
}

pub fn numeral_codepoint(n: u32) -> char {
    char::from_u32(0xE300 + n).unwrap()
}

/// Glyph name of a bar-and-dot numeral.
pub fn numeral_name(n: u32) -> String {
    format!("num{n}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inventory_size() {
        let known = (0..11).flat_map(|c| (0..6).map(move |v| (c, v))).filter(|&(c, v)| known(c, v)).count();
        assert_eq!(known, 53);
        for &(c, v) in &WITH_ALTERNATE {
            assert!(super::known(c, v));
        }
    }

    #[test]
    fn names() {
        assert_eq!(syllable_name(2, 3, true), "tsa.alt");
        assert_eq!(syllable_label(4, 2), "ü");
    }
}
