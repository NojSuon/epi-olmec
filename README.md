# Mati Mati · Epi-Olmec writing

Fonts and an online keyboard for the Epi-Olmec (Isthmian) script, built with the Mati Mati brand.

- **Keyboard:** type in Latin letters, see the signs in vertical columns, change colours and spacing, and download a transparent PNG (or SVG).
- **Three fonts**, all drawn from the same tracings:

| Font | Style |
| --- | --- |
| **Epi-Olmec Pal** | Even line weight with square corners, like a sans serif (黑体). |
| **Epi-Olmec Juego** | Heavy, round line with soft ends, friendly like the brand's Fredoka. |
| **Epi-Olmec Estela** | Thick verticals and thin horizontals with small serifs, like a Song typeface (宋体). |

Each font has the 53 known syllabic signs, 6 alternate forms, 12 logograms and the bar-and-dot numerals 1–19.

![Pal syllabary](previews/sans-syllabary.png)

## How it works

Everything is Rust:

- **`crates/core`**: the syllabary inventory and the spelling rules (Latin → signs).
- **`crates/forge`**: vectorizes the signs and writes the fonts.
  1. It cuts the syllabary chart (`sources/syllabary-chart.png`) into its 11 × 6 cells.
  2. For each sign it traces the centre lines (skeleton), solid black areas, dots and small circles, then straightens near-horizontal and near-vertical lines so boxes come out crisp.
  3. It redraws those parts with each style's pen: an even line (Pal), a round pen (Juego) or a flat wide pen that makes horizontals thin and verticals thick (Estela).
  4. It writes TrueType fonts with vertical metrics (each sign takes its own height in a column) and ligatures.

  Logograms come from `sources/logograms/*.png`, cropped from Kaufman & Justeson (2001).
- **`crates/wasm`**: the spelling rules compiled to WebAssembly for the keyboard.
- **`crates/site`**: builds the static site into `dist/` and can serve it locally.
- **`site/`**: HTML, CSS (Mati Mati design tokens) and JavaScript.

Outputs: `fonts/*.ttf`, `site/data/glyphs.json` (the same outlines, for the keyboard) and `previews/*.png` (to check the tracing by eye).

## Typing

| You type | You get |
| --- | --- |
| `pa`, `tsi`, `ko` | one syllabic sign each |
| `ha` | ha: **h** is the glottal stop /ʔ/ (`7a`, `'a`, `ʔa` also work) |
| `tsa` | tsa: **ts** is one consonant (`tza` also works) |
| `kv`, `kü` | kü (ü is the central vowel /ɨ/) |
| `ja~` | the alternate form of ja (pa, tsa, kü, hi, si and ja have one) |
| `[sky]`, `[cielo]` | a logogram (English or Spanish name) |
| `13` on the site, `#13` in the fonts | a bar-and-dot numeral |
| a new line | a new column |

With **Epi-Olmec rules** turned on (the default), the keyboard spells words the way the inscriptions do (Kaufman & Justeson §2):

- a consonant with no vowel is written with the sign that repeats the previous vowel: `tsap` → tsa·pa;
- h, j, w and y with no vowel are not written: `poyha` → po·ha;
- k (and probably p) are not written before s.

Syllables with no known sign show as ◌.

In other apps, install a `.ttf`. Ligatures turn the Latin letters into signs. For columns, use vertical text (CSS: `writing-mode: vertical-lr; text-orientation: upright`). Every sign also has a Private Use Area code point (syllables from U+E000, logograms from U+E200, numerals from U+E300). The keyboard's **Copy text (font)** button copies those.

## Run it locally

You need Rust, plus these two tools (one-time setup):

```bash
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli --version 0.2.100 --locked
```

Then:

```bash
cargo run -p epi-olmec-forge --release          # fonts, glyph data, previews
cargo run -p epi-olmec-site --release -- --serve # site at http://127.0.0.1:3000/
cargo test --workspace --exclude epi-olmec-wasm
```

## Publishing

Every push to `main` rebuilds the fonts and the site and deploys them to GitHub Pages (`.github/workflows/pages.yml`).

One-time setup: Settings → Pages → Source: "GitHub Actions".

## Sources

- Terrence Kaufman and John Justeson, *Epi-Olmec Hieroglyphic Writing and Texts* (2001): the syllabary (figure 4), the spelling rules (§2) and the logograms (§1.3, §2.3). Readings are theirs and some are tentative. This project writes their 7 (ʔ) as **h** and their tz as **ts**.
- `sources/syllabary-chart.png`: a redrawing of that syllabary chart.
