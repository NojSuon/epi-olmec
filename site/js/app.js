// Epi-Olmec keyboard: type in Latin letters, see the signs in columns,
// export a transparent PNG (or SVG). The spelling rules run in Rust
// (WebAssembly); the signs are drawn from the same outlines as the fonts.

import init, { spell } from "../pkg/epi_olmec_wasm.js?v={{v}}";

// ---------- Text in both languages ----------
const I18N = {
  es: {
    skip: "Ir al teclado",
    heroTag: "Silabario epi-olmeca · 150 a.C.–450 d.C.",
    heroTitle: "Escribe en epi-olmeca, de arriba abajo",
    heroText: "Teclea en letras latinas y mira cómo cada sílaba se vuelve un signo. Elige uno de los tres estilos, cambia el color y descarga tu texto como PNG sin fondo.",
    heroCta: "Empieza a escribir ↓",
    heroFonts: "Descarga las tipografías",
    kbTitle: "Teclado",
    kbLead: "Cada consonante con su vocal es un signo: pa, tsi, kü. Una línea nueva empieza otra columna.",
    textLabel: "Tu texto en letras latinas",
    textHelp: "Una vocal sola (<b>a</b>, <b>e</b>, <b>i</b>…) es su propio signo, el del saltillo (ʔ); dentro de una palabra escríbelo con <b>'</b> (<b>poy'a</b>). <b>ts</b> es una sola consonante y la <b>ü</b> también se escribe <b>v</b>. <b>[cielo]</b> pone un logograma, un número del 0 al 19 es un numeral de barras y puntos, y <b>~</b> después de una sílaba elige su forma alterna (<b>ja~</b>).",
    modeLabel: "Ortografía",
    modeAuto: "Reglas epi-olmecas",
    modeLiteral: "Letra por letra",
    modeHelpAuto: "Como en las inscripciones: una consonante sin vocal repite la vocal anterior (<b>tsap</b> → tsa·pa) y el saltillo (<b>'</b>), <b>j</b>, <b>w</b> y <b>y</b> sin vocal no se escriben.",
    modeHelpLiteral: "Cada sílaba tal como la escribes; una consonante sola aparece como ◌ con su letra.",
    styleSans: "Pal", styleSansSub: "lineal",
    styleJuego: "Juego", styleJuegoSub: "pincel",
    styleEstela: "Estela", styleEstelaSub: "formal",
    colorLabel: "Color de los signos",
    bgLabel: "Fondo",
    sizeLabel: "Tamaño",
    spacingLabel: "Espacio entre signos",
    columnLabel: "Espacio entre columnas",
    outlineLabel: "Contorno",
    rtlLabel: "Columnas de derecha a izquierda",
    labelsLabel: "Mostrar la lectura junto a cada signo",
    guidesLabel: "Cartuchos (marco de cada columna)",
    scaleLabel: "Resolución",
    downloadPng: "Descargar PNG",
    copyPng: "Copiar imagen",
    downloadSvg: "Descargar SVG",
    copyText: "Copiar texto (fuente)",
    fontsTitle: "Tres tipografías",
    fontsLead: "Los 59 signos silábicos, 4 signos de sílaba cerrada, 12 logogramas y los numerales del 0 al 19, vectorizados a partir de los dibujos originales y redibujados con una misma pluma para que todos compartan estilo.",
    howtoTitle: "Cómo escribir con la fuente en otros programas",
    howto1: "Instala el archivo <b>.ttf</b>. Las ligaduras convierten lo que tecleas: <b>pa</b>, <b>tsa</b>, <b>u</b>, <b>kv</b> o <b>kü</b>.",
    howto2: "Logogramas entre corchetes: <b>[sky]</b> o <b>[cielo]</b>. Numerales con almohadilla: <b>#13</b>.",
    howto3: "Para escribir en columnas usa texto vertical (en CSS: <code>writing-mode: vertical-lr; text-orientation: upright</code>). Cada signo ocupa su propia altura.",
    howto4: "El botón <b>Copiar texto (fuente)</b> copia los signos como caracteres de uso privado (U+E000…), que se ven bien con cualquiera de las tres fuentes aunque las ligaduras estén apagadas.",
    aboutTitle: "Sobre la escritura",
    about1Title: "Una escritura mixe-zoque",
    about1: "La escritura epi-olmeca o istmeña se usó en el sur de Veracruz y Chiapas entre el 150 a.C. y el 450 d.C. Su texto más largo es la estela de La Mojarra. Terrence Kaufman y John Justeson la leen como una forma antigua de lengua zoque.",
    about2Title: "Columnas de arriba abajo",
    about2: "Los signos se escriben en columnas que se leen de arriba abajo y de izquierda a derecha. Mezcla logogramas (palabras) con signos silábicos de consonante y vocal.",
    about3Title: "11 consonantes, 6 vocales",
    about3: "Consonantes <b>p t ts k s j m n w y</b> más el saltillo /ʔ/, cuya fila se escribe con la vocal sola, y vocales <b>i e ü a u o</b> (<b>ü</b> es una vocal central, /ɨ/). Hoy se conocen los signos de 53 de las 66 sílabas posibles; las que faltan se muestran como ◌.",
    about4Title: "Cómo se deletrea",
    about4: "Una consonante final se escribe con la sílaba que repite la vocal anterior (<b>tsap</b> “cielo” → tsa·pa). El saltillo, <b>j</b>, <b>w</b> y <b>y</b> no se escriben antes de otra consonante, y <b>k</b> no se escribe antes de <b>s</b>.",
    sources: "<strong>Fuentes.</strong> Terrence Kaufman y John Justeson, <i>Epi-Olmec Hieroglyphic Writing and Texts</i> (2001), figura 4 y §1–2. Tabla del silabario redibujada a partir de esa figura. Aquí su <b>7</b> (ʔ) se escribe con la vocal sola al inicio y con <b>'</b> dentro de la palabra, y su <b>tz</b> como <b>ts</b>. Las lecturas son las de los autores y algunas son tentativas.",
    ctaTitle: "¡Cuéntanos!",
    ctaText: "¿Qué escribiste en epi-olmeca? Compártelo y etiquétanos.",
    ctaButton: "Escríbenos en Instagram",
    // Built in JS
    padSyllables: "Sílabas",
    padAlternates: "Formas alternas",
    padLogograms: "Logogramas",
    padNumerals: "Numerales",
    space: "Espacio",
    newColumn: "Nueva columna",
    erase: "Borrar ⌫",
    clear: "Vaciar",
    missingNote: (l) => `Sin signo conocido: ${l}. Se muestran como ◌.`,
    otherNote: (l) => `No se escriben: ${l}.`,
    none: "Sin fondo",
    custom: "Otro color",
    downloaded: "Listo: imagen descargada.",
    copied: "Imagen copiada.",
    copiedText: "Texto copiado. Pégalo donde uses la fuente.",
    copyFail: "Tu navegador no deja copiar imágenes; usa Descargar PNG.",
    download: "Descargar",
    glyphs: "glifos",
    sansDesc: "Lineal y geométrica: solo trazos rectos y a 45°, esquinas achaflanadas y puntos cuadrados.",
    juegoDesc: "Hecha a pincel: trazo grueso que se hincha y se afina, con el pulso de la mano.",
    estelaDesc: "Verticales gruesas y horizontales finas con remate, como una Song (宋体).",
    home: "Inicio",
    keyboard: "Teclado",
    fonts: "Tipografías",
    script: "Escritura",
    donate: "Dónanos",
    footer: "Aprende, preserva y comparte lenguas indígenas.",
    follow: "Síguenos en",
    darkMode: "Modo oscuro",
    sample: "tsap\ntuki",
    helpTitle: "Cómo teclear",
    corpusTitle: "Corpus de palabras",
    corpusLead: "Palabras leídas por Kaufman y Justeson en las inscripciones, con su deletreo en signos. Toca «Escribir» para llevarlas al teclado.",
    corpusSearch: "Buscar (palabra, significado o deletreo)",
    corpusSource: "<strong>Fuente.</strong> Kaufman y Justeson (2001), “Lexical items”, pp. 2.91–2.97, y ejemplos de §1–2. Las palabras van con <b>ts</b> y <b>ü</b>, y el saltillo como vocal sola al inicio o <b>'</b> dentro; entre ⟨ ⟩ se deja su deletreo original (7 = ʔ, tz = ts, ʉ = ü). Solo se incluyen palabras que se pueden escribir con los signos de estas fuentes.",
    padCvc: "Sílabas cerradas (CVC)",
    cvcHelp: "Kaufman y Justeson reconocen cuatro signos de sílaba cerrada. Con las reglas activas se usan solos: <b>kak</b>, <b>pük</b>.",
    missingSyllables: "Sin signo conocido (◌): tso, ka, e, se, so, jü, mu, mo, no, wu, yi, yu, yo. Ninguna fuente que consultamos propone un signo para ellas.",
    reading: "lectura",
    shell: "concha",
    zeroNote: "El cero (concha) sigue la convención mesoamericana; no aparece en los textos epi-olmecas que se conservan.",
    useWord: "Escribir",
    noWords: "Ninguna palabra coincide.",
    corpusCount: (n, t) => (n === t ? `${t} palabras` : `${n} de ${t} palabras`),
  },
  en: {
    skip: "Skip to the keyboard",
    heroTag: "Epi-Olmec syllabary · 150 BCE–450 CE",
    heroTitle: "Write in Epi-Olmec, top to bottom",
    heroText: "Type in Latin letters and watch each syllable turn into a sign. Pick one of three styles, change the colour and download your text as a transparent PNG.",
    heroCta: "Start writing ↓",
    heroFonts: "Download the fonts",
    kbTitle: "Keyboard",
    kbLead: "Each consonant with its vowel is one sign: pa, tsi, kü. A new line starts a new column.",
    textLabel: "Your text in Latin letters",
    textHelp: "A vowel on its own (<b>a</b>, <b>e</b>, <b>i</b>…) is its own sign, the glottal-stop one (ʔ); inside a word type it as <b>'</b> (<b>poy'a</b>). <b>ts</b> is one consonant and <b>ü</b> can also be typed <b>v</b>. <b>[sky]</b> adds a logogram, a number from 0 to 19 is a bar-and-dot numeral, and <b>~</b> after a syllable picks its alternate form (<b>ja~</b>).",
    modeLabel: "Spelling",
    modeAuto: "Epi-Olmec rules",
    modeLiteral: "Letter by letter",
    modeHelpAuto: "As in the inscriptions: a consonant with no vowel repeats the previous vowel (<b>tsap</b> → tsa·pa), and the glottal stop (<b>'</b>), <b>j</b>, <b>w</b> and <b>y</b> with no vowel are not written.",
    modeHelpLiteral: "Every syllable just as you type it; a lone consonant shows as ◌ with its letter.",
    styleSans: "Pal", styleSansSub: "sans",
    styleJuego: "Juego", styleJuegoSub: "brush",
    styleEstela: "Estela", styleEstelaSub: "formal",
    colorLabel: "Sign colour",
    bgLabel: "Background",
    sizeLabel: "Size",
    spacingLabel: "Space between signs",
    columnLabel: "Space between columns",
    outlineLabel: "Outline",
    rtlLabel: "Columns right to left",
    labelsLabel: "Show the reading next to each sign",
    guidesLabel: "Cartouches (frame each column)",
    scaleLabel: "Resolution",
    downloadPng: "Download PNG",
    copyPng: "Copy image",
    downloadSvg: "Download SVG",
    copyText: "Copy text (font)",
    fontsTitle: "Three fonts",
    fontsLead: "All 59 syllabic signs, 4 closed-syllable signs, 12 logograms and the numerals 0 to 19, vectorized from the original drawings and redrawn with one pen so they all share a style.",
    howtoTitle: "Writing with the font in other apps",
    howto1: "Install the <b>.ttf</b> file. Ligatures turn what you type into signs: <b>pa</b>, <b>tsa</b>, <b>u</b>, <b>kv</b> or <b>kü</b>.",
    howto2: "Logograms in brackets: <b>[sky]</b> or <b>[cielo]</b>. Numerals with a hash: <b>#13</b>.",
    howto3: "To write in columns use vertical text (in CSS: <code>writing-mode: vertical-lr; text-orientation: upright</code>). Each sign takes its own height.",
    howto4: "<b>Copy text (font)</b> copies the signs as Private Use characters (U+E000…), which show correctly in any of the three fonts even with ligatures off.",
    aboutTitle: "About the script",
    about1Title: "A Mixe-Zoquean script",
    about1: "Epi-Olmec or Isthmian writing was used in southern Veracruz and Chiapas between 150 BCE and 450 CE. Its longest text is the La Mojarra stela. Terrence Kaufman and John Justeson read it as an early form of Zoquean.",
    about2Title: "Columns, top to bottom",
    about2: "Signs are written in columns read top to bottom and left to right. It mixes logograms (word signs) with consonant–vowel syllable signs.",
    about3Title: "11 consonants, 6 vowels",
    about3: "Consonants <b>p t ts k s j m n w y</b> plus the glottal stop /ʔ/, whose row is written with the bare vowel, and vowels <b>i e ü a u o</b> (<b>ü</b> is a central vowel, /ɨ/). Signs are known for 53 of the 66 possible syllables; missing ones show as ◌.",
    about4Title: "How words are spelled",
    about4: "A final consonant is written with the syllable that repeats the previous vowel (<b>tsap</b> ‘sky’ → tsa·pa). The glottal stop, <b>j</b>, <b>w</b> and <b>y</b> are not written before another consonant, and <b>k</b> is not written before <b>s</b>.",
    sources: "<strong>Sources.</strong> Terrence Kaufman and John Justeson, <i>Epi-Olmec Hieroglyphic Writing and Texts</i> (2001), figure 4 and §1–2. Syllabary chart redrawn from that figure. Here their <b>7</b> (ʔ) is the bare vowel at the start of a word and <b>'</b> inside it, and their <b>tz</b> is <b>ts</b>. Readings are the authors'; some are tentative.",
    ctaTitle: "Tell us!",
    ctaText: "What did you write in Epi-Olmec? Share it and tag us.",
    ctaButton: "Message us on Instagram",
    padSyllables: "Syllables",
    padAlternates: "Alternate forms",
    padLogograms: "Logograms",
    padNumerals: "Numerals",
    space: "Space",
    newColumn: "New column",
    erase: "Delete ⌫",
    clear: "Clear",
    missingNote: (l) => `No known sign: ${l}. Shown as ◌.`,
    otherNote: (l) => `Not written: ${l}.`,
    none: "No background",
    custom: "Other colour",
    downloaded: "Done: image downloaded.",
    copied: "Image copied.",
    copiedText: "Text copied. Paste it wherever you use the font.",
    copyFail: "Your browser can't copy images; use Download PNG.",
    download: "Download",
    glyphs: "glyphs",
    sansDesc: "Linear and geometric: only straight and 45° strokes, chamfered corners and square dots.",
    juegoDesc: "Brushed by hand: a heavy stroke that swells and tapers, with a hand's wobble.",
    estelaDesc: "Thick verticals, thin horizontals and small serifs, like a Song face (宋体).",
    home: "Home",
    keyboard: "Keyboard",
    fonts: "Fonts",
    script: "Script",
    donate: "Donate",
    footer: "Learn, preserve and share Indigenous languages.",
    follow: "Follow us at",
    darkMode: "Dark mode",
    sample: "tsap\ntuki",
    helpTitle: "How to type",
    corpusTitle: "Word corpus",
    corpusLead: "Words read by Kaufman and Justeson in the inscriptions, with their spelling in signs. Press “Write” to send one to the keyboard.",
    corpusSearch: "Search (word, meaning or spelling)",
    corpusSource: "<strong>Source.</strong> Kaufman and Justeson (2001), “Lexical items”, pp. 2.91–2.97, and examples from §1–2. Words are written with <b>ts</b> and <b>ü</b>, and the glottal stop as a bare vowel at the start or <b>'</b> inside; ⟨ ⟩ keeps their original spelling (7 = ʔ, tz = ts, ʉ = ü). Only words that can be written with the signs in these fonts are included.",
    padCvc: "Closed syllables (CVC)",
    cvcHelp: "Kaufman and Justeson recognize four closed-syllable signs. With the rules on they are used automatically: <b>kak</b>, <b>pük</b>.",
    missingSyllables: "No known sign (◌): tso, ka, e, se, so, jü, mu, mo, no, wu, yi, yu, yo. None of the sources we checked proposes a sign for them.",
    reading: "reading",
    shell: "shell",
    zeroNote: "Zero (shell) follows the Mesoamerican convention; it does not appear in the surviving Epi-Olmec texts.",
    useWord: "Write",
    noWords: "No word matches.",
    corpusCount: (n, t) => (n === t ? `${t} words` : `${n} of ${t} words`),
  },
};
let LANG = document.documentElement.lang === "en" ? "en" : "es";
const T = (k, ...a) => {
  const v = I18N[LANG][k] ?? I18N.es[k];
  return typeof v === "function" ? v(...a) : v;
};

