mod chart;
mod cards;
mod player;
mod game;
mod chartable;

use chart::Chart;
use game::Game;

fn main() {
    let mut g: Game = Game::new(4);
    g.play();
}
