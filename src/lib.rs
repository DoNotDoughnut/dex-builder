pub extern crate bincode as ser;
pub extern crate firecore_pokedex_engine as pokedex;

use std::{fs::write, path::Path};

use pokedex::serialize::SerializedDex;

pub mod battle;
pub mod items;
pub mod moves;
pub mod pokemon;
pub mod trainers;

// #[cfg(feature = "gen")]
pub mod gen;

pub fn compile<P: AsRef<Path>>(
    pokemon: P,
    moves: P,
    items: P,
    trainers: P,
    battle: Option<P>,
    output: Option<P>,
) -> SerializedDex {
    // #[cfg(feature = "gen")]
    // gen::gen(pokemon_dir, move_dir)

    println!("Loading pokemon...");
    let pokemon = pokemon::get_pokemon(pokemon);
    println!("Loading moves...");
    let moves = moves::get_moves(moves);
    println!("Loading items...");
    let items = items::get_items(items);
    println!("Loading trainer textures...");
    let trainers = trainers::get_trainers(trainers);
    let battle_moves = match battle {
        Some(battle) => {
            println!("Loading battle moves...");
            battle::get_battle_moves(battle)
        }
        None => Vec::new()
    };

    let dex = SerializedDex {
        pokemon,
        moves,
        items,
        trainers,
        battle_moves,
    };

    if let Some(output) = output {
        let output = output.as_ref();
        let contents = bincode::serialize(&dex)
            .unwrap_or_else(|err| panic!("Could not serialize pokedex with error {}", err));
        write(output, contents).unwrap_or_else(|err| {
            panic!(
                "Could not write serialized dex to output file at {:?} with error {}",
                output, err
            )
        });
    }

    dex
}
