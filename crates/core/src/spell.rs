//! Turns Latin text into Epi-Olmec signs.
//!
//! Typing works like a Japanese kana keyboard: each consonant + vowel becomes
//! one syllabic sign (`pa`, `tsi`, `kü`/`kv`), and a vowel on its own is the
//! sign of the glottal-stop row (`a`, `u`). Inside a word the glottal stop is
//! marked `'` (`poy'a`); `h`, Kaufman & Justeson's `7` and `ʔ` work too, and
//! their `tz` is written `ts`. On top of that the `Auto` mode applies the
//! spelling rules they describe (2001, §2):
//!
//! * a strong consonant (p t ts k s m n) with no vowel after it is spelled
//!   with the sign whose vowel repeats the previous vowel ("synharmony"):
//!   `tsap` → tsa·pa;
//! * a weak consonant (glottal stop, j, w, y) with no vowel after it is not
//!   written: `poy'a` → po·a;
//! * k, and probably p, are not written right before s: `nuks` → nu·si.
//!
//! `Literal` mode writes exactly what is typed and shows a lone consonant as
//! a placeholder.
//!
//! Other input: `[sky]` or `[cielo]` for a logogram, `[kak]` for a closed
//! syllable, numbers 0–19 as
//! bar-and-dot numerals, `~` after a syllable for its alternate form, a new
//! line for a new column.

