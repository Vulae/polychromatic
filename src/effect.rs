use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use crate::{Color, FPS_RANGE, PolychromaticError, defs, device::Device};

#[derive(Debug)]
pub struct EffectMatrix {
    width: u32,
    #[allow(unused)]
    height: u32,
    values: Box<[Color]>,
}

impl EffectMatrix {
    fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            values: vec![Color::default(); width as usize * height as usize].into_boxed_slice(),
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    fn pos_to_index(&self, x: u32, y: u32) -> Option<usize> {
        if x >= self.width || y >= self.height {
            return None;
        }
        Some(x as usize + y as usize * self.width as usize)
    }

    pub fn get(&self, x: u32, y: u32) -> Option<&Color> {
        self.values.get(self.pos_to_index(x, y)?)
    }

    pub fn get_mut(&mut self, x: u32, y: u32) -> Option<&mut Color> {
        self.values.get_mut(self.pos_to_index(x, y)?)
    }

    pub fn set(&mut self, color: Color, x: u32, y: u32) {
        if let Some(matrix_color) = self.get_mut(x, y) {
            *matrix_color = color;
        }
    }

    pub fn values(&self) -> &[Color] {
        &self.values
    }

    pub fn values_mut(&mut self) -> &mut [Color] {
        &mut self.values
    }

    pub fn iter(&self) -> impl Iterator<Item = (u32, u32, &Color)> {
        self.values.iter().enumerate().map(|(i, c)| {
            (
                (i % self.width as usize) as u32,
                (i / self.width as usize) as u32,
                c,
            )
        })
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (u32, u32, &mut Color)> {
        self.values.iter_mut().enumerate().map(|(i, c)| {
            (
                (i % self.width as usize) as u32,
                (i / self.width as usize) as u32,
                c,
            )
        })
    }
}

#[derive(Debug)]
pub struct Effect {
    pub name: String,
    pub author: String,
    pub icon: PathBuf,
    pub summary: String,
    device: Device,
    fps: u32,
    pub r#loop: bool,
    width: u32,
    height: u32,
    frames: Vec<EffectMatrix>,
}

impl Effect {
    pub fn new<P: AsRef<Path>>(device: Device, icon: P) -> Result<Self, PolychromaticError> {
        let (width, height) = device
            .matrix()
            .ok_or(PolychromaticError::DeviceUnsupportedEffects(device))?;
        Ok(Self {
            name: "Unnamed".to_owned(),
            author: String::new(),
            icon: icon.as_ref().to_path_buf(),
            summary: String::new(),
            device,
            fps: 1,
            r#loop: true,
            width,
            height,
            frames: Vec::new(),
        })
    }

    pub fn set_fps(&mut self, fps: u32) -> Result<(), PolychromaticError> {
        if !FPS_RANGE.contains(&fps) {
            return Err(PolychromaticError::InvalidFPS(fps));
        }
        self.fps = fps;
        Ok(())
    }

    pub fn device(&self) -> &Device {
        &self.device
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn frames(&self) -> &[EffectMatrix] {
        &self.frames
    }

    pub fn frames_mut(&mut self) -> &mut [EffectMatrix] {
        &mut self.frames
    }

    pub fn new_frame(&mut self) -> &mut EffectMatrix {
        self.frames.push(EffectMatrix::new(self.width, self.height));
        self.frames.last_mut().unwrap()
    }

    pub fn to_effect_json(&self) -> Result<String, PolychromaticError> {
        // TODO: Each frame don't encode pixels that have a very slight or no visual difference
        // from the last frame.
        Ok(serde_json::to_string(&defs::Effect {
            name: self.name.clone(),
            author: self.author.clone(),
            icon: self.icon.clone().canonicalize()?,
            summary: self.summary.clone(),
            r#type: 3,
            map_device: self.device.to_string(),
            map_device_icon: self.device.icon().to_owned(),
            // TODO: Implement.
            map_graphic: "blackwidow_v3_en_US.svg".to_owned(),
            map_cols: self.width,
            map_rows: self.height,
            save_format: 8,
            revision: 1,
            fps: self.fps,
            r#loop: self.r#loop,
            frames: self
                .frames()
                .iter()
                .map(|frame| {
                    let mut cols: HashMap<String, HashMap<String, String>> = HashMap::new();
                    frame
                        .iter()
                        .filter(|(_, _, color)| !color.is_black())
                        .for_each(|(row, col, color)| {
                            cols.entry(row.to_string())
                                .or_default()
                                .insert(col.to_string(), color.to_hex());
                        });
                    defs::EffectFrame { cols }
                })
                .collect(),
        })?)
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), PolychromaticError> {
        let str = self.to_effect_json()?;
        std::fs::write(path, &str)?;
        Ok(())
    }
}
