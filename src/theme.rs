use bevy::color::palettes::tailwind::*;
use bevy::prelude::*;
#[derive(Resource)]
pub struct Theme {
    pub background: Color,
    pub input: ThemeItem,
    pub font: Handle<Font>,
}

pub struct ThemeItem {
    pub color: Color,
    pub background_color: Color,
    pub hover_background_color: Color,
    pub hover_color: Color,
    pub size: f32,
}

const WHITE: Color = Color::srgb(0.953125, 0.95703125, 0.96484375);
const DARK: Color = Color::Srgba(GRAY_800);
const LIGHT_BLUE: Color = Color::Srgba(BLUE_100);
const BLUE: Color = Color::Srgba(BLUE_900);
impl Default for Theme {
    fn default() -> Self {
        Self {
            background: WHITE,
            input: ThemeItem {
                color: DARK,
                background_color: WHITE,

                size: 12.0,
                hover_background_color: LIGHT_BLUE,
                hover_color: BLUE,
            },
            font: Default::default(),
        }
    }
}
