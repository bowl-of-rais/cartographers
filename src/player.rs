use crate::chart::Chart;

#[derive(Clone)]
pub struct Player {
    coins : i8,
    points : [i8;4],
    chart : Box<Chart>,
}

impl Player {
    pub fn new() -> Player {
        Player {
            coins: 0,
            points: [0, 0, 0, 0],
            chart: Box::new(Chart::new())
        }
    }
}