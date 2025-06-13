use clap::Parser;
use polychromatic::{Color, Device, Effect, Keyboard};
use std::{error::Error, path::PathBuf};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[arg(short, long)]
    pub icon: PathBuf,
    #[arg(short, long)]
    pub output: PathBuf,
}

const FPS: u32 = 30;
const FRAMES: u32 = 60;

pub fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    let mut effect = Effect::new(Device::Keyboard(Keyboard::detect_one()?), &cli.icon)?;

    effect.name = cli.output.file_stem().unwrap().to_str().unwrap().to_owned();
    effect.summary = "Rainbow!!!!".to_owned();

    effect.set_fps(FPS)?;

    let width = effect.width();

    let mut hue_rot = 0.0;
    while hue_rot < 360.0 {
        hue_rot += 360.0 / (FRAMES as f32);

        let frame = effect.new_frame();
        frame.iter_mut().for_each(|(x, y, color)| {
            let hue = x as f32 / width as f32;
            let hue = if y % 2 == 0 { hue } else { -hue };
            *color = Color::from_hsl(hue * 360.0 + hue_rot, 1.0, 0.5);
        });
    }

    effect.save(&cli.output)?;

    Ok(())
}
