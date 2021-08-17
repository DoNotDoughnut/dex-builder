pub extern crate firecore_battle as battle;

use hashbrown::HashMap;
use std::path::Path;

use battle::{
    moves::Move,
    pokedex::{item::Item, moves::MoveId, pokemon::Pokemon, Dex},
};

pub mod items;
pub mod moves;
pub mod pokemon;

#[cfg(feature = "gen")]
pub mod gen;

pub fn compile<P: AsRef<Path>>(
    pokemon: P,
    moves: P,
    items: P,
) -> (Dex<Pokemon>, Dex<Move>, Dex<Item>, HashMap<MoveId, String>) {
    // #[cfg(feature = "gen")]
    // gen::gen(pokemon_dir, move_dir)

    println!("Loading pokemon...");
    let pokemon = pokemon::get_pokemon(pokemon);
    println!("Loading moves...");
    let (moves, scripts) = moves::get_moves(moves);
    println!("Loading items...");
    let items = items::get_items(items);

    (pokemon, moves, items, scripts)
}
