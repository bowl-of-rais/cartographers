mod cards;
mod chart;
mod chartable;
mod edicts;
mod game;
mod player;
mod resource;

use game::Game;

fn main() {
    let mut g: Game = Game::new(1);
    g.play();
}
