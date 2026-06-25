use anyhow::{bail, Result};
use libmpv::{Mpv, MpvNode};
use once_cell::sync::Lazy;
use std::sync::Mutex;

/// Single global mpv instance (libmpv is not thread-safe for multiple instances)
static MPV: Lazy<Mutex<Option<Mpv>>> = Lazy::new(|| Mutex::new(None));

pub fn play(url: &str, title: &str) -> Result<()> {
    let mut guard = MPV.lock().unwrap();

    // Create or reuse the mpv instance
    let mpv = match guard.as_ref() {
        Some(_) => guard.as_ref().unwrap(),
        None => {
            let m = Mpv::new().map_err(|e| anyhow::anyhow!("mpv init failed: {:?}", e))?;

            // Core options
            m.set_property("vo", "gpu").ok();           // GPU rendering
            m.set_property("hwdec", "auto").ok();       // hardware decode
            m.set_property("cache", "yes").ok();
            m.set_property("demuxer-max-bytes", "50MiB").ok();
            m.set_property("cache-secs", 10i64).ok();

            // Window
            m.set_property("force-window", "yes").ok();
            m.set_property("keep-open", "yes").ok();    // don't close on EOF

            // OSD
            m.set_property("osd-level", 1i64).ok();

            *guard = Some(m);
            guard.as_ref().unwrap()
        }
    };

    // Load the URL
    mpv.command("loadfile", &[url, "replace"])
        .map_err(|e| anyhow::anyhow!("loadfile failed: {:?}", e))?;

    // Set window title
    mpv.set_property("title", title).ok();

    Ok(())
}

pub fn stop() -> Result<()> {
    let mut guard = MPV.lock().unwrap();
    if let Some(ref mpv) = *guard {
        mpv.command("stop", &[]).ok();
    }
    Ok(())
}
