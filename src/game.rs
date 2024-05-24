use crate::player::Player;
use crate::cards::{Card, Exploration};

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
        let explorations = match Exploration::read() {
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
            let mut season_duration = 0;

            // build the deck: all explorations
            let mut deck : Vec<Box<dyn Card>> = self.explorations
                .to_vec()
                .into_iter()
                .map(|x| -> Box<dyn Card>{Box::new(x)})
                .collect();

            // shuffle the deck
            let mut rng = thread_rng();
            deck.shuffle(&mut rng);

            while season_duration < SEASON_LENGTHS[self.current_season] {
                // draw from cards
                let _ = match deck.pop() {
                    Some (c) => season_duration += self.process_card(&c),
                    None => println!("Deck is empty :0")
                };
            }

            self.current_season += 1;
            
        }
    }

    fn process_card(&mut self, card: &Box<dyn Card>) -> i8 {
        // let players choose + update their charts
        self.have_players_choose(card);

        return card.duration();
    }

    fn have_players_choose(&mut self, card: &Box<dyn Card>) {
        for p in &self.players {
            let c = &p.make_choice(card);
            // TODO: setting the chartable
        }
    }
}