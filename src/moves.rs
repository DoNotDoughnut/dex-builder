use serde::{de::DeserializeOwned, Serialize};
use hashbrown::HashMap;
use std::{
    fs::{read_dir, read_to_string},
    path::Path,
};

use pokedex::{
    moves::{Move, MoveId},
    Dex,
};

pub type Scripts = HashMap<MoveId, String>;

pub fn get_moves<U: DeserializeOwned + Serialize, P: AsRef<Path>>(moves: P) -> (Dex<Move<U>>, Scripts) {
    let move_dir = moves.as_ref();

    let moves = Dex::new(
        read_dir(move_dir)
            .unwrap_or_else(|err| {
                panic!(
                    "Could not read moves directory at {:?} with error {}",
                    move_dir, err
                )
            })
            .flat_map(|entry| match entry.map(|entry| entry.path()) {
                Ok(path) => {
                    match path.is_file() {
                        true => {
                            let m = ron::from_str::<Move<U>>(&read_to_string(&path).unwrap_or_else(|err| {
                                panic!(
                                    "Could not read move file at {:?} to string with error {}",
                                    path, err
                                )
                            }))
                            .unwrap_or_else(|err| {
                                panic!("Could not parse move file at {:?} with error {}", path, err)
                            });
                            Some((m.id, m))
                        }
                        false => None,
                    }
                }
                Err(err) => {
                    eprintln!("Could not read directory entry with error {}", err);
                    None
                }
            })
            .collect(),
    );

    let scripts = read_dir(move_dir.join("scripts"))
        .unwrap_or_else(|err| {
            panic!(
                "Could not read move scripts directory at {:?} with error {}",
                move_dir, err
            )
        })
        .flat_map(|entry| match entry {
            Ok(entry) => {
                let id = entry
                    .file_name()
                    .to_string_lossy()
                    .parse()
                    .unwrap_or_else(|err| {
                        panic!(
                            "Could not parse move script file {:?} into move ID with error {}",
                            entry.path(),
                            err
                        )
                    });
                let path = entry.path();
                Some((
                    id,
                    read_to_string(&path).unwrap_or_else(|err| {
                        panic!(
                            "Could not read move file at {:?} to string with error {}",
                            path, err
                        )
                    }),
                ))
            }
            Err(err) => {
                eprintln!("Could not read directory entry with error {}", err);
                None
            }
        })
        .collect();

    (moves, scripts)
}
