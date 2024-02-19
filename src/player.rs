use crate::chart::Chart;

pub struct Player {
    coins : i8,
    points : [i8;4],
    map : Map,
    chart : Chart,
}

impl Player {
    fn new(num_mountains: i8, num_ruins: i8, num_wastelands: i8) -> Player {
        Player {
            coins: 0,
            points: [0, 0, 0, 0],
            chart: Chart::new()
        }
    }
}