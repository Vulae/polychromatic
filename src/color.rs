/// Each component is stored as f32 in a normalized range
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b }
    }

    pub fn from_hsl(h: f32, s: f32, l: f32) -> Self {
        let h = h % 360.0;
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

    pub fn quantize(&self) -> (u8, u8, u8) {
        (
            f32::clamp(self.r * 255.0, 0.0, 255.0) as u8,
            f32::clamp(self.g * 255.0, 0.0, 255.0) as u8,
            f32::clamp(self.b * 255.0, 0.0, 255.0) as u8,
        )
    }

    pub fn is_black(&self) -> bool {
        let (r, g, b) = self.quantize();
        (r == 0) && (g == 0) && (b == 0)
    }

    pub fn to_hex(&self) -> String {
        let (r, g, b) = self.quantize();
        format!("#{:02X}{:02X}{:02X}", r, g, b)
    }
}
