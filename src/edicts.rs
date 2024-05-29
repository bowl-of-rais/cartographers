use crate::chart::Terrain;


#[derive(PartialEq)]
enum Segment {
    Row,
    Column,
    RowOrColumn,
    Cell,
    Cluster,
    Rectangle(usize, usize),
    Square(usize)
}

enum ScoreCondition {
    Containing(usize, Terrain), // TODO: min, max, only, not, different terrains?
    In(Segment, Terrain),
    NextTo(Segment, Terrain), // TODO: not, only
    DifferentTerrains(usize)
}

pub struct Edict {
    multiplier : usize,
    scored_segment : Segment,
    condition : ScoreCondition
}

impl Default for Edict {
    fn default() -> Self {
        Edict {
            multiplier : 0,
            scored_segment : Segment::Row,
            condition : ScoreCondition::Containing(1, Terrain::Empty), 
        }
    }
}

// TODO: read edicts from json