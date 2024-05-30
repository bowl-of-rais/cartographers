use crate::cards::{Card, Exploration};
use crate::edicts::{Category, Edict};
use crate::player::Player;
use crate::resource::Read;

use rand::seq::SliceRandom;
use rand::thread_rng;

const SEASON_LENGTHS: [i8; 4] = [6, 6, 7, 8];

const SEASON_NAMES: [&str; 4] = ["Spring", "Summer", "Autumn", "Winter"];

pub struct Game {
    players: Vec<Box<Player>>,
    edicts: [Edict; 4],
    explorations: Vec<Exploration>,
    current_season: usize,
}

impl Game {
    pub fn new(num_players: usize) -> Game {
        let explorations = match Exploration::read() {
            Ok(explorations) => explorations,
            Err(_) => Vec::new(), // TODO error handling/option
        };
        Game {
            players: vec![Box::new(Player::new()); num_players],
            edicts: [
                Edict::default(),
                Edict::default(),
                Edict::default(),
                Edict::default(),
            ],
            explorations: explorations,
            current_season: 0,
        }
    }

    pub fn play(&mut self) {
        self.draw_edicts();

        while self.current_season < 4 {
            let mut season_duration = 0;

            // build the deck: all explorations
            let mut deck: Vec<Box<dyn Card>> = self
                .explorations
                .to_vec()
                .into_iter()
                .map(|x| -> Box<dyn Card> { Box::new(x) })
                .collect();

            // shuffle the deck
            let mut rng = thread_rng();
            deck.shuffle(&mut rng);

            while season_duration < SEASON_LENGTHS[self.current_season] {
                println!(
                    "{} - ({}/{})",
                    SEASON_NAMES[self.current_season],
                    season_duration,
                    SEASON_LENGTHS[self.current_season]
                );

                // draw from cards
                let _ = match deck.pop() {
                    Some(c) => season_duration += self.process_card(&c),
                    None => println!("Deck is empty :0"),
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

    fn draw_edicts(&mut self) {
        let mut all_edicts = Edict::read().unwrap_or_default();
        let mut rng = thread_rng();
        all_edicts.shuffle(&mut rng);

        let find_first_of = |c: Category| {
            all_edicts
                .iter()
                .find_map(|&e| if e.category() == c { Some(e) } else { None })
                .unwrap_or_else(|| Edict::default())
        };

        self.edicts[0] = find_first_of(Category::A);
        self.edicts[1] = find_first_of(Category::B);
        self.edicts[2] = find_first_of(Category::C);
        self.edicts[3] = find_first_of(Category::D);
    }
}
