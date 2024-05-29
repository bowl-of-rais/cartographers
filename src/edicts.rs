use std::{error::Error, fs};
use serde::Deserialize;
use crate::{chart::Chart, chart::Terrain, resource::Read};


#[derive(PartialEq, Deserialize)]
enum Segment {
    Row,
    Column,
    RowOrColumn,
    Cell,
    Cluster,
    Rectangle(usize, usize),
    Square(usize)
}

#[derive(Deserialize)]
enum Category {
    A,
    B, 
    C,
    D
}

#[derive(Deserialize)]
enum ScoreCondition {
    Containing(usize, Terrain), // TODO: min, max, only, not, different terrains?
    In(Segment, Terrain),
    NextTo(Segment, Terrain), // TODO: not, only
    DifferentTerrains(usize)
}

#[derive(Deserialize)]
pub struct Edict {
    multiplier : usize,
    scored_segment : Segment,
    condition : ScoreCondition,
    category : Category
}

const EDICTS_PATH : &str = "assets/edicts.json";

impl Default for Edict {
    fn default() -> Self {
        Edict {
            multiplier : 0,
            scored_segment : Segment::Row,
            condition : ScoreCondition::Containing(1, Terrain::Empty), 
            category : Category::A
        }
    }
}

impl Read for Edict {
    fn read() -> Result<Vec<Self>, Box<dyn Error>> {
        let file_contents = fs::read_to_string(EDICTS_PATH)?;
        let edicts : Vec<Edict> = serde_json::from_str(&file_contents)?;
        Ok(edicts)
    }
}

impl Edict {
    pub fn draw() -> [Self; 4] {
        !todo!()
    }

    pub fn score(chart: &Chart) -> i8 {
        !todo!()
    }
}

// TODO: serialize edicts T-T