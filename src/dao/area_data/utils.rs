use std::{path::PathBuf, time::UNIX_EPOCH};

use crate::{AreaError, AreaResult, CsvAreaData};

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

pub fn inner_csv_area_data() -> AreaResult<CsvAreaData> {
    let code_path = PathBuf::from(format!(
        "{}/data/2023-7-area-code.csv.gz",
        env!("CARGO_MANIFEST_DIR")
    ));
    let geo_path = PathBuf::from(format!(
        "{}/data/2023-7-area-geo.csv.gz",
        env!("CARGO_MANIFEST_DIR")
    ));
    Ok(CsvAreaData::new(
        crate::CsvAreaCodeData::from_inner_path(code_path, true)?,
        Some(crate::CsvAreaGeoData::from_inner_path(geo_path, true)?),
    ))
}
