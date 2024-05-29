mod chart;
mod cards;
mod player;
mod game;
mod chartable;
mod edicts;

use game::Game;

fn main() {
    let mut g: Game = Game::new(1);
    g.play();
}