function applyI18n() {
  document.documentElement.lang = LANG;
  document.querySelectorAll("[data-i18n]").forEach((el) => (el.textContent = T(el.dataset.i18n)));
  document.querySelectorAll("[data-i18n-html]").forEach((el) => (el.innerHTML = T(el.dataset.i18nHtml)));
  document.title = LANG === "en" ? "Mati Mati · Epi-Olmec writing" : "Mati Mati · Escritura epi-olmeca";
}

// ---------- Header and footer (Mati Mati) ----------
const LOGO_SVG = `
<svg viewBox="0 0 100 100" role="img" aria-label="Mati Mati" fill="currentColor">
  <path d="M0 46V3h12l11 15 11-15h12v43H35V22L23 36 11 22v24z"/>
  <path fill-rule="evenodd" d="M54 46V14C54 7 59 3 66 3h22c7 0 12 4 12 11v32H89V33H65v13zm11-23h24v-8c0-1-1-2-2-2H67c-1 0-2 1-2 2z"/>
  <path d="M0 54h46v11H28.5v35h-11V65H0z"/>
  <path d="M54 54h46v11H82.5v24H100v11H54V89h17.5V65H54z"/>
</svg>`;
const MOON = `<svg class="icon-moon" viewBox="0 0 24 24" aria-hidden="true" fill="currentColor"><path d="M20.5 14.2A8.5 8.5 0 0 1 9.8 3.5a8.5 8.5 0 1 0 10.7 10.7z"/></svg>`;
const SUN = `<svg class="icon-sun" viewBox="0 0 24 24" aria-hidden="true" fill="none" stroke="currentColor" stroke-width="2.2" stroke-linecap="round"><circle cx="12" cy="12" r="4.2" fill="currentColor" stroke="none"/><path d="M12 2.5v2.2M12 19.3v2.2M2.5 12h2.2M19.3 12h2.2M5.3 5.3l1.6 1.6M17.1 17.1l1.6 1.6M5.3 18.7l1.6-1.6M17.1 6.9l1.6-1.6"/></svg>`;
const DONATE_URL = "https://ko-fi.com/suondilut";

