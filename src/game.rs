use crate::player::Player;
use crate::cards::{self, Exploration};

use rand::thread_rng;
use rand::seq::SliceRandom;

const SEASON_LENGTHS: [i8;4] = [6, 6, 7, 8];

pub struct Game {
    players : Vec<Box<Player>>,
    explorations : Vec<Exploration>,
    current_season : usize,
}

impl Game {
    pub fn new(num_players: usize) -> Game {
        let explorations = match cards::read_explorations("assets/explorations.json") {
                Ok(explorations) => explorations,
                Err(_) => Vec::new() // TODO error handling/option
            };
        Game {
            players : vec![Box::new(Player::new()); num_players],
            explorations : explorations,
            current_season : 0,
        }
    }

    pub fn play(&mut self) {

        while self.current_season < 4 {
            let season_duration = 0;

            let mut card_stack : Vec<Exploration> = Vec::new();
            card_stack.clone_from_slice(&self.explorations);
            let mut rng = thread_rng();
            card_stack.shuffle(&mut rng);

            while season_duration < SEASON_LENGTHS[self.current_season] {
                // draw from card stack
                
                // do card actions -> update players + charts

                // update season_duration
            }

            self.current_season += 1;
            
        }
    }
}