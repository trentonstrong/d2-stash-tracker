use diesel::{prelude::*};
use diesel::sqlite::SqliteConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use log::{trace};

use crate::d2::models::CharacterData;
use crate::models::{NewCharacter, Character};

pub mod d2;
pub mod models;
pub mod schema;

const DEFAULT_DATABASE_PATH: &str = "d2rsm.db";
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations"); // relative to crate root

pub type ConnectionPool = Pool<ConnectionManager<SqliteConnection>>;

#[derive(Debug)]
pub enum ImportStatus {
  Created,
  Updated,
}

impl std::fmt::Display for ImportStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub struct ImportResult {
  pub character: Character,
  pub status: ImportStatus,
}

pub fn get_connection_pool(db_path: Option<&str>) -> ConnectionPool {
    let db_path = db_path.unwrap_or(DEFAULT_DATABASE_PATH);
    let manager = ConnectionManager::<SqliteConnection>::new(db_path);
    Pool::builder().build(manager).expect("Failed to create pool.")
}

pub fn run_pending_migrations(conn: &mut SqliteConnection) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    trace!("Running pending migrations...");
    conn.run_pending_migrations(MIGRATIONS)?;
    trace!("Done running pending migrations.");

    return Ok(());
}

fn get_character<'a>(conn: &'a mut SqliteConnection, character_name: &'a str) -> Result<Option<Character>, Box<dyn std::error::Error>> {
    use crate::schema::characters::dsl::*;
    let character = characters
      .filter(name.eq(character_name))
      .select(Character::as_select())
      .first::<Character>(conn)
      .optional()?;

    return Ok(character);
}

fn insert_character(conn: &mut SqliteConnection, character: &NewCharacter) -> Result<Character, Box<dyn std::error::Error>> {
    use crate::schema::characters;
    let inserted_character = diesel::insert_into(characters::table)
      .values(character)
      .returning(Character::as_returning())
      .get_result(conn)?;

    return Ok(inserted_character);
}

pub fn import_character(conn: &mut SqliteConnection, character_json: &str) -> Result<ImportResult, Box<dyn std::error::Error>> {
    trace!("importing character: {}", character_json);
    let character_data = CharacterData::from_json_str(character_json)?;
    let new_character = NewCharacter::from_character_data(character_data);
    trace!("new_character: {:?}", new_character);

    let existing_character = get_character(conn, &new_character.name)?;
    let status = if existing_character.is_some() { ImportStatus::Updated } else { ImportStatus::Created };
    let character = if let Some(character) = existing_character {
      character
    } else {
      insert_character(conn, &new_character)?
    };

    return Ok(ImportResult {
      character,
      status: status,
    });
}

#[cfg(test)]
mod tests {
  use super::*;

  fn open_connection() -> SqliteConnection {
    let mut conn = SqliteConnection::establish(":memory:").unwrap();
    (&mut conn).run_pending_migrations(MIGRATIONS).unwrap();

    return conn;
  }

  #[test]
  fn test_import_character() {
    let test_character = include_bytes!("../resources/test/test_character.json");
    let mut conn = open_connection();
    let result = import_character(&mut conn, std::str::from_utf8(test_character).unwrap());
    println!("result: {:?}", result);

    assert!(result.is_ok());
  }
}
