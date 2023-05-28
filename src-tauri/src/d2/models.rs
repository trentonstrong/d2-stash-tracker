use std::collections::HashMap;
use serde::Deserialize;
use std::error::Error;
use serde_repr::Deserialize_repr;


#[derive(Deserialize, Debug)]
pub struct CharacterDataStatus {
    pub expansion: bool,
    pub died: bool,
    pub hardcore: bool,
    pub ladder: bool,
}

#[derive(Deserialize, Debug)]
pub struct CharacterDataHeader {
    pub identifier: String,
    pub name: String,
    pub level: u8,
    pub class: String,
    pub status: CharacterDataStatus,
    pub created: u32,
    pub last_played: u32,
}

pub type CharacterDataAttributes = HashMap<String, u32>;

#[repr(u8)]
#[derive(Deserialize_repr, PartialEq, Debug)]
pub enum ItemQuality {
    normal,
    exceptional,
    elite,
}


#[derive(Deserialize, Debug)]
pub struct WeaponDamage {
  mindam: Option<u8>,
  maxdam: Option<u8>,
  twohandmindam: Option<u8>,
  twohandmaxdam: Option<u8>,
}

#[derive(Deserialize, Debug)]
pub struct MagicProperty {
  id: u32,
  name: String,
  values: Vec<i32>,
  description: Option<String>,
  visible: Option<bool>,
  op_value: Option<u32>,
  op_stats: Option<Vec<String>>,
}

#[derive(Deserialize, Debug)]
pub struct ItemData {
  identified: u8,
  socketed: u8,
  new: u8,
  is_ear: u8,
  starter_item: u8,
  simple_item: u8,
  ethereal: u8,
  personalized: u8,
  personalized_name: Option<String>,
  given_runeword: u8,
  version: String,
  location_id: u8,
  equipped_id: u8,
  position_x: u8,
  position_y: u8,
  alt_position_id: u8,
  r#type: String,
  type_id: u8,
  type_name: String,
  quest_difficulty: Option<u8>,
  nr_of_items_in_sockets: u8,
  id: Option<u32>,
  level: Option<u8>,
  quality: Option<u8>,
  multiple_pictures: Option<u8>,
  picture_id: Option<u8>,
  class_specific: Option<u8>,
  low_quality_id: Option<u8>,
  timestamp: Option<u8>,
  defense_rating: Option<u16>,
  max_durability: Option<u16>,
  current_durability: Option<u16>,
  total_nr_of_sockets: Option<u8>,
  quantity: Option<u16>,
  magic_prefix: Option<u16>,
  magic_prefix_name: Option<String>,
  magic_suffix: Option<u16>,
  magic_suffix_name: Option<String>,
  runeword_id: Option<u16>,
  runeword_name: Option<String>,
  runeword_attributes: Option<Vec<MagicProperty>>,
  set_id: Option<u16>,
  set_name: Option<String>,
  set_list_count: Option<u8>,
  set_attributes: Option<Vec<Vec<MagicProperty>>>,
  set_attributes_num_req: Option<u8>,
  set_attributes_ids_req: Option<u8>,
  rare_name: Option<String>,
  rare_name2: Option<String>,
  magical_name_ids: Option<Vec<Option<u16>>>,
  unique_id: Option<u16>,
  unique_name: Option<String>,
  magic_attributes: Option<Vec<MagicProperty>>,
  combined_magic_attributes: Option<Vec<MagicProperty>>,
  socketed_items: Option<Vec<ItemData>>,
  base_damage: Option<WeaponDamage>,
  reqstr: Option<u8>,
  reqdex: Option<u8>,
  inv_width: u8,
  inv_height: u8,
  inv_file: String,
  inv_transform: Option<u8>,
  transform_color: Option<String>,
  item_quality: Option<ItemQuality>,
  categories: Vec<String>,
  file_index: Option<u8>,
  auto_affix_id: Option<u8>,
  rare_name_id: Option<u8>,
  rare_name_id2: Option<u8>,
  displayed_magic_attributes: Option<Vec<MagicProperty>>,
  displayed_runeword_attributes: Option<Vec<MagicProperty>>,
  displayed_combined_magic_attributes: Option<Vec<MagicProperty>>,
}

#[derive(Deserialize, Debug)]
pub struct CharacterData {
    pub header: CharacterDataHeader,
    pub attributes: CharacterDataAttributes,
    pub items: Vec<ItemData>,
    pub corpse_items: Vec<ItemData>,
    pub merc_items: Vec<ItemData>,
}

impl CharacterData {
  pub fn from_json_str(s: &str) -> Result<CharacterData, Box<dyn Error>> {
    return serde_json::from_str(s).map_err(|e| e.into());
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  macro_rules! test_case {($fname:expr) => (
    concat!(env!("CARGO_MANIFEST_DIR"), "/resources/test/", $fname) // assumes Linux ('/')!
  )}

  #[test]
  fn test_deserialize_character() {
    let test_character = include_bytes!(test_case!("test_character.json"));
    let character_data: CharacterData = match CharacterData::from_json_str(std::str::from_utf8(test_character).unwrap()) {
      Ok(character_data) => character_data,
      Err(err) => panic!("Error deserializing character: {}", err)
    };
  }
}
