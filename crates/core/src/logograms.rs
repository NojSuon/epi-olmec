//! Logograms (word signs) included in the fonts. Readings and glosses follow
//! Kaufman & Justeson (2001), §1.3 and §2.3; a `?` marks a reading they
//! give as tentative.

pub struct Logogram {
    /// ASCII glyph name, also the English key: `[sky]`.
    pub name: &'static str,
    /// Spanish key and gloss: `[cielo]`.
    pub es: &'static str,
    /// English gloss as printed by Kaufman & Justeson.
    pub en: &'static str,
    /// Reconstructed reading (h = glottal stop, ts = their tz).
    pub reading: &'static str,
}

pub const LOGOGRAMS: [Logogram; 12] = [
    Logogram { name: "sky", es: "cielo", en: "SKY", reading: "tsap" },
    Logogram { name: "turtle", es: "tortuga", en: "TURTLE", reading: "tuki" },
    Logogram { name: "bundle", es: "bulto", en: "BUNDLE", reading: "pit" },
    Logogram { name: "sprinkle", es: "rociar", en: "SPRINKLE", reading: "wik" },
    Logogram { name: "ten", es: "diez", en: "TEN", reading: "mak" },
    Logogram { name: "chop", es: "cortar", en: "CHOP", reading: "tsets" },
    Logogram { name: "longlip", es: "labio", en: "LONGLIP", reading: "jihts" },
    Logogram { name: "middle", es: "medio", en: "MIDDLE", reading: "kuk" },
    Logogram { name: "loincloth", es: "taparrabo", en: "LOINCLOTH?", reading: "jup" },
    Logogram { name: "plant", es: "sembrar", en: "PLANT", reading: "nihp" },
    Logogram { name: "dealwith", es: "tratar", en: "DEAL.WITH", reading: "tüp" },
    Logogram { name: "blood", es: "sangre", en: "BLOOD", reading: "nühpin" },
];

/// Finds a logogram by its English or Spanish key, ignoring case.
pub fn find(key: &str) -> Option<usize> {
    let key = key.trim().to_lowercase();
    LOGOGRAMS.iter().position(|l| l.name == key || l.es == key)
}