use crate::{cvc_index, has_alternate, known, logograms, syllable_label, CONSONANTS, CVC};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    Auto,
    Literal,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Token {
    /// A syllabic sign. `text` is what was typed for it; `echo` is true when
    /// the vowel was added by the spelling rules.
    Syllable { c: usize, v: usize, alt: bool, text: String, echo: bool },
    /// A syllable whose sign is not known yet; shown as a placeholder.
    Missing { label: String, text: String },
    /// A closed-syllable sign (index into `CVC`).
    Cvc { index: usize, text: String },
    /// Typed but not written, by the spelling rules (weak consonants…).
    Silent { text: String },
    Logogram { index: usize, text: String },
    Numeral { n: u32, text: String },
    Space,
    /// New column.
    Break,
    /// Anything else (punctuation, letters outside the language).
    Other { text: String },
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Unit {
    C(usize),
    V(usize),
    Alt,
    Bad(char),
}

/// Reads one word into consonants and vowels. Returns the units and the
/// text each one came from.
fn units(word: &[char]) -> Vec<(Unit, String)> {
    let mut out = Vec::new();
    let mut i = 0;
    while i < word.len() {
        let ch = word[i];
        let next = word.get(i + 1).copied();
        // ts (or the older tz) before t
        if ch == 't' && matches!(next, Some('s') | Some('z')) {
            out.push((Unit::C(2), format!("t{}", next.unwrap())));
            i += 2;
            continue;
        }
        // u + combining diaeresis
        if ch == 'u' && next == Some('\u{308}') {
            out.push((Unit::V(2), "ü".into()));
            i += 2;
            continue;
        }
        let unit = match ch {
            'p' => Unit::C(0),
            't' => Unit::C(1),
            'k' => Unit::C(3),
            // Glottal stop: h, with 7 ' ʔ as older spellings.
            'h' | '7' | '\'' | 'ʔ' | '’' | 'ʼ' => Unit::C(4),
            's' => Unit::C(5),
            'j' => Unit::C(6),
            'm' => Unit::C(7),
            'n' => Unit::C(8),
            'w' => Unit::C(9),
            'y' => Unit::C(10),
            'i' => Unit::V(0),
            'e' => Unit::V(1),
            'ü' | 'v' | 'ʉ' | 'ɨ' => Unit::V(2),
            'a' => Unit::V(3),
            'u' => Unit::V(4),
            'o' => Unit::V(5),
            '~' => Unit::Alt,
            other => Unit::Bad(other),
        };
        out.push((unit, ch.to_string()));
        i += 1;
    }
    out
}

fn syllable(c: usize, v: usize, text: String, echo: bool) -> Token {
    if known(c, v) {
        Token::Syllable { c, v, alt: false, text, echo }
    } else {
        Token::Missing { label: syllable_label(c, v), text }
    }
}

fn spell_word(word: &[char], mode: Mode, out: &mut Vec<Token>) {
    let us = units(word);
    let mut last_vowel: Option<usize> = None;
    let mut i = 0;
    while i < us.len() {
        let (unit, text) = (&us[i].0, us[i].1.clone());
        match *unit {
            Unit::Alt => {
                if let Some(Token::Syllable { c, v, alt, text: t, .. }) = out.last_mut() {
                    if has_alternate(*c, *v) {
                        *alt = !*alt;
                        t.push('~');
                    }
                }
                i += 1;
            }
            Unit::Bad(ch) => {
                out.push(Token::Other { text: ch.to_string() });
                i += 1;
            }
            Unit::C(c) => {
                // A closed syllable with its own sign (kak, pak, pük, yaj):
                // consonant + vowel + consonant not followed by a vowel.
                if let (Some((Unit::V(v), vt)), Some((Unit::C(c2), ct))) = (us.get(i + 1), us.get(i + 2)) {
                    let closed = !matches!(us.get(i + 3), Some((Unit::V(_), _)));
                    if let (true, Some(index)) = (closed, cvc_index(c, *v, *c2)) {
                        out.push(Token::Cvc { index, text: format!("{text}{vt}{ct}") });
                        last_vowel = Some(*v);
                        i += 3;
                        continue;
                    }
                }
                if let Some((Unit::V(v), vt)) = us.get(i + 1) {
                    out.push(syllable(c, *v, format!("{text}{vt}"), false));
                    last_vowel = Some(*v);
                    i += 2;
                    continue;
                }
                // A consonant with no vowel after it.
                i += 1;
                if mode == Mode::Literal {
                    out.push(Token::Missing { label: CONSONANTS[c].label.into(), text });
                    continue;
                }
                let before_s = matches!(us.get(i), Some((Unit::C(5), _)));
                if CONSONANTS[c].weak || ((c == 3 || c == 0) && before_s) {
                    out.push(Token::Silent { text });
                    continue;
                }
                // Synharmony: repeat the previous vowel, else the next one.
                let echo = last_vowel.or_else(|| {
                    us[i..].iter().find_map(|(u, _)| if let Unit::V(v) = u { Some(*v) } else { None })
                });
                match echo {
                    Some(v) => out.push(syllable(c, v, text, true)),
                    None => out.push(Token::Missing { label: CONSONANTS[c].label.into(), text }),
                }
            }
            Unit::V(v) => {
                // A vowel with no consonant before it is its own sign (the
                // glottal-stop row of the chart: a, e, i…).
                out.push(syllable(4, v, text, false));
                last_vowel = Some(v);
                i += 1;
            }
        }
    }
}

fn is_word_char(ch: char) -> bool {
    ch.is_alphabetic() || matches!(ch, '7' | '\'' | '’' | 'ʼ' | 'ʔ' | '~' | '\u{308}')
}

/// Spells a whole text.
pub fn spell(input: &str, mode: Mode) -> Vec<Token> {
    let chars: Vec<char> = input.chars().flat_map(|c| c.to_lowercase()).collect();
    let mut out = Vec::new();
    let mut i = 0;
    while i < chars.len() {
        let ch = chars[i];
        if ch == '\n' {
            out.push(Token::Break);
            i += 1;
        } else if ch.is_whitespace() {
            if !matches!(out.last(), Some(Token::Space) | Some(Token::Break) | None) {
                out.push(Token::Space);
            }
            i += 1;
        } else if ch == '[' {
            let end = chars[i..].iter().position(|&c| c == ']').map(|p| p + i);
            match end {
                Some(end) => {
                    let key: String = chars[i + 1..end].iter().collect();
                    let text: String = chars[i..=end].iter().collect();
                    // [sky] / [cielo] for a logogram, [kak] for a closed-syllable sign.
                    let k = key.trim().replace('v', "ü");
                    if let Some(index) = logograms::find(&key) {
                        out.push(Token::Logogram { index, text });
                    } else if let Some(index) = CVC.iter().position(|s| s.label == k) {
                        out.push(Token::Cvc { index, text });
                    } else {
                        out.push(Token::Other { text });
                    }
                    i = end + 1;
                }
                None => {
                    out.push(Token::Other { text: "[".into() });
                    i += 1;
                }
            }
        } else if ch.is_ascii_digit() && !(ch == '7' && chars.get(i + 1).is_some_and(|c| c.is_alphabetic())) {
            let start = i;
            while i < chars.len() && chars[i].is_ascii_digit() {
                i += 1;
            }
            let text: String = chars[start..i].iter().collect();
            match text.parse::<u32>() {
                Ok(n) if (0..=19).contains(&n) => out.push(Token::Numeral { n, text }),
                _ => out.push(Token::Other { text }),
            }
        } else if is_word_char(ch) {
            let start = i;
            while i < chars.len() && is_word_char(chars[i]) {
                i += 1;
            }
            spell_word(&chars[start..i], mode, &mut out);
        } else {
            out.push(Token::Other { text: ch.to_string() });
            i += 1;
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn names(input: &str, mode: Mode) -> Vec<String> {
        spell(input, mode)
            .into_iter()
            .filter_map(|t| match t {
                Token::Syllable { c, v, alt, .. } => Some(crate::syllable_name(c, v, alt)),
                Token::Missing { label, .. } => Some(format!("?{label}")),
                Token::Logogram { index, .. } => Some(crate::LOGOGRAMS[index].name.to_string()),
                Token::Numeral { n, .. } => Some(format!("num{n}")),
                Token::Space => Some("_".into()),
                Token::Break => Some("|".into()),
                _ => None,
            })
            .collect()
    }

    #[test]
    fn open_syllables() {
        assert_eq!(names("pa tsi kü u hu 7u tzi", Mode::Auto), ["pa", "_", "tsi", "_", "kv", "_", "u", "_", "u", "_", "u", "_", "tsi"]);
        assert_eq!(names("kvo", Mode::Auto), ["kv", "o"]);
    }

    #[test]
    fn spelling_rules() {
        // tsap 'sky' → tsa-pa
        assert_eq!(names("tsap", Mode::Auto), ["tsa", "pa"]);
        // poy'a 'moon' (K&J poy7a) → po-a (weak y is not written); h works too
        assert_eq!(names("poyha", Mode::Auto), ["po", "a"]);
        assert_eq!(names("poy'a", Mode::Auto), ["po", "a"]);
        // nuks → nu-si (k before s)
        assert_eq!(names("nuksi", Mode::Auto), ["nu", "si"]);
        // literal mode keeps the lone consonant as a placeholder
        assert_eq!(names("tsap", Mode::Literal), ["tsa", "?p"]);
    }

    #[test]
    fn alternates_logograms_numbers() {
        assert_eq!(names("ja~", Mode::Auto), ["ja.alt"]);
        assert_eq!(names("[sky] [Cielo]", Mode::Auto), ["sky", "_", "sky"]);
        assert_eq!(names("13 pa\nmi", Mode::Auto), ["num13", "_", "pa", "|", "mi"]);
        assert_eq!(names("ka", Mode::Auto), ["?ka"]);
    }
}

#[cfg(test)]
mod more_tests {
    use super::*;

    fn kinds(input: &str) -> Vec<String> {
        spell(input, Mode::Auto)
            .into_iter()
            .filter_map(|t| match t {
                Token::Syllable { c, v, alt, .. } => Some(crate::syllable_name(c, v, alt)),
                Token::Cvc { index, .. } => Some(format!("={}", CVC[index].name)),
                Token::Numeral { n, .. } => Some(format!("num{n}")),
                Token::Missing { label, .. } => Some(format!("?{label}")),
                _ => None,
            })
            .collect()
    }

    #[test]
    fn closed_syllables() {
        // kak 'to replace' is one sign; kakpeh 'scorpion' starts with it.
        assert_eq!(kinds("kak"), ["=kak"]);
        assert_eq!(kinds("kakpeh"), ["=kak", "pe"]);
        // pakih 'hard' is pa-ki, not pak + i.
        assert_eq!(kinds("pakih"), ["pa", "ki"]);
        assert_eq!(kinds("pük yaj [pak]"), ["=pvk", "=yaj", "=pak"]);
    }

    #[test]
    fn zero() {
        assert_eq!(kinds("0 19"), ["num0", "num19"]);
    }
}

#[cfg(test)]
mod corpus_tests {
    use super::*;

    /// Every word in the site's corpus must spell with known signs only.
    #[test]
    fn corpus_spells_cleanly() {
        let json = include_str!("../../../site/data/corpus.json");
        let mut n = 0;
        for line in json.lines() {
            let Some(start) = line.find("\"in\": \"") else { continue };
            let rest = &line[start + 7..];
            let input = &rest[..rest.find('"').unwrap()];
            for t in spell(input, Mode::Auto) {
                assert!(
                    !matches!(t, Token::Missing { .. } | Token::Other { .. }),
                    "corpus entry {input:?} gives {t:?}"
                );
            }
            n += 1;
        }
        assert!(n > 60);
    }
}
