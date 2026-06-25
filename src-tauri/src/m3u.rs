use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Channel {
    pub name:  String,
    pub url:   String,
    pub group: String,
    pub logo:  String,
}

pub fn m3u_path() -> Result<PathBuf> {
    let dir = dirs::config_dir()
        .context("cannot determine config dir")?
        .join("mytv-desktop");
    std::fs::create_dir_all(&dir)?;
    Ok(dir.join("iptv_sources.m3u8"))
}

pub fn parse_m3u(path: &PathBuf) -> Result<Vec<Channel>> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("cannot read {:?}", path))?;

    let mut channels = vec![];
    let mut name  = String::new();
    let mut group = String::new();
    let mut logo  = String::new();

    for line in content.lines() {
        let l = line.trim();
        if l.starts_with("#EXTINF") {
            name  = extract_attr(l, "tvg-name")
                .or_else(|| l.rfind(',').map(|i| l[i+1..].trim().to_string()))
                .unwrap_or_default();
            group = extract_attr(l, "group-title").unwrap_or_else(|| "其他频道".into());
            logo  = extract_attr(l, "tvg-logo").unwrap_or_default();
        } else if !l.is_empty() && !l.starts_with('#') && !name.is_empty() {
            channels.push(Channel {
                name: name.clone(),
                url: l.to_string(),
                group: group.clone(),
                logo: logo.clone(),
            });
            name.clear();
        }
    }
    Ok(channels)
}

fn extract_attr(line: &str, attr: &str) -> Option<String> {
    let key   = format!("{}=\"", attr);
    let start = line.find(&key)? + key.len();
    let end   = line[start..].find('"')? + start;
    Some(line[start..end].to_string())
}
