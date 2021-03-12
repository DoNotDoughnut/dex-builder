use std::fs::File;
use std::io::Write;

pub fn build_dex(pokemon_dir: &str, move_dir: &str, save_file: &str) -> Result<(), Box<dyn std::error::Error>> {

    let mut file = File::create(save_file)?;

    let mut pokemon = Vec::new();
    let mut moves = Vec::new();

    if let Ok(readdir) = std::fs::read_dir(pokemon_dir) {
        for entry in readdir {
            if let Ok(entry) = entry {
                let path = entry.path();
                if let Ok(data) = std::fs::read_to_string(path) {
                    if let Ok(pokemon_entry) = toml::from_str(&data) {
                        pokemon.push(pokemon_entry);
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
            &firecore_pokedex::SerializedDex {
                pokemon,
                moves
            }
        )?
    )?;

    Ok(())

}