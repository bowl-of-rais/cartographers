mod chart;
mod cards;
mod player;
mod game;

use chart::Chart;
use game::Game;

fn main() {
    let mut c: Chart = Chart::new();
    match c.set(3, 3, chart::Terrain::Trees) {
        Ok(_) => print!("{}", c),
        Err(e) => print!("{}", e)
    }

    let mut g: Game = Game::new(4);
    g.play();
}