function renderChrome() {
  const dark = document.documentElement.dataset.theme === "dark";
  document.querySelector("[data-site-header]").innerHTML = `
    <div class="wrap">
      <a class="brand" href="./">${LOGO_SVG}<span>Mati Mati</span></a>
      <div class="header-end">
        <nav class="site-nav" aria-label="Principal"><ul>
          <li><a href="#teclado">${T("keyboard")}</a></li>
          <li><a href="#tipografias">${T("fonts")}</a></li>
          <li><a href="#escritura">${T("script")}</a></li>
        </ul></nav>
        <a class="donate" href="${DONATE_URL}" target="_blank" rel="noopener">${T("donate")}</a>
        <button type="button" class="theme-toggle" data-theme-toggle aria-pressed="${dark}" aria-label="${T("darkMode")}" title="${T("darkMode")}">${MOON}${SUN}</button>
        <span class="lang-switches"><button type="button" class="lang-switch" data-lang-toggle lang="${LANG === "es" ? "en" : "es"}">${LANG === "es" ? "EN" : "ES"}</button></span>
      </div>
    </div>`;
  document.querySelector("[data-site-footer]").innerHTML = `
    <div class="wrap">
      <a class="brand" href="./">${LOGO_SVG}<span>Mati Mati</span></a>
      <p>${T("footer")}<br>${T("follow")} <a href="https://www.instagram.com/matidemati" rel="noopener">@matidemati</a>.</p>
      <a class="donate donate--footer" href="${DONATE_URL}" target="_blank" rel="noopener">${T("donate")}</a>
    </div>`;
}

