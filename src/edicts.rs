use crate::{chart::Chart, chart::Terrain, resource::Read};
use serde::Deserialize;
use std::{error::Error, fs};

#[derive(Clone, Copy, Deserialize, PartialEq)]
enum FragmentShape {
    Row,
    Column,
    RowOrColumn,
    RowAndColumn,
    Cell,
    Cluster(Terrain),
    Spaces(Terrain, usize),
    Rectangle(usize, usize),
    Square(usize),
    Diagonal(bool),
}

// represent fragments as a set of cells?
type Fragment = Vec<(usize, usize)>;

#[derive(Clone, Copy, Deserialize, PartialEq)]
pub enum Category {
    Trees,
    WatersFarms,
    Villages,
    Structures,
}

#[derive(Clone, Copy, Deserialize)]
enum ScoreCondition {
    Containing(usize, Terrain), // TODO: min, max, only, not, different terrains?
    In(FragmentType, Terrain),
    NextTo(FragmentType, Terrain), // TODO: not, only
    DifferentTerrains(usize),
}

#[derive(Clone, Copy, Deserialize)]
pub struct Edict {
    multiplier: usize,
    scored_segment: SegmentType,
    condition: ScoreCondition,
    category: Category,
}

const EDICTS_PATH: &str = "assets/edicts.json";

impl Default for Edict {
    fn default() -> Self {
        Edict {
            multiplier: 0,
            scored_segment: SegmentType::Row,
            condition: ScoreCondition::Containing(1, Terrain::Empty),
            category: Category::Trees,
        }
    }
}

impl Read for Edict {
    fn read() -> Result<Vec<Self>, Box<dyn Error>> {
        let file_contents = fs::read_to_string(EDICTS_PATH)?;
        let edicts: Vec<Edict> = serde_json::from_str(&file_contents)?;
        Ok(edicts)
    }
}

impl Edict {
    pub fn category(&self) -> Category {
        return self.category;
    }

    pub fn score(&self, chart: &Chart) -> i8 {
        // general idea: count `Segments` that satisfy the given `ScoreCondition` and apply multiplier to count

        !todo!()
    }
}

// TODO: serialize edicts T-T
