use std::fs::copy;
use std::path::Path;

fn main() {

    let pokemon_dir = "pokedex/entries";
    let textures_dir = "pokedex/textures";

    let base = Path::new(textures_dir);
    let icon = base.join("icon");
    let base = base.join("normal");
    let new = Path::new("pokedex/pokemon");

    for entry in std::fs::read_dir(pokemon_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let name = path.file_stem().unwrap();

        // Create new folder

        let folder = new.join(name);

        if !folder.exists() {
            std::fs::create_dir_all(&folder).unwrap();
        }

        let image = name.to_string_lossy().to_ascii_lowercase() + ".png";

        copy(&path, folder.join(entry.file_name())).unwrap(); // Copy pokemon entry

        copy(base.join("front").join(&image), folder.join("normal_front.png")).unwrap(); // copy front texture
        copy(base.join("back").join(&image), folder.join("normal_back.png")).unwrap(); // copy back texture
        
        let mut icon_path = icon.join(&image);
        if !icon_path.exists() {
            if image.starts_with("deo") {
                icon_path = icon.join("deoxys.png");
            } else {
                icon_path = icon.join(image.replace("-", "_"));
            }
        };

        copy(icon_path, folder.join("icon.png")).unwrap();


        // let path = entry.path();
        // // let data = std::fs::read_to_string(path)?;
        // // let pokemon_entry: Pokemon = toml::from_str(&data)?;
        // let name = pokemon_entry.data.name.to_ascii_lowercase();
        // println!("{}: {:?}", name, entry.path().file_stem().unwrap());
        // let mut front_png = Vec::new();
        // let mut back_png = Vec::new();
        // let mut icon_png = Vec::new();
        // let img = name.clone() + ".png";
        // println!("{:?}", icon.join(&img));
        // File::open()?.read_to_end(&mut front_png)?;
        // File::open(base.join("back").join(&img))?.read_to_end(&mut back_png)?;
        // match File::open(icon.join(&img)) {
        //     Ok(mut file) => {
        //         file.read_to_end(&mut icon_png)?;
        //     }
        //     Err(_) => {
        //         if name.starts_with("deo") {
        //             File::open(icon.join("deoxys.png"))?.read_to_end(&mut icon_png)?;
        //         } else {
        //             File::open()?.read_to_end(&mut icon_png)?;
        //         }
        //     }
        // }
        
    }

    // Ok(())

}