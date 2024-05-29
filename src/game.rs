use crate::player::Player;
use crate::cards::{Card, Exploration};
use crate::edicts::Edict;
use crate::resource::Read;

use rand::thread_rng;
use rand::seq::SliceRandom;

const SEASON_LENGTHS: [i8;4] = [6, 6, 7, 8];

const SEASON_NAMES: [&str; 4] = ["Spring", "Summer", "Autumn", "Winter"];

pub struct Game {
    players : Vec<Box<Player>>,
    edicts : [Edict; 4],
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
            edicts : [Edict::default(), Edict::default(), Edict::default(), Edict::default()],
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
                println!("{} - ({}/{})", SEASON_NAMES[self.current_season], season_duration, SEASON_LENGTHS[self.current_season]);

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
        // TODO: print card once?

        // have players choose + update their charts
        for p in &mut self.players {
            let _ = p.play_turn(card);
        }

        return card.duration();
    }

}