use anyhow::Result;
use libmpv::Mpv;
use once_cell::sync::Lazy;
use std::sync::Mutex;

static MPV: Lazy<Mutex<Option<Mpv>>> = Lazy::new(|| Mutex::new(None));

pub fn play(url: &str, title: &str) -> Result<()> {
    let mut guard = MPV.lock().unwrap();

    if guard.is_none() {
        let m = Mpv::new().map_err(|e| anyhow::anyhow!("mpv init failed: {:?}", e))?;
        m.set_property("vo", "gpu").ok();
        m.set_property("hwdec", "auto").ok();
        m.set_property("cache", "yes").ok();
        m.set_property("demuxer-max-bytes", "50MiB").ok();
        m.set_property("cache-secs", 10i64).ok();
        m.set_property("force-window", "yes").ok();
        m.set_property("keep-open", "yes").ok();
        m.set_property("osd-level", 1i64).ok();
        *guard = Some(m);
    }

    let mpv = guard.as_ref().unwrap();
    mpv.command("loadfile", &[url, "replace"])
        .map_err(|e| anyhow::anyhow!("loadfile failed: {:?}", e))?;
    mpv.set_property("title", title).ok();

    Ok(())
}

pub fn stop() -> Result<()> {
    let guard = MPV.lock().unwrap();
    if let Some(ref mpv) = *guard {
        mpv.command("stop", &[]).ok();
    }
    Ok(())
}