document.addEventListener("click", (ev) => {
  if (ev.target.closest?.("[data-theme-toggle]")) {
    const theme = document.documentElement.dataset.theme === "dark" ? "light" : "dark";
    document.documentElement.dataset.theme = theme;
    try { localStorage.setItem("theme", theme); } catch {}
    renderChrome();
    renderAll();
  }
  if (ev.target.closest?.("[data-lang-toggle]")) {
    LANG = LANG === "es" ? "en" : "es";
    try { localStorage.setItem("lang", LANG); } catch {}
    applyI18n();
    renderChrome();
    buildPad();
    buildSwatches();
    buildFonts();
    renderAll();
    buildCorpus();
  }
});

// ---------- Settings ----------
const PALETTE = [
  ["#2B1D1A", "Tinta"], ["#C1502E", "Terracota Mati"], ["#7A2E1D", "Cacao Rojo"], ["#9C5400", "Ocre"],
  ["#D9A93F", "Mostaza"], ["#44989D", "Turquesa Mar"], ["#2F6B73", "Petróleo"], ["#3A7A6D", "Jade"],
  ["#4A2F5B", "Berenjena"], ["#7A63A8", "Jacaranda"], ["#A898C9", "Glicina"], ["#9A2157", "Magenta Tinta"],
  ["#683547", "Ciruela"], ["#2E4A7D", "Añil"], ["#FFFFFF", "Blanco"],
];
const BACKGROUNDS = [
  [null, "none"], ["#FFF9F5", "Papel"], ["#E8E4F1", "Lila Suave"], ["#FFE7E0", "Rosa Crema"], ["#E4CCB3", "Arena Dorada"],
  ["#CDDFDD", "Espuma"], ["#4A2F5B", "Berenjena"], ["#2B1D1A", "Tinta"],
];
const DEFAULTS = { style: "sans", color: "#2B1D1A", bg: null, size: 120, spacing: 0, colgap: 40, outline: 0, rtl: false, labels: false, guides: false, mode: "auto", scale: "2" };
let state = { ...DEFAULTS };
try { Object.assign(state, JSON.parse(localStorage.getItem("epi-olmec") || "{}")); } catch {}
const save = () => { try { localStorage.setItem("epi-olmec", JSON.stringify({ ...state, text: $("#text").value })); } catch {} };
try { const s = JSON.parse(localStorage.getItem("epi-olmec") || "{}"); if (typeof s.text === "string") document.querySelector("#text").value = s.text; } catch {}

