/// Each component is stored as f32 in a normalized range
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {
    pub const fn new(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b }
    }

    pub fn from_hsl(h: f32, s: f32, l: f32) -> Self {
        let h = h.rem_euclid(360.0);
        let s = f32::clamp(s, 0.0, 1.0);
        let l = f32::clamp(l, 0.0, 1.0);

        let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
        let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
        let m = l - c / 2.0;

        let (r, g, b) = match h {
            0.0..=60.0 => (c, x, 0.0),
            60.0..=120.0 => (x, c, 0.0),
            120.0..=180.0 => (0.0, c, x),
            180.0..=240.0 => (0.0, x, c),
            240.0..=300.0 => (x, 0.0, c),
            300.0..=360.0 => (c, 0.0, x),
            _ => unreachable!(),
        };

        Self::new(r + m, g + m, b + m)
    }

    pub const fn to_quantized(&self) -> (u8, u8, u8) {
        (
            f32::clamp(self.r * 255.0, 0.0, 255.0) as u8,
            f32::clamp(self.g * 255.0, 0.0, 255.0) as u8,
            f32::clamp(self.b * 255.0, 0.0, 255.0) as u8,
        )
    }

    pub const fn from_quantized(r: u8, g: u8, b: u8) -> Self {
        Self::new(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0)
    }

    pub const fn is_black(&self) -> bool {
        let (r, g, b) = self.to_quantized();
        (r == 0) && (g == 0) && (b == 0)
    }

    pub fn to_hex(&self) -> String {
        let (r, g, b) = self.to_quantized();
        format!("#{:02X}{:02X}{:02X}", r, g, b)
    }

    pub fn from_hex(hex: &str) -> Option<Color> {
        let hex = hex.strip_prefix('#').unwrap_or(hex);
        match hex.chars().collect::<Box<[char]>>().as_ref() {
            [c_r, c_g, c_b] | [c_r, c_g, c_b, _] => Some(Color::new(
                (c_r.to_digit(16)? as f32) / 15.0,
                (c_g.to_digit(16)? as f32) / 15.0,
                (c_b.to_digit(16)? as f32) / 15.0,
            )),
            [c1_r, c2_r, c1_g, c2_g, c1_b, c2_b] | [c1_r, c2_r, c1_g, c2_g, c1_b, c2_b, _, _] => {
                Some(Color::new(
                    (((c1_r.to_digit(16)? << 4) | c2_r.to_digit(16)?) as f32) / 255.0,
                    (((c1_g.to_digit(16)? << 4) | c2_g.to_digit(16)?) as f32) / 255.0,
                    (((c1_b.to_digit(16)? << 4) | c2_b.to_digit(16)?) as f32) / 255.0,
                ))
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::Color;

    #[test]
    fn test() {
        fn test_hex(r: u8, g: u8, b: u8) {
            let color = Color::from_quantized(r, g, b);
            assert_eq!(Color::from_hex(&color.to_hex()).unwrap(), color);
        }

        test_hex(255, 127, 0);
        test_hex(69, 42, 127);
    }
}
