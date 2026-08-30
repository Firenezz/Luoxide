use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use luoxide::prelude::*;
use sha2::{Digest, Sha256};

use super::common::{project_root, CheckStatus};
use crate::Result;

const DEFAULT_VERSION: &str = "5.5.1";

/// Official tarballs from https://www.lua.org/tests/ (`lua-<ver>-tests.tar.gz`).
const PINS: &[(&str, &str)] = &[
    (
        "5.5.1",
        "da07b543872dc0bb2ff12aabd0c248578d78df3eb6b67efdc537a46d455c7f31",
    ),
    (
        "5.5.0",
        "5e47bbfad7db2965d69580e918ee64edeb8d8d32de404b8dae9ce5c6d76a1472",
    ),
    (
        "5.4.8",
        "9581d5a7c39ffbf29b8ccde2709083c380f7bbddbd968dcb15712d2f2e33f4e5",
    ),
];

struct Pin {
    version: String,
    url: String,
    tarball: String,
    sha256: &'static str,
}

impl Pin {
    fn resolve(version: &str) -> Result<Self> {
        let sha256 = PINS
            .iter()
            .find(|(v, _)| *v == version)
            .map(|(_, hash)| *hash)
            .ok_or_else(|| {
                let known: Vec<&str> = PINS.iter().map(|(v, _)| *v).collect();
                format!(
                    "unknown lua test suite {version}; known: {}",
                    known.join(", ")
                )
            })?;
        let tarball = format!("lua-{version}-tests.tar.gz");
        Ok(Self {
            url: format!("https://www.lua.org/tests/{tarball}"),
            tarball,
            version: version.to_owned(),
            sha256,
        })
    }
}

pub fn run(args: &[String]) -> Result<()> {
    let version = match args {
        [] => DEFAULT_VERSION,
        [v] if !v.starts_with('-') => v.as_str(),
        _ => {
            return Err(format!("usage: cargo xtask lua-suite [{DEFAULT_VERSION}]").into());
        }
    };
    let pin = Pin::resolve(version)?;
    let root = project_root();
    let testes = ensure_suite(&root, &pin)?;
    let artifact_dir = root.join("target/lua-suite").join(&pin.version);
    fs::create_dir_all(&artifact_dir)?;
    eprintln!("artifacts {}", artifact_dir.display());

    let mut files = collect_lua_files(&testes)?;
    files.sort();

    let mut catalog = String::new();
    writeln!(
        catalog,
        "version {}\nurl {}\nsha256 {}\n",
        pin.version, pin.url, pin.sha256
    )?;

    let mut passed = 0u32;
    let mut partial = 0u32;
    let mut failed = 0u32;

    eprintln!("running {} tests", files.len());
    for path in &files {
        let rel = path.strip_prefix(&testes)?.to_path_buf();
        let bytes = fs::read(path)?;
        let (source, lossy) = decode_source(&bytes);
        let mut session = Session::new();
        let outcome = session.parse_chunk(&source);
        let (status, mut body) = artifact_text(&session, &source, &outcome);
        if lossy {
            body.insert_str(0, "utf8-lossy\n");
        }
        let name = rel.display();
        match status {
            "ok" => {
                passed += 1;
                eprintln!("test {name} ... ok");
            }
            "partial" => {
                partial += 1;
                eprintln!("test {name} ... FAILED (partial)");
            }
            _ => {
                failed += 1;
                eprintln!("test {name} ... FAILED");
            }
        }

        let artifact_rel = rel.with_extension("lua.artifact");
        let dest = artifact_dir.join(&artifact_rel);
        if let Some(parent) = dest.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&dest, body.as_bytes())?;

        let digest = hex_sha256(body.as_bytes());
        writeln!(catalog, "{status}\t{digest}\t{}", artifact_rel.display())?;
    }

    let result = if failed == 0 && partial == 0 {
        "ok"
    } else {
        "FAILED"
    };
    eprintln!();
    eprintln!(
        "test result: {result}. {passed} passed; {partial} partial; {failed} failed; 0 ignored; 0 measured; 0 filtered out"
    );

    let snap_name = format!("lua-{}", pin.version);
    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path(root.join("crates/luoxide-parser/tests/lua-suite"));
    settings.set_prepend_module_to_snapshot(false);
    settings.bind(|| {
        insta::assert_snapshot!(snap_name, catalog.as_str());
    });

    Ok(())
}

