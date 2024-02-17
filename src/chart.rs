use std::fmt;
use std::fmt::Display;

use array2d::{Array2D, Error};

const CHART_SIZE: usize = 11;

#[derive(Debug)]
pub enum MapError {
    SpaceNotEmpty,
    ArrayError(Error)
}

impl Display for MapError {
    fn fmt(&self, f: &mut std::fmt::Formatter)->std::fmt::Result{
        match self {
            MapError::SpaceNotEmpty => Display::fmt(&"SpaceNotEmpty", f),
            MapError::ArrayError(e) => Display::fmt(e, f)
        }
    }
}

impl From<Error> for MapError {
    fn from(err: Error) -> MapError {
        MapError::ArrayError(err)
    }
}

#[derive(Clone)]
#[derive(PartialEq)]
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
    fn fmt(&self, f: &mut std::fmt::Formatter)->std::fmt::Result{
        match self {
            Terrain::Empty => Display::fmt(&" _ ", f),
            Terrain::Farm =>  Display::fmt(&" F ", f),
            Terrain::Monster => Display::fmt(&" M ", f),
            Terrain::Mountain =>  Display::fmt(&" ^ ", f),
            Terrain::Trees =>  Display::fmt(&" T ", f),
            Terrain::Village =>  Display::fmt(&" V ", f),
            Terrain::Wasteland =>  Display::fmt(&" * ", f),
            Terrain::Water =>  Display::fmt(&" W ", f),
        }
    }
}

pub struct Chart {
    contents: Array2D<Terrain>,
    ruins: Vec<(usize, usize)>,
    penalized: Vec<(usize, usize)>
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
            ruins: Vec::new(),
            penalized: Vec::new()
        }
    }

    pub fn set(&mut self, row: usize, col: usize, terrain: Terrain) -> Result<(), MapError> {
        self.contents.set_terrain(row, col, terrain)?;
        self.penalized.retain(| (x, y) | *x != row || *y != col);
        Ok(())
    }

    pub fn get(&mut self, row: usize, col: usize) -> Option<&Terrain> {
        return self.contents.get(row, col);
    }
}

pub trait TerrainSettable {
    fn set_terrain(&mut self, row: usize, column: usize, element: Terrain) -> Result<(), MapError>;
}

impl TerrainSettable for Array2D<Terrain> {
    fn set_terrain(&mut self, row: usize, column: usize, element: Terrain) -> Result<(), MapError> {
        let current = self.get(row, column).ok_or(Error::IndicesOutOfBounds(row, column))?;
        
        if *current == Terrain::Empty { 
            self.set(row, column, element)?; 
        } else { 
            return Err(MapError::SpaceNotEmpty);
        }

        Ok(())
    }
}