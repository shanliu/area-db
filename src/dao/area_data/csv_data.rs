use std::path::PathBuf;

use csv::{ReaderBuilder, StringRecord};
use parking_lot::Mutex;

use crate::{AreaCodeData, AreaDataProvider, AreaError, AreaGeoData, AreaGeoDataItem, AreaResult};

use super::utils::{read_file_modified_time, read_file_to_string};
impl From<std::io::Error> for AreaError {
    fn from(err: std::io::Error) -> Self {
        AreaError::DB(err.to_string())
    }
}
pub struct CsvAreaCodeData {
    csv_data: Option<PathBuf>,
    skip: usize,              //跳过头部n行
    column_name: u8,          //城市完整名字段，从0开始
    column_code: u8,          //城市编码字段，从0开始
    column_hide: u8,          //是否隐藏字段，可忽略
    column_key_word: Vec<u8>, //搜索关键字字段，可忽略
}

impl CsvAreaCodeData {
    pub fn from_path(
        csv_data: PathBuf,
        skip: usize,
        column_name: u8,
        column_code: u8,
        column_hide: u8,
        column_key_word: Vec<u8>,
    ) -> Self {
        Self {
            csv_data: Some(csv_data),
            skip,
            column_name,
            column_code,
            column_hide,
            column_key_word,
        }
    }
    fn create_inner_data(csv_data: Option<PathBuf>) -> Self {
        Self {
            csv_data,
            skip: 1,
            column_code: 0,
            column_name: 1,
            column_hide: 4,
            column_key_word: vec![2, 3],
        }
    }
    pub fn from_inner_path(csv_data: PathBuf) -> AreaResult<Self> {
        std::fs::metadata(&csv_data)?;
        Ok(Self::create_inner_data(Some(csv_data)))
    }
    #[cfg(feature = "data-csv-embed-code")]
    pub fn from_inner_data() -> AreaResult<Self> {
        Ok(Self::create_inner_data(None))
    }
}

pub struct CsvAreaGeoData {
    csv_data: Option<PathBuf>,
    skip: usize,          //跳过头部几行
    column_code: u8,      //城市编码字段，从0开始
    column_center: u8,    //城市中心坐标字段，不存在时从范围中取中心，从0开始
    column_polygon: u8,   //坐标范围字段，从0开始
    code_len: Vec<usize>, //CODE 长度限制
}

impl CsvAreaGeoData {
    pub fn from_path(
        csv_data: PathBuf,
        skip: usize,
        column_code: u8,
        column_center: u8,
        column_polygon: u8,
        code_len: Vec<usize>,
    ) -> Self {
        Self {
            csv_data: Some(csv_data),
            skip,
            column_code,
            column_center,
            column_polygon,
            code_len,
        }
    }
    fn create_inner_data(csv_data: Option<PathBuf>) -> Self {
        Self {
            csv_data,
            skip: 1,
            column_code: 0,
            column_center: 1,
            column_polygon: 2,
            code_len: vec![1, 6],
        }
    }
    #[cfg(feature = "data-csv-embed-geo")]
    pub fn from_inner_data() -> AreaResult<Self> {
        Ok(Self::create_inner_data(None))
    }
    pub fn from_inner_path(csv_data: PathBuf) -> AreaResult<Self> {
        std::fs::metadata(&csv_data)?;
        Ok(Self::create_inner_data(Some(csv_data)))
    }
}

