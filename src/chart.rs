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
    ruins: Vec<(usize, usize)>
}