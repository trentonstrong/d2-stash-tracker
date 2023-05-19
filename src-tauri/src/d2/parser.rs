use nom::{
  bytes::complete::{tag, take_while_m_n, take},
  number::complete::{le_u16, le_u32, le_u8},
  combinator::map_res,
  sequence::Tuple,
  IResult,
  Err,
};

struct Character<'a> {
  name: &'a str,
  version: u32,
  status: CharacterStatus,
  class: CharacterClass,
  level: u8,
  created_at: u32,
  last_played_at: u32,
}

type CharacterStatus = u8;
const CHARACTER_STATUS_HARDCORE: CharacterStatus = 1 << 2;
const CHARACTER_STATUS_DEAD: CharacterStatus = 1 << 3;
const CHARACTER_STATUS_EXPANSION: CharacterStatus = 1 << 5;
const CHARACTER_STATUS_LADDER: CharacterStatus = 1 << 6;


#[derive(Debug)]
#[derive(PartialEq, Eq)]
#[repr(u8)]
enum CharacterClass {
  Amazon = 0,
  Sorceress = 1,
  Necromancer = 2,
  Paladin = 3,
  Barbarian = 4,
  Druid = 5,
  Assassin = 6,
}

impl std::convert::TryFrom<u8> for CharacterClass {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
          0x00 => Ok(CharacterClass::Amazon),
          0x01 => Ok(CharacterClass::Sorceress),
          0x02 => Ok(CharacterClass::Necromancer),
          0x03 => Ok(CharacterClass::Paladin),
          0x04 => Ok(CharacterClass::Barbarian),
          0x05 => Ok(CharacterClass::Druid),
          0x06 => Ok(CharacterClass::Assassin),
          _ => Err("Invalid character class")
        }
    }
}

fn character_name(rest: &[u8]) -> IResult<&[u8], &str> {
  let (rest, name_rest) = take(16usize)(rest)?;
  let (_, name_parsed) = take_while_m_n(2, 15, |c: u8| c != 0x00)(name_rest)?;
  let name = std::str::from_utf8(name_parsed).unwrap();

  return Ok((rest, name));
}

fn character_current(data: &[u8], version: u32) -> IResult<&[u8], Character> {
    // start: 0x24
    let rest = &data[0x24..];

    // character status 0x24..0x25
    let (rest, status) = le_u8(rest)?;
    let status = status as CharacterStatus;

    // character progression 0x25..0x26
    let (rest, _) = le_u8(rest)?;

    // active arms 0x26..0x28
    let (rest, active_arms) = le_u16(rest)?;

    // character class 0x28..0x29
    let (rest, class) = le_u8(rest)?;
    let class = match CharacterClass::try_from(class) {
      Err(err) => panic!("{}", err), // TODO: Wrap error class
      Ok(class) => class,
    };

    // 2-byte padding 0x29..0x2B
    let (rest, _) = le_u16(rest)?;

    // character level 0x2B..0x2C
    let (rest, level) = le_u8(rest)?;

    // created 0x2C..0x30
    let (rest, created_at) = le_u32(rest)?;

    // last played 0x30..0x34
    let (rest, last_played_at) = le_u32(rest)?;

    // 4-byte padding 0x34..0x38
    let (rest, _) = le_u32(rest)?;

    // assigned skills 0x38..0x78
    let (rest, _) = take(40usize)(rest)?;

    // character name (0x10B)
    let (_, name) = character_name(&data[0x10B..])?;


    return Ok((
      &rest[0x00..],
      Character {
        name: name,
        class: class,
        version: version,
        status: status,
        level: level,
        created_at: created_at,
        last_played_at: last_played_at,
      },
    ));
}

fn character_from_bytes(data: &[u8]) -> IResult<&[u8], Character> {
  // magic number (0x00)
  let (rest, _) = tag([0x55, 0xAA, 0x55, 0xAA])(data)?;
  // version number (0x04)
  let (_, version) = le_u32(rest)?;
  println!("version: {:?}", version);
  if version > 0x61 {
    // we pass the entire buffer to keep offsets absolute
    return character_current(data, version);
  } else {
    unimplemented!("Version {:?} not supported", version)
  }
}

fn parse_character(rest: &[u8]) -> Result<Character, String> {
  let (_, character) = character_from_bytes(rest).map_err(|err| match err {
    Err::Error(err) => format!("Error parsing character: {:?}", err),
    Err::Failure(err) => format!("Failure parsing character: {:?}", err),
    Err::Incomplete(needed) => format!("Incomplete parsing character: {:?}", needed),
  })?;

  return Ok(character);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_parse_character_name() {
    let rest = include_bytes!("../../resources/test/test.d2s");
    let (_, name) = character_name(&rest[267..]).unwrap();
    assert_eq!(name, "Alina");
  }

  #[test]
  fn test_parse_character() {
    let rest = include_bytes!("../../resources/test/test.d2s");
    let character = parse_character(rest).unwrap();
    assert_eq!(character.name, "Alina");
    assert_eq!(character.class, CharacterClass::Sorceress);
    assert_eq!(character.version, 99);
    assert!(character.status & CHARACTER_STATUS_HARDCORE == 0);
    assert!(character.status & CHARACTER_STATUS_EXPANSION != 0);
    assert!(character.status & CHARACTER_STATUS_LADDER == 0);
    assert_eq!(character.level, 81);
  }
}
