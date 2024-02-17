use array2d::Array2D;

static CHART_SIZE: usize = 11;

#[derive(Clone)]
pub enum Terrain {
    Borderland,
    Empty,
    Farm,
    Monster,
    Mountain,
    Trees,
    Village,
    Wasteland,
    Water,
}

pub struct Chart {
    contents: Array2D<Terrain>,
    ruins: Vec<(usize, usize)>,
    penalized: Vec<(usize, usize)>
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