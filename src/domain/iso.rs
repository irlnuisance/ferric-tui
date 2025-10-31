use crate::domain::{
    paths::{DirPath, IsoPath},
    units::ByteSize,
};
use std::{collections::HashSet, fs, path::Path, time::SystemTime};

#[derive(Debug, Clone)]
pub struct IsoMeta {
    pub path: IsoPath,
    pub size: ByteSize,
    pub modified: Option<SystemTime>,
}

pub fn scan_default_roots(query: &str) -> Vec<IsoMeta> {
    let mut roots: Vec<DirPath> = Vec::new();
    if let Ok(cwd) = std::env::current_dir() {
        roots.push(DirPath::from(cwd));
    }
    if let Some(home) = home_dir() {
        roots.push(home.join("Downloads"));
        roots.push(home);
    }
    let roots = normalize_roots(roots);
    scan(roots, query, 5, 100 * 1024 * 1024)
}

pub fn scan(roots: Vec<DirPath>, query: &str, max_depth: usize, min_size: u64) -> Vec<IsoMeta> {
    let mut out = Vec::new();
    let mut seen: HashSet<IsoPath> = HashSet::new();
    for root in roots {
        if root.exists() {
            walk(
                root.as_path(),
                0,
                max_depth,
                min_size,
                query,
                &mut out,
                &mut seen,
            );
        }
    }
    out.sort_by(|a, b| {
        b.modified
            .cmp(&a.modified)
            .then_with(|| a.path.cmp(&b.path))
    });
    out
}

fn walk(
    path: &Path,
    depth: usize,
    max_depth: usize,
    min_size: u64,
    query: &str,
    out: &mut Vec<IsoMeta>,
    seen: &mut HashSet<IsoPath>,
) {
    if depth > max_depth {
        return;
    }
    let entries = match fs::read_dir(path) {
        Ok(rd) => rd,
        Err(_) => return,
    };
    for entry in entries.flatten() {
        let path = entry.path();
        let file_name = entry.file_name();
        let file_name = file_name.to_string_lossy();
        if file_name.starts_with('.') && path.is_dir() {
            continue;
        }
        let file_type = match entry.file_type() {
            Ok(ft) => ft,
            Err(_) => continue,
        };
        if file_type.is_symlink() {
            continue;
        }
        if file_type.is_dir() {
            walk(&path, depth + 1, max_depth, min_size, query, out, seen);
        } else if file_type.is_file() {
            if !has_iso_extension(&path) {
                continue;
            }
            if !matches_filter(&file_name, query) {
                continue;
            }
            match fs::metadata(&path) {
                Ok(meta) => {
                    let size = meta.len();
                    if size < min_size {
                        continue;
                    }
                    let modified = meta.modified().ok();
                    let iso_path = IsoPath::from(path.clone());
                    if seen.insert(iso_path.clone()) {
                        out.push(IsoMeta {
                            path: iso_path,
                            size: ByteSize::new(size),
                            modified,
                        });
                    }
                }
                Err(_) => continue,
            }
        }
    }
}

fn has_iso_extension(p: &Path) -> bool {
    match p
        .extension()
        .and_then(|e| e.to_str())
        .map(|s| s.to_ascii_lowercase())
    {
        Some(ext) if matches!(ext.as_str(), "iso" | "img" | "raw") => true,
        _ => false,
    }
}

fn matches_filter(name: &str, query: &str) -> bool {
    if query.is_empty() {
        return true;
    }
    let name = name.to_ascii_lowercase();
    let q = query.to_ascii_lowercase();
    name.contains(&q)
}

fn home_dir() -> Option<DirPath> {
    std::env::var("HOME")
        .ok()
        .map(|s| DirPath::from(Path::new(&s).to_owned()))
}

fn normalize_roots(roots: Vec<DirPath>) -> Vec<DirPath> {
    let home = home_dir();
    let home_can = home.as_ref().and_then(|h| h.canonicalize().ok());
    let downloads_can = home
        .as_ref()
        .map(|h| h.join("Downloads"))
        .and_then(|d| d.canonicalize().ok());

    let mut out: Vec<(DirPath, DirPath)> = Vec::new();
    let mut seen_canon: HashSet<DirPath> = HashSet::new();
    for r in roots {
        let can = r.canonicalize().unwrap_or_else(|_| r.clone());
        if seen_canon.insert(can.clone()) {
            out.push((r, can));
        }
    }

    let has_downloads = downloads_can
        .as_ref()
        .map(|d| out.iter().any(|(_o, c)| c == d))
        .unwrap_or(false);
    let home_index = home_can
        .as_ref()
        .and_then(|h| out.iter().position(|(_o, c)| c == h));

    if has_downloads {
        if let Some(idx) = home_index {
            out.remove(idx);
        }
    }

    out.into_iter().map(|(o, _c)| o).collect()
}
