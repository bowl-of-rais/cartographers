use std::fs;
use std::fmt::Display;
use std::error::Error;
use crate::chart::Terrain;
use crate::chartable::Shape;
use serde::Deserialize;

pub trait Card {
    fn duration() -> i8;
    fn terrain_options() -> Vec<Terrain>;
    fn shape_options() -> Vec<Shape>;
}


#[derive(Debug, Deserialize, Clone)]
pub struct Exploration {
    name : String,
    duration : i8,
    terrains : Vec<Terrain>,
    shapes : Vec<Vec<Vec<bool>>>,
    coin_shapes : Vec<bool>, // game rules: only shapes can give coins
}

impl Display for Exploration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.name, f)?;
        Display::fmt("\n", f)?;

        for terrain in &self.terrains { Display::fmt(terrain, f)?; }
        Display::fmt("\n", f)?;

        for shape in &self.shapes {
            for row in shape {
                for cell in row {
                    if *cell {
                        Display::fmt("X", f)?;
                    } else {
                        Display::fmt("_", f)?;
                    }
                }
                Display::fmt("\n", f)?;
            }
            Display::fmt("\n", f)?;
        }

        Ok(())
    }
}

pub fn read_explorations(path: &str) -> Result<Vec<Exploration>, Box<dyn Error>> {
    let file_contents = fs::read_to_string(path)?;
    let explorations : Vec<Exploration> = serde_json::from_str(&file_contents)?;
    Ok(explorations)
}