const $ = (s) => document.querySelector(s);
let DATA = null;
const pathCache = new Map();
function glyphPath(style, name) {
  const key = style + "/" + name;
  if (!pathCache.has(key)) {
    const g = DATA.styles[style].glyphs[name];
    pathCache.set(key, g ? new Path2D(g.d) : null);
  }
  return pathCache.get(key);
}
const glyph = (style, name) => DATA.styles[style].glyphs[name];

// What a sign reads as: the syllable (i, si), the logogram's gloss in
// capitals (as Kaufman & Justeson write them) or the number.
function readingOf(t) {
  if (t.k === "missing") return t.l;
  const syl = DATA.syllables.find((s) => s.name === t.g);
  if (syl) return syl.label;
  const cvc = DATA.cvc.find((c) => c.name === t.g);
  if (cvc) return cvc.label;
  const logo = DATA.logograms.find((l) => l.name === t.g);
  if (logo) return (LANG === "en" ? logo.en : logo.es).toUpperCase();
  if (t.g.startsWith("num")) return t.g.slice(3);
  return t.t;
}

// ---------- Layout ----------
// Everything is laid out in CSS pixels at scale 1; export multiplies.
function layout(tokens, opt) {
  const size = opt.size;
  const s = size / 1000;
  const pad = Math.round(size * 0.22);
  const gap = (opt.spacing / 400) * size;
  const labelW = opt.labels ? size * 0.5 : 0;
  const frame = opt.guides ? size * 0.12 : 0;
  const columns = [[]];
  for (const t of tokens) {
    if (t.k === "break") columns.push([]);
    else columns[columns.length - 1].push(t);
  }
  const items = [];
  const colW = size * 0.92 + labelW + frame * 2;
  let maxH = 0;
  const cols = columns.map((col) => {
    const out = [];
    let y = frame;
    let first = true;
    for (const t of col) {
      if (t.k === "space") {
        if (!first) y += size * 0.3;
        continue;
      }
      if (t.k === "sign" || t.k === "missing") {
        const name = t.k === "sign" ? t.g : "placeholder";
        const g = glyph(opt.style, name);
        if (!g) continue;
        if (!first) y += gap;
        out.push({ type: "glyph", name, y: y - g.vtop * s, label: readingOf(t), missing: t.k === "missing", mid: y + (g.vadv * s) / 2 });
        y += g.vadv * s;
        first = false;
      }
    }
    y += frame;
    maxH = Math.max(maxH, y);
    return { out, h: y };
  });
  const n = cols.length;
  const width = pad * 2 + n * colW + (n - 1) * opt.colgap;
  const height = pad * 2 + Math.max(maxH, size * 0.5);
  cols.forEach((c, i) => {
    const idx = opt.rtl ? n - 1 - i : i;
    const x0 = pad + idx * (colW + opt.colgap);
    if (opt.guides && c.out.length) items.push({ type: "frame", x: x0, y: pad, w: colW, h: c.h, r: size * 0.12 });
    const gx = x0 + frame - size * 0.04; // glyph box is 1000 units; signs are ~920 wide
    for (const it of c.out) {
      items.push({ type: "glyph", name: it.name, x: gx, y: pad + it.y, s });
      if (opt.labels || it.missing) {
        items.push({
          type: "label",
          text: it.label,
          x: opt.labels ? x0 + frame + size * 0.92 + 4 : gx + 500 * s,
          y: pad + it.mid,
          size: opt.labels ? Math.max(11, size * 0.16) : Math.max(9, size * 0.17),
          align: opt.labels ? "left" : "center",
        });
      }
    }
  });
  return { width: Math.ceil(width), height: Math.ceil(height), items };
}

function draw(ctx, lay, opt, scale) {
  ctx.save();
  ctx.scale(scale, scale);
  ctx.clearRect(0, 0, lay.width, lay.height);
  if (opt.bg) {
    ctx.fillStyle = opt.bg;
    ctx.fillRect(0, 0, lay.width, lay.height);
  }
  const outlineColor = opt.bg || (isDark(opt.color) ? "#FFFFFF" : "#2B1D1A");
  for (const it of lay.items) {
    if (it.type === "frame") {
      ctx.lineWidth = Math.max(1.5, opt.size * 0.025);
      ctx.strokeStyle = opt.color;
      ctx.beginPath();
      ctx.roundRect(it.x, it.y, it.w, it.h, it.r);
      ctx.stroke();
    }
  }
  for (const it of lay.items) {
    if (it.type !== "glyph") continue;
    const p = glyphPath(opt.style, it.name);
    if (!p) continue;
    ctx.save();
    ctx.translate(it.x, it.y);
    ctx.scale(it.s, it.s);
    if (opt.outline > 0) {
      ctx.lineJoin = "round";
      ctx.lineWidth = (opt.outline * 2) / it.s;
      ctx.strokeStyle = outlineColor;
      ctx.stroke(p);
    }
    ctx.fillStyle = opt.color;
    ctx.fill(p);
    ctx.restore();
  }
  for (const it of lay.items) {
    if (it.type !== "label") continue;
    ctx.font = `800 ${it.size}px "Nunito Sans", sans-serif`;
    ctx.textAlign = it.align;
    ctx.textBaseline = "middle";
    ctx.fillStyle = opt.color;
    ctx.fillText(it.text, it.x, it.y);
  }
  ctx.restore();
}

function isDark(hex) {
  const n = parseInt(hex.slice(1), 16);
  const [r, g, b] = [(n >> 16) & 255, (n >> 8) & 255, n & 255];
  return 0.299 * r + 0.587 * g + 0.114 * b < 140;
}

