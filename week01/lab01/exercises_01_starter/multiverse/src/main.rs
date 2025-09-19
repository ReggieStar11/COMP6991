// hmm this doesn't look right!!
struct UniverseDetails {
    universe_name: String,
    universe_winner: String,
    universe_population: u32,
}

fn get_universe_details(universe_id: u32) -> Option<UniverseDetails> {
    if universe_id % 3 == 0 && universe_id % 5 == 0 {
        return Some(UniverseDetails {
            universe_name: "Stardew Valley".to_string(),
            universe_winner: "Jojo Corp".to_string(),
            universe_population: 1,
        });
    } else if universe_id % 3 == 0 {
        return Some(UniverseDetails {
            universe_name: "Star Wars".to_string(),
            universe_winner: "The Rebellion".to_string(),
            universe_population: 4294967295,
        });
    } else if universe_id % 5 == 0 {
        return Some(UniverseDetails {
            universe_name: "Miraculous".to_string(),
            universe_winner: "Hawk Moth".to_string(),
            universe_population: 22,
        });
    }
    None
}
// No changes needed here; "Star Wars" is already a string literal, but Rust requires .to_string() to convert it to a String type.

// this main function is fine, except for two gaps
// the print statements could make use of "{variable}" instead of
// ("{}", variable)

fn main() {
    for id in 1..=15 {
        let universe_details = get_universe_details(id);
        if let Some(d) = universe_details {
            println!(
                "Universe with id {} is called {}, won by {} and has a population of {}",
                id, d.universe_name, d.universe_winner, d.universe_population
            );
        } else {
            println!("Universe with id {} is unknown", id);
        }
    }
}
