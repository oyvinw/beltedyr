use bevy::prelude::*;

pub fn rgb(r: u8, g: u8, b: u8) -> Color {
    Color::rgb(r as f32 / 255., g as f32 / 255., b as f32 / 255.)
}

pub fn hex(hex_code: &String) -> Result<Color, std::num::ParseIntError> {
        let r: u8 = u8::from_str_radix(&hex_code[1..3], 16)?;
        let g: u8 = u8::from_str_radix(&hex_code[3..5], 16)?;
        let b: u8 = u8::from_str_radix(&hex_code[5..7], 16)?;

    Ok(rgb(r, g, b))
}