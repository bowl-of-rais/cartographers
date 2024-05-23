mod chart;
mod cards;
mod player;

use chart::Chart;

fn main() {
    let mut c: Chart = Chart::new();
    match c.set(3, 3, chart::Terrain::Trees) {
        Ok(_) => print!("{}", c),
        Err(e) => print!("{}", e)
    }

    let path = "assets/explorations.json";
    let expls = cards::read_explorations(path);

    match expls {
        Ok(explorations) => for ex in explorations { print!("{}", ex); },
        Err(e) => print!("{}", e)
    }
}
