use serde::{Deserialize, Serialize};
use std::fs;
use std::io;

use crate::items::Item;
use crate::map::Map;
use crate::monster::Monster;
use crate::player::Player;

#[derive(Serialize, Deserialize)]
pub struct SaveData {
    pub player: Player,
    pub map: Map,
    pub monsters: Vec<Monster>,
    pub ground_items: Vec<(usize, usize, Item)>,
    pub current_floor: i32,
    pub log: Vec<String>,
}

pub fn save_game(
    player: &Player,
    map: &Map,
    monsters: &[Monster],
    ground_items: &std::collections::HashMap<(usize, usize), Item>,
    current_floor: i32,
    log: &[String],
) -> io::Result<()> {
    let ground_items_vec: Vec<(usize, usize, Item)> = ground_items
        .iter()
        .map(|(&(x, y), item)| (x, y, item.clone()))
        .collect();

    let data = SaveData {
        player: player.clone(),
        map: map.clone(),
        monsters: monsters.to_vec(),
        ground_items: ground_items_vec,
        current_floor,
        log: log.to_vec(),
    };

    let json = serde_json::to_string(&data).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    fs::write("save.json", json)?;
    Ok(())
}

pub fn load_game() -> io::Result<SaveData> {
    let json = fs::read_to_string("save.json")?;
    let data: SaveData =
        serde_json::from_str(&json).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    Ok(data)
}

pub fn has_save() -> bool {
    std::path::Path::new("save.json").exists()
}