fn ensure_suite(root: &Path, pin: &Pin) -> Result<PathBuf> {
    let cache = root.join("target/lua-tests");
    let dest = cache.join(&pin.version);
    if let Some(suite) = suite_root(&dest) {
        return Ok(suite);
    }

    fs::create_dir_all(&cache)?;
    let tarball = cache.join(&pin.tarball);
    if !tarball.is_file() {
        download(&tarball, &pin.url)?;
    }
    verify_sha256(&tarball, pin.sha256)?;

    fs::create_dir_all(&dest)?;
    Command::new("tar")
        .args(["-xzf"])
        .arg(&tarball)
        .arg("-C")
        .arg(&dest)
        .args(["--strip-components=1"])
        .status()?
        .check()?;

    suite_root(&dest)
        .ok_or_else(|| format!("tarball missing all.lua under {}", dest.display()).into())
}

fn suite_root(dest: &Path) -> Option<PathBuf> {
    if dest.join("all.lua").is_file() {
        Some(dest.to_path_buf())
    } else if dest.join("testes/all.lua").is_file() {
        Some(dest.join("testes"))
    } else {
        None
    }
}

fn download(dest: &Path, url: &str) -> Result<()> {
    eprintln!("downloading {url}");
    let status = Command::new("curl")
        .args([
            "-fL",
            "--retry",
            "3",
            "-A",
            "luoxidant-xtask/0.1 (https://github.com/Firenezz/Luoxidant)",
            "-o",
        ])
        .arg(dest)
        .arg(url)
        .status();
    match status {
        Ok(s) if s.success() => Ok(()),
        Ok(s) => Err(format!("curl exited {}", s.code().unwrap_or(-1)).into()),
        Err(_) => {
            Command::new("wget")
                .args(["-O"])
                .arg(dest)
                .arg(url)
                .status()?
                .check()?;
            Ok(())
        }
    }
}

fn verify_sha256(path: &Path, expected: &str) -> Result<()> {
    let bytes = fs::read(path)?;
    let got = hex_sha256(&bytes);
    if got != expected {
        return Err(format!(
            "sha256 mismatch for {}: got {got}, want {expected}",
            path.display()
        )
        .into());
    }
    Ok(())
}

fn hex_sha256(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    let mut out = String::with_capacity(64);
    for byte in digest {
        let _ = write!(out, "{byte:02x}");
    }
    out
}

fn decode_source(bytes: &[u8]) -> (String, bool) {
    match std::str::from_utf8(bytes) {
        Ok(s) => (s.to_owned(), false),
        Err(_) => (String::from_utf8_lossy(bytes).into_owned(), true),
    }
}

fn collect_lua_files(dir: &Path) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    collect_lua_files_inner(dir, &mut files)?;
    Ok(files)
}

fn collect_lua_files_inner(dir: &Path, files: &mut Vec<PathBuf>) -> Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            collect_lua_files_inner(&path, files)?;
        } else if path.extension().and_then(|e| e.to_str()) == Some("lua") {
            files.push(path);
        }
    }
    Ok(())
}

fn artifact_text(
    session: &Session,
    source: &str,
    outcome: &Outcome<luoxide::prelude::Chunk, Vec<ParseError>>,
) -> (&'static str, String) {
    let mut out = String::new();
    match outcome {
        Outcome::Ok(chunk) => {
            let _ = writeln!(out, "ok");
            let _ = write!(out, "{}", session.display(chunk, source));
            if !out.ends_with('\n') {
                out.push('\n');
            }
            ("ok", out)
        }
        Outcome::PartialFailure(chunk, errors) => {
            let _ = writeln!(out, "partial");
            let _ = write!(out, "{}", session.display(chunk, source));
            write_errors(&mut out, errors, source);
            ("partial", out)
        }
        Outcome::TotalFailure(errors) => {
            let _ = writeln!(out, "total failure");
            write_errors(&mut out, errors, source);
            ("total-failure", out)
        }
    }
}

fn write_errors(out: &mut String, errors: &[ParseError], source: &str) {
    let _ = writeln!(out, "\n--- errors ({}) ---", errors.len());
    for (i, error) in errors.iter().enumerate() {
        let (title, notes) = error.details();
        let _ = write!(out, "[{i}] {title}");
        if let Some(span) = error.at {
            let range = span.start.to_usize()..span.end.to_usize();
            let lexeme = source.get(range).unwrap_or("");
            let _ = write!(out, " at {span} {lexeme:?}");
        }
        let _ = writeln!(out);
        for note in notes {
            let _ = writeln!(out, "    {note}");
        }
    }
}
