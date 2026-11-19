//! Builds the static site into `dist/` and, with `--serve`, serves it at
//! http://127.0.0.1:3000/.
//!
//! ```text
//! cargo run -p epi-olmec-site                # build
//! cargo run -p epi-olmec-site -- --serve     # build and serve
//! ```
//!
//! Run `cargo run -p epi-olmec-forge --release` first when the glyphs change:
//! it writes `fonts/` and `site/data/glyphs.json`, which are copied here.
//! All links in the site are relative, so it works under any path
//! (GitHub Pages serves it from /epi-olmec/).

use std::{
    collections::hash_map::DefaultHasher,
    fs,
    hash::{Hash, Hasher},
    path::{Path, PathBuf},
    process::Command,
};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let serve_it = args.iter().any(|a| a == "--serve");
    let no_wasm = args.iter().any(|a| a == "--no-wasm");

    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..").canonicalize().unwrap();
    let out = root.join("dist");
    if let Err(e) = build(&root, &out, !no_wasm) {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
    println!("site built in {}", out.display());
    if serve_it {
        let port = args.iter().position(|a| a == "--port").and_then(|i| args.get(i + 1)).and_then(|p| p.parse().ok()).unwrap_or(3000);
        serve(&out, port);
    }
}

fn build(root: &Path, out: &Path, wasm: bool) -> Result<(), String> {
    if out.exists() {
        // Keep pkg/ when the WebAssembly is not rebuilt.
        for entry in fs::read_dir(out).map_err(|e| e.to_string())? {
            let p = entry.map_err(|e| e.to_string())?.path();
            if !wasm && p.ends_with("pkg") {
                continue;
            }
            if p.is_dir() { fs::remove_dir_all(&p) } else { fs::remove_file(&p) }.map_err(|e| e.to_string())?;
        }
    }
    fs::create_dir_all(out).map_err(|e| e.to_string())?;
    copy_dir(&root.join("site"), out)?;
    copy_dir(&root.join("fonts"), &out.join("fonts"))?;
    if wasm {
        build_wasm(root, out)?;
    }

    // Cache busting: every asset link gets ?v=<hash of the built files>.
    let mut hasher = DefaultHasher::new();
    for f in walk(out) {
        if f.extension().is_some_and(|e| e != "html") {
            fs::read(&f).unwrap_or_default().hash(&mut hasher);
        }
    }
    let version = format!("{:x}", hasher.finish() & 0xffff_ffff);
    for f in walk(out) {
        if f.extension().is_some_and(|e| e == "html" || e == "js" || e == "css") {
            let text = fs::read_to_string(&f).map_err(|e| e.to_string())?;
            if text.contains("{{v}}") {
                fs::write(&f, text.replace("{{v}}", &version)).map_err(|e| e.to_string())?;
            }
        }
    }
    Ok(())
}

fn walk(dir: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    if let Ok(rd) = fs::read_dir(dir) {
        for e in rd.flatten() {
            let p = e.path();
            if p.is_dir() {
                out.extend(walk(&p));
            } else {
                out.push(p);
            }
        }
    }
    out
}

fn copy_dir(from: &Path, to: &Path) -> Result<(), String> {
    fs::create_dir_all(to).map_err(|e| e.to_string())?;
    for entry in fs::read_dir(from).map_err(|e| format!("{}: {e}", from.display()))? {
        let p = entry.map_err(|e| e.to_string())?.path();
        let dest = to.join(p.file_name().unwrap());
        if p.is_dir() {
            copy_dir(&p, &dest)?;
        } else {
            fs::copy(&p, &dest).map_err(|e| format!("{}: {e}", p.display()))?;
        }
    }
    Ok(())
}

fn build_wasm(root: &Path, out: &Path) -> Result<(), String> {
    let status = Command::new("cargo")
        .current_dir(root)
        .args(["build", "-p", "epi-olmec-wasm", "--release", "--target", "wasm32-unknown-unknown"])
        .status()
        .map_err(|e| e.to_string())?;
    if !status.success() {
        return Err("building the WebAssembly failed. Run: rustup target add wasm32-unknown-unknown".into());
    }
    let wasm = root.join("target/wasm32-unknown-unknown/release/epi_olmec_wasm.wasm");
    let status = Command::new("wasm-bindgen")
        .args(["--target", "web", "--no-typescript", "--out-dir"])
        .arg(out.join("pkg"))
        .arg(&wasm)
        .status()
        .map_err(|_| "wasm-bindgen was not found. Install it with: cargo install wasm-bindgen-cli --version 0.2.100 --locked".to_string())?;
    if !status.success() {
        return Err("wasm-bindgen failed".into());
    }
    Ok(())
}

fn content_type(path: &Path) -> &'static str {
    match path.extension().and_then(|e| e.to_str()).unwrap_or("") {
        "html" => "text/html; charset=utf-8",
        "css" => "text/css; charset=utf-8",
        "js" => "text/javascript; charset=utf-8",
        "json" => "application/json",
        "wasm" => "application/wasm",
        "svg" => "image/svg+xml",
        "png" => "image/png",
        "ttf" => "font/ttf",
        _ => "application/octet-stream",
    }
}

fn serve(dir: &Path, port: u16) {
    let server = tiny_http::Server::http(("127.0.0.1", port)).expect("could not bind port");
    println!("Epi-Olmec keyboard → http://127.0.0.1:{port}/");
    for request in server.incoming_requests() {
        let url = request.url().split(['?', '#']).next().unwrap_or("/").to_string();
        let safe = !url.split('/').any(|seg| seg == "..");
        let mut path = dir.join(url.trim_start_matches('/'));
        if path.is_dir() {
            path = path.join("index.html");
        }
        let (status, file) = if safe && path.is_file() { (200, path) } else { (404, dir.join("index.html")) };
        let body = fs::read(&file).unwrap_or_default();
        let header = tiny_http::Header::from_bytes("Content-Type", content_type(&file)).unwrap();
        let _ = request.respond(tiny_http::Response::from_data(body).with_status_code(status).with_header(header));
    }
}