pub struct CsvAreaData {
    code_config: CsvAreaCodeData,
    geo_config: Option<CsvAreaGeoData>,
    code_file_time: Mutex<Option<u64>>,
    geo_file_time: Mutex<Option<u64>>,
}
impl CsvAreaData {
    pub fn new(code_config: CsvAreaCodeData, geo_config: Option<CsvAreaGeoData>) -> Self {
        Self {
            code_config,
            geo_config,
            code_file_time: Mutex::new(None),
            geo_file_time: Mutex::new(None),
        }
    }
    fn read_data<T>(
        &self,
        csv_data: &str,
        skip: usize,
        f: impl Fn(&StringRecord) -> Option<T>,
    ) -> AreaResult<Vec<T>> {
        let mut rdr = ReaderBuilder::new()
            .has_headers(false)
            .from_reader(csv_data.as_bytes());
        let mut out = vec![];
        for (i, result) in rdr.records().enumerate() {
            if i < skip {
                continue;
            }
            let record = result.map_err(|e| AreaError::DB(e.to_string()))?;
            if let Some(item) = f(&record) {
                out.push(item)
            }
        }
        Ok(out)
    }
}
impl AreaDataProvider for CsvAreaData {
    fn read_code_data(&self) -> AreaResult<Vec<AreaCodeData>> {
        let csv_data = match &self.code_config.csv_data {
            Some(path) => {
                let u = read_file_modified_time(path);
                *self.code_file_time.lock() = u;
                read_file_to_string(path)?
            }
            None => {
                let mut s = String::new();
                #[cfg(feature = "data-csv-embed-code")]
                {
                    use std::io::Read;
                    let zip_data: &[u8] = include_bytes!("../../../data/2023-7-area-code.csv.gz");
                    let mut gz = flate2::read::GzDecoder::new(zip_data);
                    gz.read_to_string(&mut s)
                        .map_err(|e| AreaError::System(e.to_string()))?;
                }
                s
            }
        };
        self.read_data(&csv_data, self.code_config.skip, |row| {
            if let Some(code) = row.get(self.code_config.column_code as usize) {
                if code.is_empty() {
                    return None;
                }
                let hide = row.get(self.code_config.column_hide as usize).unwrap_or("");
                let name = row
                    .get(self.code_config.column_name as usize)
                    .unwrap_or_default();
                let mut key_word = vec![];
                for tmp in self.code_config.column_key_word.iter() {
                    let keyword = row.get(*tmp as usize).unwrap_or("");
                    let tmp = keyword
                        .split('|')
                        .filter(|e| !e.is_empty())
                        .map(|e| e.to_owned())
                        .collect::<Vec<_>>();
                    key_word.extend(tmp);
                }
                let item = AreaCodeData {
                    code: code.to_owned(),
                    hide: hide == "1" || name.is_empty(),
                    name: name.to_owned(),
                    key_word,
                };
                Some(item)
            } else {
                None
            }
        })
    }
    fn read_geo_data(&self) -> AreaResult<Vec<AreaGeoData>> {
        match &self.geo_config {
            Some(geo_config) => {
                let csv_data = match &geo_config.csv_data {
                    Some(path) => {
                        let u = read_file_modified_time(path);
                        *self.geo_file_time.lock() = u;
                        read_file_to_string(path)?
                    }
                    None => {
                        let mut s = String::new();
                        #[cfg(feature = "data-csv-embed-geo")]
                        {
                            use std::io::Read;
                            let zip_data: &[u8] =
                                include_bytes!("../../../data/2023-7-area-geo.csv.gz");
                            let mut gz = flate2::read::GzDecoder::new(zip_data);
                            gz.read_to_string(&mut s)
                                .map_err(|e| AreaError::System(e.to_string()))?;
                        }
                        s
                    }
                };
                let out = self.read_data(&csv_data, geo_config.skip, |row| {
                    if let Some(code) = row.get(geo_config.column_code as usize) {
                        if !geo_config.code_len.contains(&code.len()) {
                            return None;
                        }
                        let center = row
                            .get(geo_config.column_center as usize)
                            .unwrap_or("")
                            .to_owned();
                        let polygon = row
                            .get(geo_config.column_polygon as usize)
                            .unwrap_or("")
                            .to_owned();
                        return Some(AreaGeoData {
                            code: code.to_owned(),
                            item: vec![AreaGeoDataItem { center, polygon }],
                        });
                    }
                    None
                })?;
                Ok(out)
            }
            None => Ok(vec![]),
        }
    }
    fn code_data_is_change(&self) -> bool {
        if let Some(path) = &self.code_config.csv_data {
            if let Some(lt) = read_file_modified_time(path) {
                if let Some(st) = *self.code_file_time.lock() {
                    return lt > st;
                }
            }
        }
        false
    }
    fn geo_data_is_change(&self) -> bool {
        if let Some(geo_config) = &self.geo_config {
            if let Some(path) = &geo_config.csv_data {
                if let Some(lt) = read_file_modified_time(path) {
                    if let Some(st) = *self.code_file_time.lock() {
                        return lt > st;
                    }
                }
            }
        }
        false
    }
}
