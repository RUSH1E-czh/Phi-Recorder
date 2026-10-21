use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum IPCEvent {
    Loading,
    Mixing,
    MixingSfx(u64),
    Sfx(u64),
    RenderFrame(u64),
    Frame(u64),
    Paused,
    Resumed,
    Done(f64),
}

pub mod client {
    use serde::Serialize;

    pub fn send<T: Serialize>(value: T) {
        println!("{}", serde_json::to_string(&value).unwrap());
    }
}
use crate::render::RenderParams;
use std::path::PathBuf;
use std::sync::Mutex;

static RENDER_PARAMS: Mutex<Option<RenderParams>> = Mutex::new(None);
static RENDER_OUTPUT: Mutex<Option<PathBuf>> = Mutex::new(None);

pub fn set_render_params(p: RenderParams, o: PathBuf) {
    *RENDER_PARAMS.lock().unwrap() = Some(p);
    *RENDER_OUTPUT.lock().unwrap() = Some(o);
}

pub fn get_render_params() -> anyhow::Result<RenderParams> {
    RENDER_PARAMS.lock().unwrap().take().ok_or_else(|| anyhow::anyhow!("No params"))
}

pub fn get_render_output() -> anyhow::Result<PathBuf> {
    RENDER_OUTPUT.lock().unwrap().take().ok_or_else(|| anyhow::anyhow!("No output"))
}