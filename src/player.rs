use std::{fmt::Display, io};
use crate::{cards::Card, chart::{Chart, Terrain, MapError}, chartable::{Chartable, Position, Shape}};

pub enum PlayerError {
    ChoiceError
}

#[derive(Clone)]
pub struct Player {
    coins : i8,
    points : [i8;4],
    chart : Box<Chart>,
}

impl Player {
    pub fn new() -> Player {
        Player {
            coins: 0,
            points: [0, 0, 0, 0],
            chart: Box::new(Chart::new())
        }
    }

    pub fn play_turn(&mut self, card: &Box<dyn Card>) -> Result<(), MapError> {

        let t : Terrain;
        let mut s : &Shape = &Shape::default();
        let p : Position;

        let mut coins : i8 = 0;

        // choose a terrain
        match self.choose_option(card.terrain_options(), None) {
            (Some(terrain_option), _) => t = *terrain_option,
            (None, _) => t = Terrain::Empty,
        }

        // choose a shape
        match self.choose_option(card.shape_options(), card.rewards()) {
            (Some(shape_option), reward) => { 
                s = shape_option;
                if reward { coins += 1 }
            },
            (None, _) => {} // TODO this feels ugly
        }

        // choose a position
        match self.choose_position(t, &s) {
            Some(position) => p = position,
            None => p = Position::default(),
        }

        // put it into the player's chart
        let c = Chartable::new(t, s.clone(), p);
        self.chart.set(c)?; // TODO: more error handling if this goes wrong

        println!("Updated chart:");
        println!("{}", self.chart);

        // update coins
        self.add_coins(coins);

        Ok(())

    }

    fn add_coins(&mut self, number : i8) {
        self.coins += number;
    }

    fn choose_option<'a, T: Display>(&'a self, options: &'a Vec<T>, rewards : Option<&Vec<bool>>) -> (Option<&T>, bool) {
        let first_choice = options.get(0);

        if let Some(_choice) = first_choice {

            println!("Please choose one of the following options:");
    
            if let Some(rewards) = rewards {
                if rewards.contains(&true) {
                    println!("The following option(s) give a reward of 1 coin:");
                    for (num, reward) in rewards.iter().enumerate() {
                        if *reward { print!("{} ", num+1); }
                    }
                    println!("")
                }
            }
            
            for (num, option) in options.iter().enumerate() {
                println!("Option {}:", num+1);
                println!("{}", option);
            }

            loop {
                let mut str_choice = String::new();
                let num_choice : usize;
                
                io::stdin()
                .read_line(&mut str_choice)
                .expect("Failed to read line");

                num_choice = str_choice.trim().parse().expect("Please type a number!");

                match options.get(num_choice-1) {
                    Some(choice) => { 
                        let reward = rewards.unwrap_or(&Vec::new()).get(num_choice-1).is_some_and(|x| *x);
                        return (Some(choice), reward) 
                    },
                    None => { 
                        println!("Please choose a number between {} and {}!", 1, options.len());
                        continue;
                    }
                }
            }
        }

        (None, false)
    }

    fn choose_position(&self, terrain : Terrain, shape : &Shape) -> Option<Position> {
        
        println!("Please choose a position for the upper left corner of the shape (0-based):");

        println!("Terrain:");
        println!("{}", terrain);
        println!("Shape:");
        println!("{}", shape);
        println!("Current chart:");
        println!("{}", self.chart);


        let mut str_row = String::new();
        let mut str_col = String::new();
        
        println!("Row:");

        io::stdin()
        .read_line(&mut str_row)
        .expect("Failed to read line");

        let row = str_row.trim().parse().expect("Please type a number!");

        println!("Column:");

        io::stdin()
        .read_line(&mut str_col)
        .expect("Failed to read line");

        let col = str_col.trim().parse().expect("Please type a number!");

        // TODO: validate values
        Some((row, col))

    }
}