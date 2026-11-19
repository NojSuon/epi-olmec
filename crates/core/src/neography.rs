//! Neographies: modern proposals for syllables that have no known
//! Epi-Olmec sign. They are NOT attested in any inscription.
//!
//! Two groups are covered:
//!
//! * the 13 syllables of the language whose sign has not been identified
//!   (tso, ka, e, se, so, jü, mu, mo, no, wu, yi, yu, yo);
//! * the l and r rows (la, le… ra, re…), sounds that Epi-Olmec and its
//!   Mixe-Zoquean relatives did not have, for writing modern names and
//!   loanwords.
//!
//! Every neography follows one rule, so they read as a system:
//!
//! 1. **Consonant**: a real sign of the same row (the "donor"), shrunk to
//!    the top of the block. For l the donor is a y sign (Mixe-Zoquean turned
//!    an older l into y); for r it is a t sign.
//! 2. **Vowel**: a mark under it, one per vowel, built from shapes the
//!    script already uses: i `|`, e `||`, ü `○`, a `—`, u `∪`, o `◇`.
//! 3. **Foreign sound** (l and r only): a vertical stroke on the left,
//!    straight for l and zigzag for r.

use crate::{known, CONSONANTS, VOWELS};

/// How the left side of a neography is marked.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Side {
    None,
    /// l: a straight vertical stroke.
    Line,
    /// r: a zigzag vertical stroke.
    Zigzag,
}

/// The real sign (consonant row, vowel) that lends its shape to a row's
/// neographies, and the side mark. Donors are the simplest sign of each row.
pub fn donor(c: usize) -> ((usize, usize), Side) {
    match c {
        0 => ((0, 2), Side::None),   // p  ← pü
        1 => ((1, 5), Side::None),   // t  ← to
        2 => ((2, 1), Side::None),   // ts ← tse
        3 => ((3, 4), Side::None),   // k  ← ku
        4 => ((4, 0), Side::None),   // bare vowel ← i
        5 => ((5, 4), Side::None),   // s  ← su
        6 => ((6, 5), Side::None),   // j  ← jo
        7 => ((7, 2), Side::None),   // m  ← mü
        8 => ((8, 3), Side::None),   // n  ← na
        9 => ((9, 1), Side::None),   // w  ← we
        10 => ((10, 2), Side::None), // y  ← yü
        11 => ((10, 2), Side::Line), // l  ← yü + straight stroke
        12 => ((1, 5), Side::Zigzag), // r ← to + zigzag stroke
        _ => unreachable!(),
    }
}

/// True when (c, v) has no attested sign and therefore gets a neography.
pub fn exists(c: usize, v: usize) -> bool {
    c < CONSONANTS.len() && v < VOWELS.len() && !known(c, v)
}

/// Every neography, in table order.
pub fn all() -> Vec<(usize, usize)> {
    (0..CONSONANTS.len()).flat_map(|c| (0..VOWELS.len()).map(move |v| (c, v))).filter(|&(c, v)| exists(c, v)).collect()
}

/// Glyph name: `neo.ka`, `neo.e` (bare vowel), `neo.la`.
pub fn name(c: usize, v: usize) -> String {
    format!("neo.{}{}", CONSONANTS[c].name, VOWELS[v].name)
}

/// Private Use Area code point: U+E400 + row × 8 + vowel.
pub fn codepoint(c: usize, v: usize) -> char {
    char::from_u32(0xE400 + (c as u32) * 8 + v as u32).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inventory() {
        // 13 gaps in the language + 6 for l + 6 for r.
        assert_eq!(all().len(), 25);
        assert!(exists(3, 3)); // ka
        assert!(exists(4, 1)); // e
        assert!(!exists(0, 3)); // pa is attested
        for c in 0..CONSONANTS.len() {
            let ((dc, dv), _) = donor(c);
            assert!(known(dc, dv), "donor of row {c} must be a real sign");
        }
        assert_eq!(name(3, 3), "neo.ka");
        assert_eq!(name(4, 1), "neo.e");
    }
}
