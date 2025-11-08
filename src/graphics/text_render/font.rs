use std::collections::HashMap;

use super::super::texture2d::Texture2D;
use super::character::FontCharacter;

pub struct FontData {
    pub texture: Texture2D,
    pub glyphs: HashMap<char, FontCharacter>
}

impl FontData {
    pub fn new(texture: Texture2D, glyphs: HashMap<char, FontCharacter>) -> Self {
        Self {
            texture,
            glyphs
        }
    }
}