use anyhow::Result;
use std::fs;
use std::path::Path;

const PACKAGE_JSON: &str = include_str!("../assets/plugin/package.json");
const MANIFEST_JSON: &str = include_str!("../assets/plugin/openclaw.plugin.json");
const INDEX_JS: &str = include_str!("../assets/plugin/index.js");
const README_MD: &str = include_str!("../assets/plugin/README.md");

pub fn plugin_asset_contents() -> [(&'static str, &'static str); 4] {
    [
        ("package.json", PACKAGE_JSON),
        ("openclaw.plugin.json", MANIFEST_JSON),
        ("index.js", INDEX_JS),
        ("README.md", README_MD),
    ]
}

pub fn write_plugin_assets(target_dir: &Path) -> Result<()> {
    fs::create_dir_all(target_dir)?;
    for (name, content) in plugin_asset_contents() {
        fs::write(target_dir.join(name), content)?;
    }
    Ok(())
}
