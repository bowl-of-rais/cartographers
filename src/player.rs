use crate::chart::Chart;

pub struct Player {
    coins : i8,
    points : [i8;4],
    chart : Chart,
}

impl Player {
    fn new() -> Player {
        Player {
            coins: 0,
            points: [0, 0, 0, 0],
            chart: Chart::new()
        }
    }
}