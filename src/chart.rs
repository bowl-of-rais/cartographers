use std::fmt;
use std::fmt::Display;

use array2d::Array2D;

const CHART_SIZE: usize = 11;

#[derive(Clone)]
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
}