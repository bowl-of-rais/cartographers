mod chart;

use chart::Chart;

fn main() {
    let mut c: Chart = Chart::new();
    match c.set(3, 3, chart::Terrain::Trees) {
        Ok(_) => print!("{}", c),
        Err(e) => print!("{}", e)
    }
}
