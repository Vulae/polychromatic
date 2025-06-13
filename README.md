
# [polychromatic-rs](#polychromatic-rs)

Generate [polychromatic](https://polychromatic.app/) effects with code!

# [Example](#example)

```Rust
use polychromatic::{Color, Device, Effect, Keyboard, PolychromaticError};

const FPS: u32 = 30;
const FRAMES: u32 = 60;

pub fn polychromatic_effect_rainbow(output: std::path::PathBuf, icon: std::path::PathBuf) -> Result<(), PolychromaticError> {
    let mut effect = Effect::new(Device::Keyboard(Keyboard::detect_one()?), &icon)?;

    effect.name = output.file_stem().unwrap().to_str().unwrap().to_owned();
    effect.summary = "Rainbow!!!!".to_owned();

    effect.set_fps(FPS)?;

    let width = effect.width();

    let mut hue_rot = 0.0;
    while hue_rot < 360.0 {
        hue_rot += 360.0 / (FRAMES as f32);

        let frame = effect.new_frame();
        frame.iter_mut().for_each(|(x, _y, color)| {
            let hue = (x as f32 / width as f32) * 360.0;
            let hue = if y % 2 == 0 { hue } else { -hue };
            *color = Color::from_hsl(hue + hue_rot, 1.0, 0.5);
        });
    }

    effect.save(&output)?;

    Ok(())
}
```

> [!TIP]
> Run the above example:
> 
> `cargo run --example rainbow -- --output "./effects/Rainbow.json" --icon "./effects/pride.png"`
> 
> Move the output file to `~/.config/polychromatic/effects/` to be able to use the effect in the polychromatic app.

