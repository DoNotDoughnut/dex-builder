use pokedex::moves::Move;
use std::{
    fs::{read_dir, read_to_string},
    path::Path,
};

pub fn get_moves<P: AsRef<Path>>(moves: P) -> Vec<Move> {
    let move_dir = moves.as_ref();
    read_dir(move_dir)
        .unwrap_or_else(|err| {
            panic!(
                "Could not read moves directory at {:?} with error {}",
                move_dir, err
            )
        })
        .map(|entry| match entry.map(|entry| entry.path()) {
            Ok(path) => Some(
                ron::from_str(&read_to_string(&path).unwrap_or_else(|err| {
                    panic!(
                        "Could not read move file at {:?} to string with error {}",
                        path, err
                    )
                }))
                .unwrap_or_else(|err| {
                    panic!("Could not parse move file at {:?} with error {}", path, err)
                }),
            ),
            Err(err) => {
                eprintln!("Could not read directory entry with error {}", err);
                None
            }
        })
        .flatten()
        .collect()
}
