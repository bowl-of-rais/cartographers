use crate::chart::Terrain;
use crate::chartable::Shape;
use crate::resource::Read;
use serde::Deserialize;
use std::error::Error;
use std::fmt::Display;
use std::fs;

// CARDS
// Are drawn from a stack, players put new entries into their charts based on the contents.
// Can be read from corresponding input file.
pub trait Card: Read {
    fn duration(&self) -> i8;
    fn terrain_options(&self) -> &Vec<Terrain>;
    fn shape_options(&self) -> &Vec<Shape>;
    fn rewards(&self) -> Option<&Vec<bool>>;
}

// EXPLORATIONS
// Cards containing a choice of shapes (some of which award a coin) and a choice of terrains.
// Players choose a shape and terrain to put somewhere on their personal chart.

#[derive(Debug, Deserialize, Clone)]
pub struct Exploration {
    name: String,
    duration: i8,
    terrains: Vec<Terrain>,
    shapes: Vec<Shape>,
    coin_shapes: Vec<bool>, // game rules: only shapes can give coins
}

const EXPLORATION_PATH: &str = "assets/explorations.json";

impl Read for Exploration {
    fn read() -> Result<Vec<Self>, Box<dyn Error>> {
        let file_contents = fs::read_to_string(EXPLORATION_PATH)?;
        let explorations: Vec<Exploration> = serde_json::from_str(&file_contents)?;
        Ok(explorations)
    }
}

impl Card for Exploration {
    fn duration(&self) -> i8 {
        return self.duration;
    }

    fn terrain_options(&self) -> &Vec<Terrain> {
        return &self.terrains;
    }

    fn shape_options(&self) -> &Vec<Shape> {
        return &self.shapes;
    }

    fn rewards(&self) -> Option<&Vec<bool>> {
        return Some(&self.coin_shapes);
    }
}

impl Display for Exploration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.name, f)?;
        Display::fmt("\n", f)?;

        for terrain in &self.terrains {
            Display::fmt(terrain, f)?;
        }
        Display::fmt("\n", f)?;

        for shape in &self.shapes {
            Display::fmt(shape, f)?;
            Display::fmt("\n", f)?;
        }

        Ok(())
    }
}
