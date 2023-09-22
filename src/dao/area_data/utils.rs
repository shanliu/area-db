use std::{path::PathBuf, time::UNIX_EPOCH};

use crate::{AreaError, AreaResult};

#[allow(dead_code)]
pub(crate) fn read_file_modified_time(path: &PathBuf) -> Option<u64> {
    if let Ok(metadata) = std::fs::metadata(path) {
        if let Ok(modified_time) = metadata.modified() {
            Some(
                modified_time
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs(),
            )
        } else {
            None
        }
    } else {
        None
    }
}
#[allow(dead_code)]
pub(crate) fn read_file(path: &PathBuf) -> AreaResult<Vec<u8>> {
    std::fs::read(path).map_err(|e| AreaError::System(e.to_string()))
}

pub(crate) fn de_gz_data(zip_data: Vec<u8>) -> AreaResult<Vec<u8>> {
    let mut s = vec![];
    use std::io::Read;
    let mut gz = flate2::read::GzDecoder::new(&zip_data[..]);
    gz.read_to_end(&mut s)
        .map_err(|e| AreaError::System(e.to_string()))?;
    Ok(s)
}
pub(crate) fn en_name_keyword(input: &str) -> String {
    let mut result = String::new();
    let mut prev_char = ' ';

    for c in input.chars() {
        if c.is_uppercase() {
            if !prev_char.is_whitespace() {
                result.push(' ');
            }
            result.push(c.to_ascii_lowercase());
        } else {
            result.push(c);
        }
        prev_char = c;
    }

    result
}
