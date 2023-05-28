use chrono::{NaiveDateTime};
use diesel::prelude::*;
use crate::schema::characters;
use crate::d2::models::CharacterData;

#[derive(Insertable, Debug)]
#[diesel(table_name = characters)]
pub struct NewCharacter {
    pub name: String,
    pub level: i32,
    pub class: String,
    pub is_expansion: bool,
    pub has_died: bool,
    pub is_hardcore: bool,
    pub is_ladder: bool,
    pub saved_at: NaiveDateTime,
}

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = characters)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Character {
    pub id: i32,
    pub name: String,
    pub level: i32,
    pub class: String,
    pub is_expansion: bool,
    pub has_died: bool,
    pub is_hardcore: bool,
    pub is_ladder: bool,
    pub saved_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl NewCharacter {
  pub fn from_character_data(character_data: CharacterData) -> NewCharacter {
    let header = character_data.header;
    let status = header.status;
    let saved_at = NaiveDateTime::from_timestamp_millis(header.last_played as i64)
      .unwrap_or_default();

    return NewCharacter {
      name: header.name,
      level: header.level as i32,
      class: header.class,
      is_expansion: status.expansion,
      has_died: status.died,
      is_hardcore: status.hardcore,
      is_ladder: status.ladder,
      saved_at: saved_at,
    };
  }
}

#[cfg(test)]
mod tests {
  use super::*;

}
