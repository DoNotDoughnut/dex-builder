pub extern crate firecore_pokedex as pokedex;

use serde::{de::DeserializeOwned, Serialize};
use std::path::Path;

use pokedex::{item::Item, moves::Move, pokemon::Pokemon, Dex};

pub mod items;
pub mod moves;
pub mod pokemon;

pub use moves::Scripts;

pub fn compile<U: DeserializeOwned + Serialize, P: AsRef<Path>>(
    pokemon: P,
    moves: P,
    items: P,
) -> (Dex<Pokemon>, Dex<Move<U>>, Dex<Item>, Scripts) {
    // #[cfg(feature = "gen")]
    // gen::gen(pokemon_dir, move_dir)

    println!("Loading pokemon...");
    let pokemon = pokemon::get_pokemon(pokemon);
    println!("Loading moves...");
    let (moves, scripts) = moves::get_moves::<U, P>(moves);
    println!("Loading items...");
    let items = items::get_items(items);

    (pokemon, moves, items, scripts)
}
