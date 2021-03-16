use std::fs::File;
use std::fs::read_dir;
use std::fs::read_to_string;
use std::io::Read;
use std::io::Write;
use std::path::Path;

use firecore_pokedex::pokemon::Pokemon;
use firecore_pokedex::serialized::SerializedPokemon;

pub fn build_dex(pokemon_dir: &str, move_dir: &str, textures_dir: &str, save_file: &str) -> Result<(), Box<dyn std::error::Error>> {

    let mut file = File::create(save_file)?;

    let mut pokemon = Vec::new();
    let mut moves = Vec::new();

    let base = Path::new(textures_dir);
    let icon = base.join("icon");
    let base = base.join("normal");
    for entry in read_dir(pokemon_dir)? {
        let path = entry?.path();
        let data = read_to_string(path)?;
        let pokemon_entry: Pokemon = toml::from_str(&data)?;
        let name = pokemon_entry.data.name.to_ascii_lowercase();
        let mut front_png = Vec::new();
        let mut back_png = Vec::new();
        let mut icon_png = Vec::new();
        let img = name.clone() + ".png";
        println!("{:?}", icon.join(&img));
        File::open(base.join("front").join(&img))?.read_to_end(&mut front_png)?;
        File::open(base.join("back").join(&img))?.read_to_end(&mut back_png)?;
        match File::open(icon.join(&img)) {
            Ok(mut file) => {
                file.read_to_end(&mut icon_png)?;
            }
            Err(_) => {
                if name.starts_with("deo") {
                    File::open(icon.join("deoxys.png"))?.read_to_end(&mut icon_png)?;
                } else {
                    File::open(icon.join(img.replace("-", "_")))?.read_to_end(&mut icon_png)?;
                }
            }
        }
        
        
        pokemon.push(SerializedPokemon {
            pokemon: pokemon_entry,
            cry_ogg: Vec::new(),
            front_png,
            back_png,
            icon_png,
        });
    }

    for entry in read_dir(move_dir)? {
        let path = entry?.path();
        let data = read_to_string(path)?;
        let pokemon_move = toml::from_str(&data)?;
        moves.push(pokemon_move);
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

pub fn build_icon_directory(pret_input: &str, output_dir: &str) {
    let path = Path::new(output_dir);
    if !path.exists() {
        std::fs::create_dir_all(path).unwrap();
    }
    for entry in read_dir(pret_input).unwrap() {
        let entry = entry.unwrap();
        let name = entry.file_name().to_string_lossy().to_string();
        let path = entry.path();
        if path.is_dir() {
            let input = path.join("icon.png");
            let output = Path::new(output_dir).join(name + ".png");
            println!("Input: {:?}", input);
            println!("Output: {:?}", output);
            if let Err(err) = std::fs::copy(&input, &output) {
                eprintln!("Could not copy {:?} to {:?} with error {}", input, output, err);
            }
        }        
    }
    std::fs::copy(Path::new(pret_input).join("unown").join("e").join("icon.png"), Path::new(output_dir).join("unown.png")).unwrap();
}