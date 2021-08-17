use std::{
    fs::{read_dir, read_to_string},
    path::Path,
};

use battle::pokedex::{item::Item, Dex};

pub fn get_items<P: AsRef<Path>>(path: P) -> Dex<Item> {
    let path = path.as_ref();
    Dex::new(read_dir(path)
        .unwrap_or_else(|err| {
            panic!(
                "Could not read item directory at {:?} with error {}",
                path, err
            )
        })
        .flatten()
        .map(|entry| {
            let path = entry.path();
            let i = ron::from_str::<Item>(&read_to_string(&path).unwrap_or_else(|err| {
                panic!(
                    "Could not read item entry at {:?} to string with error {}",
                    path, err
                )
            }))
            .unwrap_or_else(|err| {
                panic!(
                    "Could not deserialize item entry at {:?} with error {}",
                    path, err
                )
            });
            (i.id, i)
        })
        .collect())
}
