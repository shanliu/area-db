use std::ops::Deref;

use csv::{ReaderBuilder, StringRecord};

use crate::{AreaCodeData, AreaDataProvider, AreaError, AreaGeoData, AreaGeoDataItem, AreaResult};

pub enum CsvData<'c> {
    Data(String),
    DataRef(&'c str),
}
impl<'c> Deref for CsvData<'c> {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        match self {
            CsvData::Data(data) => data.as_str(),
            CsvData::DataRef(data) => data,
        }
    }
}

pub struct CsvAreaCodeData<'c> {
    pub csv_data: CsvData<'c>,
    pub skip: usize,              //跳过头部n行
    pub column_name: u8,          //城市完整名字段，从0开始
    pub column_code: u8,          //城市编码字段，从0开始
    pub column_hide: u8,          //是否隐藏字段，可忽略
    pub column_key_word: Vec<u8>, //搜索关键字字段，可忽略
}

impl<'c> CsvAreaCodeData<'c> {
    pub fn data(csv_data: CsvData<'c>) -> Self {
        Self {
            csv_data,
            skip: 0,
            column_name: 1,
            column_code: 0,
            column_hide: 4,
            column_key_word: vec![2, 3],
        }
    }
    #[cfg(feature = "data-csv-embed-code")]
    pub fn inner_data() -> AreaResult<Self> {
        use std::io::Read;
        let zip_data: &[u8] = include_bytes!("../../../data/2023-7-area-code.csv.gz");
        let mut gz = flate2::read::GzDecoder::new(zip_data);
        let mut s = String::new();
        gz.read_to_string(&mut s)
            .map_err(|e| AreaError::System(e.to_string()))?;
        Ok(Self::data(CsvData::Data(s)))
    }
    pub fn path(path: &str) -> AreaResult<Self> {
        let file_contents =
            std::fs::read_to_string(path).map_err(|e| AreaError::System(e.to_string()))?;
        Ok(Self::data(CsvData::Data(file_contents)))
    }
}

pub struct CsvAreaGeoData<'c> {
    pub csv_data: CsvData<'c>,
    pub skip: usize,          //跳过头部几行
    pub column_code: u8,      //城市编码字段，从0开始
    pub column_center: u8,    //城市中心坐标字段，不存在时从范围中取中心，从0开始
    pub column_polygon: u8,   //坐标范围字段，从0开始
    pub code_len: Vec<usize>, //CODE 长度限制
}

impl<'c> CsvAreaGeoData<'c> {
    pub fn data(csv_data: CsvData<'c>) -> Self {
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
    pub fn inner_data() -> AreaResult<Self> {
        use std::io::Read;
        let zip_data: &[u8] = include_bytes!("../../../data/2023-7-area-geo.csv.gz");
        let mut gz = flate2::read::GzDecoder::new(zip_data);
        let mut s = String::new();
        gz.read_to_string(&mut s)
            .map_err(|e| AreaError::System(e.to_string()))?;
        Ok(Self::data(CsvData::Data(s)))
    }
    pub fn path(path: &str) -> AreaResult<Self> {
        let file_contents =
            std::fs::read_to_string(path).map_err(|e| AreaError::System(e.to_string()))?;
        Ok(Self::data(CsvData::Data(file_contents)))
    }
}

pub struct CsvAreaData<'c> {
    code_config: CsvAreaCodeData<'c>,
    geo_config: Option<CsvAreaGeoData<'c>>,
}
impl<'c> CsvAreaData<'c> {
    pub fn new(code_config: CsvAreaCodeData<'c>, geo_config: Option<CsvAreaGeoData<'c>>) -> Self {
        Self {
            code_config,
            geo_config,
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
impl<'c> AreaDataProvider for CsvAreaData<'c> {
    fn read_code_data(&self) -> AreaResult<Vec<AreaCodeData>> {
        self.read_data(&self.code_config.csv_data, self.code_config.skip, |row| {
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
            Some(get_config) => {
                let out = self.read_data(&get_config.csv_data, get_config.skip, |row| {
                    if let Some(code) = row.get(get_config.column_code as usize) {
                        if !get_config.code_len.contains(&code.len()) {
                            return None;
                        }
                        let center = row
                            .get(get_config.column_center as usize)
                            .unwrap_or("")
                            .to_owned();
                        let polygon = row
                            .get(get_config.column_polygon as usize)
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
}
