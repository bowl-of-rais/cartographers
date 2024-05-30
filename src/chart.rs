use std::fmt;
use std::fmt::Display;

use crate::chartable::Chartable;
use array2d::{Array2D, Error};
use serde::Deserialize;

const CHART_SIZE: usize = 11;

#[derive(Debug)]
pub enum MapError {
    SpaceNotEmpty,
    ArrayError(Error),
}

impl Display for MapError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MapError::SpaceNotEmpty => Display::fmt(&"SpaceNotEmpty", f),
            MapError::ArrayError(e) => Display::fmt(e, f),
        }
    }
}

impl From<Error> for MapError {
    fn from(err: Error) -> MapError {
        MapError::ArrayError(err)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq)]
pub enum Terrain {
    Empty,
    Farm,
    Monster,
    Mountain,
    Trees,
    Village,
    Wasteland,
    Water,
}

impl Display for Terrain {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Terrain::Empty => Display::fmt(&" _ ", f),
            Terrain::Farm => Display::fmt(&" F ", f),
            Terrain::Monster => Display::fmt(&" M ", f),
            Terrain::Mountain => Display::fmt(&" ^ ", f),
            Terrain::Trees => Display::fmt(&" T ", f),
            Terrain::Village => Display::fmt(&" V ", f),
            Terrain::Wasteland => Display::fmt(&" * ", f),
            Terrain::Water => Display::fmt(&" W ", f),
        }
    }
}

impl Default for Terrain {
    fn default() -> Self {
        return Terrain::Empty;
    }
}

#[derive(Clone)]
pub struct Chart {
    contents: Array2D<Terrain>,
    free_ruins: Vec<(usize, usize)>,
    penalized: Vec<(usize, usize)>,
}

impl Display for Chart {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
        for row_iter in self.contents.rows_iter() {
            Display::fmt(&'|', f)?;
            for element in row_iter {
                Display::fmt(element, f)?;
            }
            Display::fmt(&'|', f)?;
            Display::fmt(&'\n', f)?;
        }
        Ok(())
    }
}

impl Chart {
    pub fn new() -> Chart {
        Chart {
            contents: Array2D::filled_with(Terrain::Empty, CHART_SIZE, CHART_SIZE),
            free_ruins: Vec::new(),
            penalized: Vec::new(),
        }
    }

    pub fn set(&mut self, c: Chartable) -> Result<(), MapError> {
        let (xlen, ylen) = c.shape().size();
        let (x, y) = c.position();

        for i in 0..xlen {
            for j in 0..ylen {
                if c.shape().at(i, j) {
                    self.set_cell(x + i, y + j, c.terrain())?;
                }
            }
        }
        Ok(())
    }

    fn set_cell(&mut self, row: usize, col: usize, terrain: Terrain) -> Result<(), MapError> {
        self.contents.set_terrain(row, col, terrain)?;
        self.penalized.retain(|(x, y)| *x != row || *y != col);
        self.free_ruins.retain(|(x, y)| *x != row || *y != col);
        Ok(())
    }

    pub fn get(&mut self, row: usize, col: usize) -> Option<&Terrain> {
        return self.contents.get(row, col);
    }

    pub fn contents(&self) -> &Array2D<Terrain> {
        return &self.contents;
    }

    pub fn free_ruins(&self) -> &Vec<(usize, usize)> {
        return &self.free_ruins;
    }

    pub fn penalized(&self) -> &Vec<(usize, usize)> {
        return &self.penalized;
    }
}

pub trait TerrainSettable {
    fn set_terrain(&mut self, row: usize, column: usize, element: Terrain) -> Result<(), MapError>;
}

impl TerrainSettable for Array2D<Terrain> {
    fn set_terrain(&mut self, row: usize, column: usize, element: Terrain) -> Result<(), MapError> {
        let current = self
            .get(row, column)
            .ok_or(Error::IndicesOutOfBounds(row, column))?;

        if *current == Terrain::Empty {
            self.set(row, column, element)?;
        } else {
            return Err(MapError::SpaceNotEmpty);
        }

        Ok(())
    }
}