function toSvg(lay, opt) {
  const esc = (s) => String(s).replace(/[&<>"]/g, (c) => ({ "&": "&amp;", "<": "&lt;", ">": "&gt;", '"': "&quot;" })[c]);
  const parts = [`<svg xmlns="http://www.w3.org/2000/svg" width="${lay.width}" height="${lay.height}" viewBox="0 0 ${lay.width} ${lay.height}">`];
  if (opt.bg) parts.push(`<rect width="100%" height="100%" fill="${opt.bg}"/>`);
  const outlineColor = opt.bg || (isDark(opt.color) ? "#FFFFFF" : "#2B1D1A");
  for (const it of lay.items) {
    if (it.type === "frame") {
      parts.push(`<rect x="${it.x}" y="${it.y}" width="${it.w}" height="${it.h}" rx="${it.r}" fill="none" stroke="${opt.color}" stroke-width="${Math.max(1.5, opt.size * 0.025)}"/>`);
    } else if (it.type === "glyph") {
      const g = glyph(opt.style, it.name);
      const stroke = opt.outline > 0 ? ` stroke="${outlineColor}" stroke-width="${(opt.outline * 2) / it.s}" stroke-linejoin="round" paint-order="stroke"` : "";
      parts.push(`<path transform="translate(${it.x.toFixed(2)} ${it.y.toFixed(2)}) scale(${it.s})" d="${g.d}" fill="${opt.color}"${stroke}/>`);
    } else if (it.type === "label") {
      parts.push(`<text x="${it.x}" y="${it.y}" font-family="Nunito Sans, sans-serif" font-weight="800" font-size="${it.size}" text-anchor="${it.align === "left" ? "start" : "middle"}" dominant-baseline="middle" fill="${opt.color}">${esc(it.text)}</text>`);
    }
  }
  parts.push("</svg>");
  return parts.join("");
}

// ---------- Rendering ----------
let tokens = [];
function update() {
  tokens = JSON.parse(spell($("#text").value, state.mode === "auto"));
  const missing = [...new Set(tokens.filter((t) => t.k === "missing").map((t) => t.l))];
  const other = [...new Set(tokens.filter((t) => t.k === "other").map((t) => t.t))];
  const notes = [];
  if (missing.length) notes.push(T("missingNote", missing.join(", ")));
  if (other.length) notes.push(T("otherNote", other.join(" ")));
  $("#stage-note").textContent = notes.join(" ");
  renderAll();
  save();
}

function renderAll() {
  if (!DATA) return;
  const lay = layout(tokens, state);
  const c = $("#canvas");
  const dpr = Math.min(window.devicePixelRatio || 1, 2);
  c.width = lay.width * dpr;
  c.height = lay.height * dpr;
  c.style.width = lay.width + "px";
  draw(c.getContext("2d"), lay, state, dpr);
  heroSample();
}

function heroSample() {
  const c = $("#hero-canvas");
  if (!c || !DATA) return;
  const toks = JSON.parse(spell(T("sample"), true));
  const opt = { ...DEFAULTS, style: "juego", size: 120, color: "#4A2F5B", colgap: 18 };
  const lay = layout(toks, opt);
  c.width = lay.width * 2;
  c.height = lay.height * 2;
  draw(c.getContext("2d"), lay, opt, 2);
}

// ---------- Pad ----------
const C_LABELS = ["p", "t", "ts", "k", "–", "s", "j", "m", "n", "w", "y"];
const V_LABELS = ["i", "e", "ü", "a", "u", "o"];

function keySvg(name) {
  const g = glyph(state.style, name);
  return g ? `<svg viewBox="0 0 1000 1000" aria-hidden="true"><path d="${g.d}"/></svg>` : "";
}

// ---------- Folding sections ----------
// Every <details data-fold="id"> remembers whether it was open or closed.
function loadFolds() {
  try { return JSON.parse(localStorage.getItem("epi-olmec-folds") || "{}"); } catch { return {}; }
}
function applyFolds(root = document) {
  const folds = loadFolds();
  root.querySelectorAll("details[data-fold]").forEach((d) => {
    if (d.dataset.fold in folds) d.open = folds[d.dataset.fold];
  });
}
document.addEventListener("toggle", (ev) => {
  const d = ev.target;
  if (!(d instanceof HTMLDetailsElement) || !d.dataset.fold) return;
  const folds = loadFolds();
  folds[d.dataset.fold] = d.open;
  try { localStorage.setItem("epi-olmec-folds", JSON.stringify(folds)); } catch {}
}, true);
const fold = (id, title, body, open = true, cls = "") =>
  `<details class="fold ${cls}" data-fold="${id}"${open ? " open" : ""}><summary><span>${title}</span></summary>${body}</details>`;

function buildPad() {
  if (!DATA) return;
  const pad = $("#pad");
  const syl = new Map(DATA.syllables.map((s) => [s.name, s]));
  let html = `<div class="pad-table"><span></span>`;
  html += V_LABELS.map((v) => `<span class="pad-head">${v}</span>`).join("");
  C_LABELS.forEach((c, ci) => {
    html += `<span class="pad-head">${c}</span>`;
    V_LABELS.forEach((v, vi) => {
      const s = DATA.syllables.find((x) => x.c === ci && x.v === vi && !x.alt);
      if (!s) {
        html += `<span class="pad-empty" title="${c}${v}: —"></span>`;
        return;
      }
      const hasAlt = syl.has(s.name + ".alt");
      html += `<button type="button" class="pad-key" data-insert="${s.label}" title="${s.label}" aria-label="${s.label}">${keySvg(s.name)}<span class="k">${s.label}</span>${hasAlt ? '<span class="alt-dot" aria-hidden="true"></span>' : ""}</button>`;
    });
  });
  html += `</div><p class="caption pad-missing">${T("missingSyllables")}</p>`;
  let out = fold("pad-syllables", T("padSyllables"), html);

  let cvc = `<p class="caption">${T("cvcHelp")}</p><div class="pad-row">`;
  for (const s of DATA.cvc) {
    cvc += `<button type="button" class="pad-key" data-insert="[${s.label}]" title="${s.label}">${keySvg(s.name)}<span class="k">${s.label}</span></button>`;
  }
  out += fold("pad-cvc", T("padCvc"), cvc + `</div>`);

  let alts = `<div class="pad-row">`;
  for (const s of DATA.syllables.filter((x) => x.alt)) {
    alts += `<button type="button" class="pad-key" data-insert="${s.label}~" title="${s.label}~">${keySvg(s.name)}<span class="k">${s.label}~</span></button>`;
  }
  out += fold("pad-alternates", T("padAlternates"), alts + `</div>`, false);

  // Logograms with their proposed reading under the name.
  let logos = `<div class="pad-row pad-row--logos">`;
  for (const l of DATA.logograms) {
    const key = LANG === "en" ? l.name : l.es;
    logos += `<button type="button" class="pad-key pad-key--logo" data-insert="[${key}]" title="${l.en} · ${T("reading")}: ${l.reading}">${keySvg(l.name)}<span class="k">${key}<span class="r">${l.reading}</span></span></button>`;
  }
  out += fold("pad-logograms", T("padLogograms"), logos + `</div>`);

  // Numerals 0–19 in a 5 × 4 grid: each row adds a bar.
  let nums = `<div class="pad-grid5">`;
  for (let n = 0; n <= 19; n++) {
    const label = n === 0 ? `0 <small>${T("shell")}</small>` : n;
    nums += `<button type="button" class="pad-key" data-insert="${n} " title="${n}">${keySvg("num" + n)}<span class="k">${label}</span></button>`;
  }
  nums += `</div><p class="caption">${T("zeroNote")}</p>`;
  out += fold("pad-numerals", T("padNumerals"), nums);

  out += `<div class="pad-tools">
    <button type="button" data-insert=" ">${T("space")}</button>
    <button type="button" data-insert="\n">${T("newColumn")}</button>
    <button type="button" data-erase>${T("erase")}</button>
    <button type="button" data-clear>${T("clear")}</button>
  </div>`;
  pad.innerHTML = out;
  applyFolds(pad);
}

// ---------- Corpus ----------
let CORPUS = null;
function buildCorpus() {
  const box = $("#corpus-list");
  if (!box || !CORPUS || !DATA) return;
  const q = ($("#corpus-search")?.value || "").trim().toLowerCase();
  const words = CORPUS.words.filter((w) => !q || [w.w, w.es, w.en, w.kj].some((s) => s.toLowerCase().includes(q)));
  box.innerHTML = words.map((w, i) => `
    <li class="word">
      <canvas class="word-signs" data-i="${CORPUS.words.indexOf(w)}" aria-hidden="true"></canvas>
      <div class="word-text">
        <span class="word-form">${w.w}</span>
        <span class="word-gloss">${LANG === "en" ? w.en : w.es}</span>
        <span class="word-kj" title="Kaufman & Justeson">${w.kj.replace(/</g, "⟨").replace(/>/g, "⟩")} · p. ${w.p}</span>
      </div>
      <button type="button" class="word-use" data-word="${CORPUS.words.indexOf(w)}">${T("useWord")}</button>
    </li>`).join("") || `<li class="caption">${T("noWords")}</li>`;
  $("#corpus-count").textContent = T("corpusCount", words.length, CORPUS.words.length);
  box.querySelectorAll("canvas.word-signs").forEach((c) => {
    const w = CORPUS.words[Number(c.dataset.i)];
    const toks = JSON.parse(spell(w.in, true));
    const opt = { ...DEFAULTS, style: state.style, size: 46, color: getComputedStyle(document.body).getPropertyValue("--mati-tinta").trim() || "#2B1D1A", colgap: 6 };
    const lay = layout(toks, opt);
    c.width = lay.width * 2;
    c.height = lay.height * 2;
    c.style.width = lay.width + "px";
    draw(c.getContext("2d"), lay, opt, 2);
  });
}
document.addEventListener("click", (ev) => {
  const b = ev.target.closest("[data-word]");
  if (!b) return;
  const w = CORPUS.words[Number(b.dataset.word)];
  const ta = $("#text");
  ta.value = ta.value.trim() ? ta.value.replace(/\s+$/, "") + "\n" + w.in : w.in;
  update();
  $("#teclado").scrollIntoView({ behavior: "smooth" });
});

$("#pad").addEventListener("click", (ev) => {
  const b = ev.target.closest("button");
  if (!b) return;
  const ta = $("#text");
  const [a, z] = [ta.selectionStart, ta.selectionEnd];
  if (b.hasAttribute("data-clear")) {
    ta.value = "";
  } else if (b.hasAttribute("data-erase")) {
    if (a !== z) ta.setRangeText("", a, z, "end");
    else if (a > 0) ta.setRangeText("", a - 1, a, "end");
  } else {
    ta.setRangeText(b.dataset.insert, a, z, "end");
  }
  update();
});

// ---------- Controls ----------
function setPressed(sel, pred) {
  document.querySelectorAll(sel).forEach((b) => b.setAttribute("aria-pressed", String(pred(b))));
}

function buildSwatches() {
  const sw = $("#swatches");
  sw.innerHTML =
    PALETTE.map(([hex, name]) => `<button type="button" class="swatch" data-color="${hex}" style="background:${hex}" title="${name}" aria-label="${name}"></button>`).join("") +
    `<span class="swatch-custom" title="${T("custom")}"><input type="color" id="color-custom" aria-label="${T("custom")}" value="${state.color}"></span>`;
  const bg = $("#bg-swatches");
  bg.innerHTML =
    BACKGROUNDS.map(([hex, name]) =>
      hex
        ? `<button type="button" class="swatch" data-bg="${hex}" style="background:${hex}" title="${name}" aria-label="${name}"></button>`
        : `<button type="button" class="swatch swatch--none" data-bg="" title="${T("none")}" aria-label="${T("none")}"></button>`
    ).join("") +
    `<span class="swatch-custom" title="${T("custom")}"><input type="color" id="bg-custom" aria-label="${T("custom")}" value="${state.bg || "#FFF9F5"}"></span>`;
  syncControls();
}

function syncControls() {
  setPressed("#style-seg button", (b) => b.dataset.style === state.style);
  setPressed("#swatches .swatch", (b) => b.dataset.color.toLowerCase() === state.color.toLowerCase());
  setPressed("#bg-swatches .swatch", (b) => (b.dataset.bg || null) === state.bg);
  for (const id of ["size", "spacing", "colgap", "outline"]) $("#" + id).value = state[id];
  $("#rtl").checked = state.rtl;
  $("#labels").checked = state.labels;
  $("#guides").checked = state.guides;
  $("#scale").value = state.scale;
  document.querySelector(`input[name="mode"][value="${state.mode}"]`).checked = true;
  const help = $("#mode-help");
  help.dataset.i18nHtml = state.mode === "auto" ? "modeHelpAuto" : "modeHelpLiteral";
  help.innerHTML = T(help.dataset.i18nHtml);
}

document.addEventListener("click", (ev) => {
  const st = ev.target.closest("#style-seg button");
  if (st) {
    state.style = st.dataset.style;
    syncControls();
    buildPad();
    update();
    buildCorpus();
  }
  const sw = ev.target.closest("[data-color]");
  if (sw) {
    state.color = sw.dataset.color;
    syncControls();
    update();
  }
  const bg = ev.target.closest("[data-bg]");
  if (bg) {
    state.bg = bg.dataset.bg || null;
    syncControls();
    update();
  }
});
document.addEventListener("input", (ev) => {
  const t = ev.target;
  if (t.id === "text") return update();
  if (t.id === "color-custom") { state.color = t.value; syncControls(); return update(); }
  if (t.id === "bg-custom") { state.bg = t.value; syncControls(); return update(); }
  if (["size", "spacing", "colgap", "outline"].includes(t.id)) { state[t.id] = Number(t.value); return update(); }
  if (["rtl", "labels", "guides"].includes(t.id)) { state[t.id] = t.checked; return update(); }
  if (t.name === "mode") { state.mode = t.value; syncControls(); return update(); }
  if (t.id === "scale") { state.scale = t.value; save(); }
});

// ---------- Export ----------
function exportCanvas() {
  const lay = layout(tokens, state);
  const scale = Number(state.scale) || 2;
  const c = document.createElement("canvas");
  c.width = Math.ceil(lay.width * scale);
  c.height = Math.ceil(lay.height * scale);
  draw(c.getContext("2d"), lay, state, scale);
  return c;
}
function fileName(ext) {
  const words = $("#text").value.trim().split(/\s+/).slice(0, 3).join("-").replace(/[^\p{L}\p{N}-]/gu, "");
  return `epi-olmec-${words || "texto"}.${ext}`;
}
function downloadBlob(blob, name) {
  const a = document.createElement("a");
  a.href = URL.createObjectURL(blob);
  a.download = name;
  a.click();
  setTimeout(() => URL.revokeObjectURL(a.href), 1000);
}
const status = (msg) => ($("#export-status").textContent = msg);

$("#download-png").addEventListener("click", () => {
  exportCanvas().toBlob((b) => { downloadBlob(b, fileName("png")); status(T("downloaded")); }, "image/png");
});
$("#copy-png").addEventListener("click", async () => {
  try {
    const blob = await new Promise((r) => exportCanvas().toBlob(r, "image/png"));
    await navigator.clipboard.write([new ClipboardItem({ "image/png": blob })]);
    status(T("copied"));
  } catch {
    status(T("copyFail"));
  }
});
$("#download-svg").addEventListener("click", () => {
  const svg = toSvg(layout(tokens, state), state);
  downloadBlob(new Blob([svg], { type: "image/svg+xml" }), fileName("svg"));
  status(T("downloaded"));
});
$("#copy-text").addEventListener("click", async () => {
  const cp = new Map([...DATA.syllables.map((s) => [s.name, s.cp]), ...DATA.logograms.map((l) => [l.name, l.cp]), ...DATA.cvc.map((c) => [c.name, c.cp])]);
  const out = tokens.map((t) => {
    if (t.k === "sign") {
      if (t.g.startsWith("num")) return String.fromCodePoint(0xe300 + Number(t.g.slice(3)));
      return String.fromCodePoint(parseInt(cp.get(t.g), 16));
    }
    if (t.k === "missing") return "◌";
    if (t.k === "space") return " ";
    if (t.k === "break") return "\n";
    return "";
  }).join("");
  try {
    await navigator.clipboard.writeText(out);
    status(T("copiedText"));
  } catch {
    status(T("copyFail"));
  }
});

// ---------- Fonts ----------
const FONT_FILES = { sans: "EpiOlmec-Sans.ttf", juego: "EpiOlmec-Juego.ttf", estela: "EpiOlmec-Estela.ttf" };
function buildFonts() {
  const grid = $("#font-grid");
  grid.innerHTML = Object.entries(FONT_FILES).map(([id, file]) => {
    const fam = DATA.styles[id].family;
    return `<article class="font-card">
      <h3>${fam}</h3>
      <p class="caption">${T(id + "Desc")}</p>
      <div class="sample" style="font-family:'${fam}'" lang="und">tsapa tuki<br>pa ki #13 [sky]</div>
      <a class="button" href="fonts/${file}" download>${T("download")} .ttf</a>
    </article>`;
  }).join("");
}

async function loadFonts() {
  for (const [id, file] of Object.entries(FONT_FILES)) {
    const face = new FontFace(DATA.styles[id].family, `url(fonts/${file}?v={{v}})`);
    try { document.fonts.add(await face.load()); } catch (e) { console.warn("font", file, e); }
  }
}

// ---------- Start ----------
applyI18n();
renderChrome();
await init();
DATA = await (await fetch("data/glyphs.json?v={{v}}")).json();
try { CORPUS = await (await fetch("data/corpus.json?v={{v}}")).json(); } catch (e) { console.warn("corpus", e); }
buildPad();
buildSwatches();
buildFonts();
update();
buildCorpus();
applyFolds();
loadFonts();
document.addEventListener("input", (ev) => { if (ev.target.id === "corpus-search") buildCorpus(); });
