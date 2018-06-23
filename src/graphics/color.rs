#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[repr(C)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(C)]
pub struct Color32 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl From<Color32> for Color {
    fn from(color32: Color32) -> Self {
        Color {
            r: (color32.r as f32) / 255.0,
            g: (color32.g as f32) / 255.0,
            b: (color32.b as f32) / 255.0,
            a: (color32.a as f32) / 255.0,
        }
    }
}

impl From<Color> for Color32 {
    fn from(color: Color) -> Self {
        Color32 {
            r: (color.r * 255.99) as u8,
            g: (color.g * 255.99) as u8,
            b: (color.b * 255.99) as u8,
            a: (color.a * 255.99) as u8,
        }
    }
}
