use clap::Parser;
use polychromatic::{Color, Device, Effect, Keyboard};
use std::{error::Error, path::PathBuf, sync::LazyLock};

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
const FRAMES: u32 = 400;

const SCALE: f32 = 0.15;

#[rustfmt::skip]
static COLORS: LazyLock<Box<[Color]>> = LazyLock::new(|| {
    [
        "#E50000", "#FF8D00", "#FFEE00", "#028121", "#004CFF", "#770088", // Pride
        "#55CDFD", "#F6AAB7", "#FFFFFF", "#F6AAB7", "#55CDFD", // Transgender
        "#FCF431", "#FCFCFC", "#9D59D2", "#282828", // Nonbinary
        "#FE76A2", "#FFFFFF", "#BF12D7", "#000000", "#303CBE", // Genderfluid
        "#D60270", "#9B4F96", "#0038A8", // Bisexual
        "#FF1C8D", "#FFD700", "#1AB3FF", // Pansexual
        "#078D70", "#98E8C1", "#FFFFFF", "#7BADE2", "#3D1A78", // Gay
        "#D62800", "#FF9B56", "#FFFFFF", "#D462A6", "#A40062", // Lesbian
        "#000000", "#A4A4A4", "#FFFFFF", "#810081", // Asexual
        "#3BA740", "#A8D47A", "#FFFFFF", "#ABABAB", "#000000", // Aromantic
        "#F714BA", "#01D66A", "#1594F6", // Polysexual
    ]
    .into_iter()
    .map(|hex| Color::from_hex(hex).unwrap())
    .collect()
});

fn get_color(percent: f32) -> Color {
    let percent = percent % 1.0;

    let index = (percent * COLORS.len() as f32) as usize;
    let next_index = (index + 1) % COLORS.len();
    let color = COLORS[index];
    let next_color = COLORS[next_index];

    fn lerp(v0: f32, v1: f32, t: f32) -> f32 {
        (1.0 - t) * v0 + t * v1
    }

    fn simple_ease(x: f32) -> f32 {
        if x > 0.5 {
            1.0 - (2.0 * (1.0 - x)).powi(4) / 2.0
        } else {
            (2.0 * x).powi(4) / 2.0
        }
    }

    let interpolation = percent % (1.0 / COLORS.len() as f32) * COLORS.len() as f32;
    let interpolation = simple_ease(interpolation);

    Color::new(
        lerp(color.r, next_color.r, interpolation),
        lerp(color.g, next_color.g, interpolation),
        lerp(color.b, next_color.b, interpolation),
    )
}

pub fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    let mut effect = Effect::new(Device::Keyboard(Keyboard::RazerOrnataChroma), &cli.icon)?;

    effect.name = cli.output.file_stem().unwrap().to_str().unwrap().to_owned();

    effect.set_fps(FPS)?;

    let width = effect.width();

    let mut anim_percent = 0.0;
    while anim_percent < 1.0 {
        anim_percent += 1.0 / (FRAMES as f32);

        let frame = effect.new_frame();
        frame.iter_mut().for_each(|(x, _y, color)| {
            let percent = x as f32 / width as f32;
            *color = get_color((percent * SCALE) + anim_percent);
        });
    }

    effect.save(&cli.output)?;

    Ok(())
}
