use std::io::Error;
use std::io::ErrorKind;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

#[derive(Clone, Copy)]
#[derive(PartialEq, Eq)]
#[derive(Debug)]
enum SaveType {
    Character,
    SharedStash,
    PlugYStash // future consideration
}

#[derive(Debug)]
struct Save<'a> {
    save_type: SaveType,
    path: &'a Path,
    data: Vec<u8>
}

fn detect_save_type(path: &Path) -> Result<SaveType, Box<dyn std::error::Error>> {
    match path.extension() {
        None => return Err(Box::new(Error::new(ErrorKind::InvalidData, "Save file has no extension"))),
        Some(ext) => {
            match ext.to_str() {
                None => return Err(Box::new(Error::new(ErrorKind::InvalidData, "Save file extension is not valid UTF-8"))),
                Some(ext) => {
                    match ext.to_lowercase().as_str() {
                        "d2s" => return Ok(SaveType::Character),
                        "d2i" => return Ok(SaveType::SharedStash),
                        // "sss" => return Ok(SaveType::PlugYStash),
                        // "d2x" => return Ok(SaveType::PlugYStash),
                        _ => return Err(Box::new(Error::new(ErrorKind::InvalidData, "Save file extension is not valid")))
                    }
                }
            }
        }
    }
}

fn validate_save_type(save_type: SaveType, data: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    match save_type {
        SaveType::Character => {
            match data.get(0x00..0x04) {
                None => return Err(Box::new(Error::new(ErrorKind::InvalidData, "Character save file is not the right type or is corrupted."))),
                Some(header) => {
                    if header != [0x55, 0xAA, 0x55, 0xAA] {
                        return Err(Box::new(Error::new(ErrorKind::InvalidData, "Character save file is not the correct size")));
                    }
                }
            }
        },
        SaveType::SharedStash => {
            match data.get(0x00..0x04) {
                None => return Err(Box::new(Error::new(ErrorKind::InvalidData, "Character save file is not the right type or is corrupted."))),
                Some(header) => {
                    if header != [0x55, 0xAA, 0x55, 0xAA] {
                        return Err(Box::new(Error::new(ErrorKind::InvalidData, "Character save file is not the correct size")));
                    }
                }
            }
        },
        _ => return Err(Box::new(Error::new(ErrorKind::InvalidData, "Save file type is not supported")))
    }

    return Ok(());
}

fn load_save_from_path(path: &Path) -> Result<Save, Box<dyn std::error::Error>> {
    let mut file = match File::open(path) {
        Err(err) => return Err(Box::new(err)),
        Ok(file) => file
    };

    let mut data = Vec::new();
    match file.read_to_end(&mut data) {
        Err(err) => return Err(Box::new(err)),
        Ok(_) => ()
    };

    let save_type = match detect_save_type(path) {
        Err(err) => return Err(err),
        Ok(save_type) => save_type
    };

    match validate_save_type(save_type, data.as_slice()) {
        Err(err) => return Err(err),
        Ok(_) => ()
    }

    let save = Save {
        save_type: save_type,
        path: path,
        data: data
    };

    return Ok(save);
}


#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_case {($fname:expr) => (
        concat!(env!("CARGO_MANIFEST_DIR"), "/resources/test/", $fname) // assumes Linux ('/')!
      )}

    #[test]
    fn test_load_save_from_path() {
        let p = Path::new(test_case!("test.d2s"));
        match load_save_from_path(p) {
            Err(err) => panic!("Error loading save file: {}", err),
            Ok(save) => {
                assert_eq!(save.save_type, SaveType::Character);
                assert_eq!(save.path, p);
                assert_eq!(save.data.len(), 2912);

            }
        }
    }
}
