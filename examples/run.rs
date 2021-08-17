use std::time::Instant;
use firecore_battle::moves::usage::MoveUsage;

fn main() {
    println!("Building dex...");
    let start = Instant::now();
    firecore_pokedex_builder::compile::<MoveUsage, &str>("assets/pokedex/pokemon", "assets/pokedex/moves", "assets/pokedex/items");
    println!("Finished in {}ms!", start.elapsed().as_millis());
}