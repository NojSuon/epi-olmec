//! WebAssembly bindings: the browser keyboard calls `spell` on every
//! keystroke and draws the signs it returns.

use epi_olmec_core::{spell as spell_text, syllable_name, Mode, Token, LOGOGRAMS};
use wasm_bindgen::prelude::*;

fn esc(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

/// Spells `input` and returns a JSON array of tokens:
/// `{"k":"sign","g":"pa","t":"pa"}`, `{"k":"missing","l":"ka","t":"ka"}`,
/// `{"k":"silent","t":"y"}`, `{"k":"space"}`, `{"k":"break"}`,
/// `{"k":"other","t":"?"}`. `auto` applies the spelling rules.
#[wasm_bindgen]
pub fn spell(input: &str, auto: bool) -> String {
    let mode = if auto { Mode::Auto } else { Mode::Literal };
    let items: Vec<String> = spell_text(input, mode)
        .into_iter()
        .map(|t| match t {
            Token::Syllable { c, v, alt, text, echo } => format!(
                r#"{{"k":"sign","g":{},"t":{},"echo":{}}}"#,
                esc(&syllable_name(c, v, alt)),
                esc(&text),
                echo
            ),
            Token::Logogram { index, text } => {
                format!(r#"{{"k":"sign","g":{},"t":{},"logo":true}}"#, esc(LOGOGRAMS[index].name), esc(&text))
            }
            Token::Numeral { n, text } => format!(r#"{{"k":"sign","g":"num{n}","t":{}}}"#, esc(&text)),
            Token::Missing { label, text } => format!(r#"{{"k":"missing","l":{},"t":{}}}"#, esc(&label), esc(&text)),
            Token::Silent { text } => format!(r#"{{"k":"silent","t":{}}}"#, esc(&text)),
            Token::Space => r#"{"k":"space"}"#.to_string(),
            Token::Break => r#"{"k":"break"}"#.to_string(),
            Token::Other { text } => format!(r#"{{"k":"other","t":{}}}"#, esc(&text)),
        })
        .collect();
    format!("[{}]", items.join(","))
}
