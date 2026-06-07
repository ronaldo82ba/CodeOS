use std::path::Path;

use codesvc_pkg::{CappPackager, CodeOsManifest};

pub fn run(path: &str) {
    let root = Path::new(path);
    let manifest_path = root.join("codeos_manifest.toml");
    if !manifest_path.exists() {
        eprintln!("error: codeos_manifest.toml not found in {path}");
        std::process::exit(1);
    }

    let content = std::fs::read_to_string(&manifest_path).expect("read manifest");
    let manifest = CodeOsManifest::from_toml(&content).expect("parse manifest");
    manifest.validate().expect("validate manifest");

    let mut payload: Vec<(String, Vec<u8>)> = Vec::new();

    let binary_path = root.join(&manifest.entry.binary);
    if !binary_path.exists() {
        eprintln!(
            "error: entry binary not found: {}",
            binary_path.display()
        );
        std::process::exit(1);
    }
    payload.push((
        manifest.entry.binary.clone(),
        std::fs::read(&binary_path).expect("read entry binary"),
    ));

    if let Some(ui) = &manifest.ui {
        for relative in [&ui.main_view, &ui.icon] {
            let file_path = root.join(relative);
            if file_path.exists() {
                payload.push((
                    relative.clone(),
                    std::fs::read(&file_path).expect("read ui asset"),
                ));
            }
        }
    }

    for optional_dir in ["assets", "res"] {
        let dir = root.join(optional_dir);
        if !dir.is_dir() {
            continue;
        }
        collect_dir_files(&dir, root, &mut payload);
    }

    let output = root.join(format!("{}.capp", manifest.app.id.replace('.', "-")));
    let payload_refs: Vec<(&str, &[u8])> = payload
        .iter()
        .map(|(name, data)| (name.as_str(), data.as_slice()))
        .collect();

    CappPackager::pack(&manifest, &output, &payload_refs).expect("pack .capp");
    println!("Built {}", output.display());
}

fn collect_dir_files(dir: &Path, root: &Path, payload: &mut Vec<(String, Vec<u8>)>) {
    for entry in std::fs::read_dir(dir).expect("read dir") {
        let entry = entry.expect("dir entry");
        let path = entry.path();
        if path.is_dir() {
            collect_dir_files(&path, root, payload);
            continue;
        }
        let relative = path
            .strip_prefix(root)
            .expect("strip prefix")
            .to_string_lossy()
            .replace('\\', "/");
        if payload.iter().any(|(name, _)| name == &relative) {
            continue;
        }
        payload.push((relative, std::fs::read(path).expect("read file")));
    }
}
