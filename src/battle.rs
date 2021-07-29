use std::{
    fs::{read_dir, read_to_string},
    path::Path,
};

use pokedex::battle_move::serialized::{SerializedBattleMoveBytes, SerializedBattleMoveFile};

pub fn get_battle_moves<P: AsRef<Path>>(battle: P) -> Vec<SerializedBattleMoveBytes> {
    read_dir(battle.as_ref())
        .unwrap_or_else(|err| panic!("Could not read battle moves directory at {:?} with error {}", battle.as_ref(), err))
        .flatten()
        .filter(|d| d.path().is_dir())
        .flat_map(|d| {
            let path = d.path();
            read_dir(&path).unwrap_or_else(|err| panic!("Could not read battle move directory at {:?} with error {}", path, err))
        })
        .flatten()
        .map(|d| {
            let path = d.path();
            let string = read_to_string(&path).unwrap_or_else(|err| {
                panic!(
                    "Could not read battle move at {:?} to string with error {}",
                    path, err
                )
            });
            ron::from_str::<SerializedBattleMoveFile>(&string)
                .unwrap_or_else(|err| {
                    panic!(
                        "Could not parse move battle file at {:?} with error {}",
                        path, err
                    )
                })
                .into(path)
        }).collect()
}
