//! https://docs.polychromatic.app/config/effects/

use std::{collections::HashMap, path::PathBuf};

use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(transparent)]
pub struct EffectFrame {
    pub cols: HashMap<String, HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
pub struct Effect {
    pub name: String,
    pub r#type: i64,
    pub author: String,
    pub icon: PathBuf,
    pub summary: String,
    pub map_device: String,
    pub map_device_icon: String,
    pub map_graphic: String,
    pub map_cols: u32,
    pub map_rows: u32,
    pub save_format: i64,
    pub revision: u64,
    pub fps: u32,
    pub r#loop: bool,
    pub frames: Vec<EffectFrame>,
}
