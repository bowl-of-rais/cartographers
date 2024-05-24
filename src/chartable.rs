use std::fmt::Display;
use serde::Deserialize;
use crate::chart::Terrain;


// encodes a configuration of chart cells
#[derive(Clone, Debug, Default, Deserialize)]
pub struct Shape(pub Vec<Vec<bool>>);

impl Display for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.0 {
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
        Ok(())
    }
}


// neater representation of a position on the chart
#[derive(Default)]
pub struct Position(usize, usize);


// has everything needed to be put on a chart
#[derive(Default)]
pub struct Chartable {
    terrain : Terrain,
    shape : Shape,
    position : Position
}

impl Chartable {
    pub fn new (terrain : Terrain, shape : Shape, position : Position) -> Chartable {
        Chartable {
            terrain,
            shape,
            position
        }
    }

    pub fn terrain(&self) -> Terrain {
        return self.terrain;
    }

    pub fn shape(&self) -> &Shape {
        return &self.shape;
    }

    pub fn x(&self) -> usize {
        return self.position.0;
    }

    pub fn y(&self) -> usize {
        return self.position.1;
    }
}