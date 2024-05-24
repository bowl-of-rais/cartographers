use std::fmt::Display;
use serde::Deserialize;
use crate::chart::Terrain;


// encodes a configuration of chart cells
#[derive(Clone, Debug, Default, Deserialize)]
pub struct Shape(Vec<Vec<bool>>);

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
pub struct Position((usize, usize));


// has everything needed to be put on a chart
#[derive(Default)]
pub struct Chartable {
    terrain : Terrain,
    shape : Shape,
    position : Position
}