use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;

use firecore_pokedex::pokemon::Pokemon;
use firecore_pokedex::serialized::SerializedPokemon;

pub fn build_dex(pokemon_dir: &str, move_dir: &str, textures_dir: &str, save_file: &str) -> Result<(), Box<dyn std::error::Error>> {

    let mut file = File::create(save_file)?;

    let mut pokemon = Vec::new();
    let mut moves = Vec::new();

    if let Ok(readdir) = std::fs::read_dir(pokemon_dir) {
        for entry in readdir {
            if let Ok(entry) = entry {
                let path = entry.path();
                if let Ok(data) = std::fs::read_to_string(path) {
                    let result: Result<Pokemon, toml::de::Error> = toml::from_str(&data);
                    if let Ok(pokemon_entry) = result {
                        let name = pokemon_entry.data.name.to_ascii_lowercase();
                        let mut front_png = Vec::new();
                        let mut back_png = Vec::new();
                        let base = Path::new(textures_dir).join("normal");
                        File::open(base.join("front").join(name.clone() + ".png"))?.read_to_end(&mut front_png)?;
                        File::open(base.join("back").join(name + ".png"))?.read_to_end(&mut back_png)?;
                        pokemon.push(SerializedPokemon {
                            pokemon: pokemon_entry,
                            cry_ogg: Vec::new(),
                            front_png,
                            back_png,
                        });
                    }
                }
            }
        }
    }

    if let Ok(readdir) = std::fs::read_dir(move_dir) {
        for entry in readdir {
            if let Ok(entry) = entry {
                let path = entry.path();
                if let Ok(data) = std::fs::read_to_string(path) {
                    if let Ok(pokemon_move) = toml::from_str(&data) {
                        moves.push(pokemon_move);
                    }
                }
            }
        }
    }

    file.write_all(
        &bincode::serialize(
            &firecore_pokedex::serialized::SerializedDex {
                pokemon,
                moves
            }
        )?
    )?;

    Ok(())

}