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
