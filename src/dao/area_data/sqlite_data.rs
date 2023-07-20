use std::path::PathBuf;

use log::warn;
use parking_lot::Mutex;
use rusqlite::{Connection, Row};

use crate::{AreaCodeData, AreaDataProvider, AreaError, AreaGeoData, AreaGeoDataItem, AreaResult};

use super::utils::read_file_modified_time;

impl From<rusqlite::Error> for AreaError {
    fn from(err: rusqlite::Error) -> Self {
        AreaError::DB(err.to_string())
    }
}

pub struct SqliteAreaCodeData {
    pub conn: PathBuf,
    pub sql: String,
    pub column_name: String,
    pub column_code: String,
    pub column_hide: String,
    pub column_key_word: String,
    pub key_word_name: bool,
}

impl SqliteAreaCodeData {
    pub fn from_path(conn: PathBuf) -> Self {
        Self {
            conn,
            sql: "select name,code,kw_name||','||kw_py as key_word,hide from area_code".to_string(),
            column_name: "name".to_string(),
            column_code: "code".to_string(),
            column_hide: "hide".to_string(),
            column_key_word: "key_word".to_string(),
            key_word_name: true,
        }
    }
}

pub struct SqliteAreaGeoData {
    pub conn: PathBuf,
    pub sql: String,
    pub column_code: String,
    pub column_center: String,
    pub column_polygon: String,
}

impl SqliteAreaGeoData {
    pub fn from_path(conn: PathBuf) -> Self {
        Self {
            conn,
            sql: "select code,center,polygon from area_geo where code ='0' or code like '______%'"
                .to_string(),
            column_code: "code".to_string(),
            column_center: "center".to_string(),
            column_polygon: "polygon".to_string(),
        }
    }
}

pub struct SqliteAreaData {
    code_config: SqliteAreaCodeData,
    geo_config: Option<SqliteAreaGeoData>,
    code_file_time: Mutex<Option<u64>>,
    geo_file_time: Mutex<Option<u64>>,
}
impl SqliteAreaData {
    pub fn new(code_config: SqliteAreaCodeData, geo_config: Option<SqliteAreaGeoData>) -> Self {
        Self {
            code_config,
            geo_config,
            code_file_time: Mutex::new(None),
            geo_file_time: Mutex::new(None),
        }
    }
    fn read_data<T>(
        &self,
        conn: &Connection,
        sql: &str,
        f: impl for<'r> Fn(&'r Row<'r>) -> Option<T>,
    ) -> AreaResult<Vec<T>> {
        let mut stmt = conn.prepare(sql)?;
        let mut out = vec![];
        let mut rows = stmt.query(())?;
        while let Some(row) = rows.next()? {
            if let Some(item) = f(row) {
                out.push(item)
            }
        }
        Ok(out)
    }
}
impl AreaDataProvider for SqliteAreaData {
    fn read_code_data(&self) -> AreaResult<Vec<AreaCodeData>> {
        let conn = rusqlite::Connection::open(&self.code_config.conn)?;
        let u = read_file_modified_time(&self.code_config.conn);
        *self.geo_file_time.lock() = u;
        self.read_data(&conn, &self.code_config.sql, |row| {
            if let Ok(code) = row.get(self.code_config.column_code.as_str()) {
                let hide = row.get(self.code_config.column_hide.as_str()).unwrap_or(0);
                let name: Result<String, rusqlite::Error> =
                    row.get(self.code_config.column_name.as_str());
                let keyword = row
                    .get(self.code_config.column_key_word.as_str())
                    .unwrap_or_else(|_| "".to_string());
                let mut key_word = keyword
                    .split(',')
                    .filter(|e| !e.is_empty())
                    .map(|e| e.to_owned())
                    .collect::<Vec<_>>();
                if self.code_config.key_word_name {
                    if let Ok(ename) = &name {
                        key_word.push(ename.to_owned());
                    }
                }
                Some(AreaCodeData {
                    code,
                    hide: hide == 1 || name.as_ref().map(|e| e.is_empty()).unwrap_or(true),
                    name: name.unwrap_or_else(|_| "[_._]".to_string()),
                    key_word,
                })
            } else {
                None
            }
        })
    }
    fn read_geo_data(&self) -> AreaResult<Vec<AreaGeoData>> {
        match &self.geo_config {
            Some(get_config) => {
                let u = read_file_modified_time(&get_config.conn);
                *self.geo_file_time.lock() = u;
                let conn = rusqlite::Connection::open(&get_config.conn)?;
                let out = self.read_data(&conn, get_config.sql.as_str(), |row| {
                    match row.get(get_config.column_code.as_str()) {
                        Ok(code) => {
                            if let Ok(polygon) = row.get(get_config.column_polygon.as_str()) {
                                return Some(AreaGeoData {
                                    code,
                                    item: vec![AreaGeoDataItem {
                                        center: row
                                            .get(get_config.column_center.as_str())
                                            .unwrap_or_else(|_| "".to_string()),
                                        polygon,
                                    }],
                                });
                            }
                        }
                        Err(e) => {
                            warn!("geo error:{}", e);
                            return None;
                        }
                    }
                    None
                })?;
                Ok(out)
            }
            None => Ok(vec![]),
        }
    }
    fn code_data_is_change(&self) -> bool {
        if let Some(lt) = read_file_modified_time(&self.code_config.conn) {
            if let Some(st) = *self.code_file_time.lock() {
                return lt > st;
            }
        }
        false
    }
    fn geo_data_is_change(&self) -> bool {
        if let Some(geo_config) = &self.geo_config {
            if let Some(lt) = read_file_modified_time(&geo_config.conn) {
                if let Some(st) = *self.code_file_time.lock() {
                    return lt > st;
                }
            }
        }
        false
    }
}
