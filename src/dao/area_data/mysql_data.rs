use mysql::{prelude::Queryable, Conn, Opts, Row};

use crate::{AreaCodeData, AreaDataProvider, AreaError, AreaGeoData, AreaGeoDataItem, AreaResult};

impl From<mysql::Error> for AreaError {
    fn from(err: mysql::Error) -> Self {
        AreaError::DB(err.to_string())
    }
}
impl From<mysql::UrlError> for AreaError {
    fn from(err: mysql::UrlError) -> Self {
        AreaError::DB(err.to_string())
    }
}

pub struct MysqlAreaCodeData {
    pub uri: String,
    pub sql: String,
    pub column_name: String,
    pub column_code: String,
    pub column_hide: String,
    pub column_key_word: String,
    pub key_word_name: bool,
}

impl MysqlAreaCodeData {
    pub fn from_uri(uri: &str) -> Self {
        Self {
            uri: uri.to_owned(),
            sql: "select name,code,hide,key_word from area_code".to_string(),
            column_name: "name".to_string(),
            column_code: "code".to_string(),
            column_hide: "hide".to_string(),
            column_key_word: "key_word".to_string(),
            key_word_name: true,
        }
    }
}

pub struct MysqlAreaGeoData {
    pub uri: String,
    pub sql: String,
    pub column_code: String,
    pub column_center: String,
    pub column_polygon: String,
}

impl MysqlAreaGeoData {
    pub fn from_uri(uri: &str) -> Self {
        Self {
            uri:uri.to_owned(),
            sql: "select code,center,polygon from area_geo where code in ('0') or code like '______%'".to_string(),
            column_code: "code".to_string(),
            column_center: "center".to_string(),
            column_polygon: "polygon".to_string(),
        }
    }
}

pub struct MysqlAreaData {
    code_config: MysqlAreaCodeData,
    geo_config: Option<MysqlAreaGeoData>,
}
impl MysqlAreaData {
    pub fn new(code_config: MysqlAreaCodeData, geo_config: Option<MysqlAreaGeoData>) -> Self {
        Self {
            code_config,
            geo_config,
        }
    }
}

impl AreaDataProvider for MysqlAreaData {
    fn read_code_data(&self) -> AreaResult<Vec<AreaCodeData>> {
        let opt = Opts::try_from(self.code_config.uri.as_str())?;
        let mut conn = Conn::new(opt)?;
        let result: Vec<Row> = conn.query(self.code_config.sql.as_str())?;
        Ok(result
            .into_iter()
            .flat_map(|row| {
                if let Some(code) = row.get(self.code_config.column_code.as_str()) {
                    let hide = row.get(self.code_config.column_hide.as_str()).unwrap_or(0);
                    let name: Option<String> = row.get(self.code_config.column_name.as_str());
                    let keyword: String = row
                        .get(self.code_config.column_key_word.as_str())
                        .unwrap_or_default();
                    let mut key_word = keyword
                        .split(',')
                        .filter(|e| !e.is_empty())
                        .map(|e| e.to_owned())
                        .collect::<Vec<_>>();
                    if self.code_config.key_word_name {
                        if let Some(ename) = &name {
                            key_word.push(ename.to_owned());
                        }
                    }
                    Some(AreaCodeData {
                        code,
                        hide: hide == 1 || name.as_ref().map(|e| e.is_empty()).unwrap_or(true),
                        name: name.unwrap_or_else(|| "[_._]".to_string()),
                        key_word,
                    })
                } else {
                    None
                }
            })
            .collect::<Vec<_>>())
    }
    fn read_geo_data(&self) -> AreaResult<Vec<AreaGeoData>> {
        match &self.geo_config {
            Some(get_config) => {
                let opt = Opts::try_from(get_config.uri.as_str())?;
                let mut conn = Conn::new(opt)?;
                let result: Vec<Row> = conn.query(get_config.sql.as_str())?;
                Ok(result
                    .into_iter()
                    .flat_map(|row| {
                        if let Some(code) = row.get(get_config.column_code.as_str()) {
                            if let Some(polygon) = row.get(get_config.column_polygon.as_str()) {
                                return Some(AreaGeoData {
                                    code,
                                    item: vec![AreaGeoDataItem {
                                        center: row
                                            .get(get_config.column_center.as_str())
                                            .unwrap_or_default(),
                                        polygon,
                                    }],
                                });
                            }
                        }
                        None
                    })
                    .collect::<Vec<_>>())
            }
            None => Ok(vec![]),
        }
    }
    fn code_data_is_change(&self) -> bool {
        true
    }
    fn geo_data_is_change(&self) -> bool {
        true
    }
}
