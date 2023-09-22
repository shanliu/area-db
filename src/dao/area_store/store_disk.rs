use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::path::PathBuf;

use crate::{
    AreaCode, AreaCodeIndexData, AreaCodeIndexInfo, AreaCodeIndexTree, AreaCodeProvider,
    AreaCodeTantivy, AreaError, AreaGeo, AreaGeoIndexInfo, AreaGeoProvider, AreaResult,
    AreaStoreProvider,
};
use geo::{LineString, Point};
use memmap2::{Mmap, MmapMut};
use std::fs::OpenOptions;
use std::io::{Seek, SeekFrom};
use tantivy::directory::MmapDirectory;
use tantivy::{schema::Schema, store::Compressor, Index, IndexBuilder, IndexSettings, IndexWriter};

fn mmap_find_version(mmap: &Mmap, ver_start_index: usize) -> AreaResult<String> {
    let ver_len = std::mem::size_of::<usize>();
    let tmp = unsafe {
        let version_len = mmap[ver_start_index..].as_ptr() as *const usize;
        let ver_start = ver_start_index + ver_len;
        let ver_end = ver_start_index + ver_len + version_len.read();
        String::from_utf8_lossy(&mmap[ver_start..ver_end]).to_string()
    };
    Ok(tmp)
}
fn mmap_check_index(max_len: usize, index: usize) -> AreaResult<()> {
    if max_len == 0 {
        return Err(AreaError::Store("data is empty".to_string()));
    }
    if index > max_len - 1 {
        return Err(AreaError::Store(format!(
            "max is :{},your submit index is:{}",
            max_len, index
        )));
    }
    Ok(())
}

pub struct AreaCodeIndexDataDisk {
    path: PathBuf,
    mmap: Option<Mmap>,
    hash: HashMap<String, AreaCodeIndexInfo>,
}

impl AreaCodeIndexDataDisk {
    pub fn new(path: PathBuf) -> Self {
        Self {
            mmap: None,
            path,
            hash: HashMap::new(),
        }
    }
}

fn mmap_find_code_index_info(
    mmap: &Mmap,
    index: usize,
) -> AreaResult<(u64, String, AreaCodeIndexInfo)> {
    let info_len = std::mem::size_of::<(usize, usize, usize, usize)>();
    let prefix = std::mem::size_of::<(u64, bool, usize, usize)>();
    let (max_len, max_key_length, max_value_length, version_len) = unsafe {
        let ptr = mmap[0..].as_ptr() as *const (usize, usize, usize, usize);
        ptr.read()
    };
    mmap_check_index(max_len, index)?;
    let item_len = prefix + max_key_length + max_value_length;
    let (tmp, key_tmp, val_tmp) = unsafe {
        let ptr = mmap[info_len + version_len + index * item_len..].as_ptr()
            as *const (u64, bool, usize, usize);
        let tmp: (u64, bool, usize, usize) = ptr.read();
        // println!("{:?}:{}", tmp, index);
        // 读取20字节长度数据
        let key_start = index * item_len + info_len + version_len + prefix;
        let key_end = tmp.2;
        let val_start = index * item_len + info_len + version_len + prefix + max_key_length;
        let val_end = tmp.3;
        let key_tmp = String::from_utf8_lossy(&mmap[key_start..key_start + key_end]).to_string();
        let val_tmp = String::from_utf8_lossy(&mmap[val_start..val_start + val_end]).to_string();
        (tmp, key_tmp, val_tmp)
    };
    Ok((
        tmp.0,
        key_tmp,
        AreaCodeIndexInfo {
            hide: tmp.1,
            name: val_tmp,
        },
    ))
}

impl AreaCodeIndexData for AreaCodeIndexDataDisk {
    fn set(&mut self, key: String, val: AreaCodeIndexInfo) -> AreaResult<()> {
        self.hash.insert(key, val);
        Ok(())
    }
    fn clear(&mut self) -> AreaResult<()> {
        self.hash = HashMap::new();
        self.mmap = None;
        Ok(())
    }
    fn get(&self, index: &str) -> Option<AreaCodeIndexInfo> {
        if let Some(mmap) = &self.mmap {
            let info_len = std::mem::size_of::<(usize, usize, usize, usize)>();
            let prefix = std::mem::size_of::<(u64, bool, usize, usize)>();
            let (max_len, max_key_length, max_value_length, version_len) = unsafe {
                let ptr = mmap[0..].as_ptr() as *const (usize, usize, usize, usize);
                ptr.read()
            };

            if max_len == 0 {
                return None;
            }

            let item_len = prefix + max_key_length + max_value_length;

            let find_start = index.parse::<u64>().unwrap_or(0);
            let (start_index, start_code) = if find_start > 0 {
                //折半查找。。。fix...
                let mut start = max_len / 2;
                loop {
                    if start > 0 {
                        mmap_check_index(max_len, start).ok()?;
                        let code = unsafe {
                            let ptr = mmap[info_len + version_len + start * item_len..].as_ptr()
                                as *const u64;
                            ptr.read()
                        };
                        if code == 0 {
                            break (0, 0);
                        }
                        if code > find_start {
                            start /= 2;
                            continue;
                        } else {
                            let add_tmp = start / 2;
                            if add_tmp == 0 {
                                break (start, code);
                            }
                            start += add_tmp;
                        }
                    }
                    break (0, 0);
                }
            } else {
                (0, 0)
            };
            if start_index > 0 && start_code == find_start {
                let mut prev_index = start_index;
                loop {
                    let (code, key, val) = mmap_find_code_index_info(mmap, prev_index).ok()?;
                    if key.as_str() == index {
                        return Some(val);
                    }
                    if prev_index == 0 || find_start > code {
                        break;
                    }
                    prev_index -= 1;
                }
            }
            for i in start_index..=max_len - 1 {
                let (code, key, val) = mmap_find_code_index_info(mmap, i).ok()?;
                if code > find_start {
                    break;
                }
                if key.as_str() == index {
                    return Some(val);
                }
            }
        }
        None
    }
    fn save(&mut self, version: &str) -> AreaResult<()> {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.path)?;
        let mut max_key_length = 0;
        let mut max_value_length = 0;
        let max_len = self.hash.len();
        let mut vec_data = Vec::with_capacity(max_len);
        for (key, value) in self.hash.iter() {
            if key.len() > max_key_length {
                max_key_length = key.len();
            }
            if value.name.len() > max_value_length {
                max_value_length = value.name.len();
            }
            let index = key.parse::<u64>().unwrap_or(0);
            vec_data.push((index, key, value))
        }
        vec_data.sort_by(|a, b| a.0.cmp(&b.0));
        //元素数量，最大key长度，最大value长度，版本信息长度+ 版本内容长度
        let info_len = std::mem::size_of::<(usize, usize, usize, usize)>() + version.len();
        let prefix = std::mem::size_of::<(u64, bool, usize, usize)>();
        let item_len = prefix + max_key_length + max_value_length;
        file.set_len((info_len + item_len * max_len) as u64)?;
        file.seek(SeekFrom::Start(0))?;
        let mut mmap = unsafe { MmapMut::map_mut(&file)? };
        let ptr = mmap.as_mut_ptr();
        let tmp = &(max_len, max_key_length, max_value_length, version.len())
            as *const (usize, usize, usize, usize) as *const u8;
        unsafe {
            std::ptr::copy_nonoverlapping(tmp, ptr, info_len);
            std::ptr::copy_nonoverlapping(
                version.as_bytes().as_ptr(),
                ptr.add(info_len),
                version.len(),
            );
        }

        for (i, (index, key, value)) in vec_data.into_iter().enumerate() {
            let tmp = &(index, value.hide, key.len(), value.name.len())
                as *const (u64, bool, usize, usize) as *const u8;
            let ptr = mmap.as_mut_ptr();
            unsafe {
                std::ptr::copy_nonoverlapping(tmp, ptr.add(i * item_len + info_len), prefix);
                let key_start = i * item_len + info_len + prefix;
                std::ptr::copy_nonoverlapping(
                    key.as_bytes().as_ptr(),
                    ptr.add(key_start),
                    key.len(),
                );
                let val_start = i * item_len + info_len + prefix + max_key_length;
                std::ptr::copy_nonoverlapping(
                    value.name.as_bytes().as_ptr(),
                    ptr.add(val_start),
                    value.name.len(),
                );
            }
        }
        mmap.flush()?;
        file.seek(SeekFrom::Start(0))?;
        let mmap = unsafe { Mmap::map(&file)? };
        self.hash = HashMap::new();
        self.mmap = Some(mmap);
        Ok(())
    }
    fn version(&self) -> String {
        self.mmap
            .as_ref()
            .map(|e| {
                mmap_find_version(e, std::mem::size_of::<(usize, usize, usize)>())
                    .unwrap_or_default()
            })
            .unwrap_or_default()
    }
}

pub struct AreaCodeIndexTreeDisk {
    mmap: Option<Mmap>,
    path: PathBuf,
    data: HashMap<String, Vec<String>>,
}
impl AreaCodeIndexTreeDisk {
    pub fn new(path: PathBuf) -> Self {
        Self {
            mmap: None,
            path,
            data: HashMap::new(),
        }
    }
}

fn mmap_find_code_tree(mmap: &Mmap, index: usize) -> AreaResult<(u64, String, Vec<String>)> {
    let info_len = std::mem::size_of::<(usize, usize, usize, usize, usize)>();
    let prefix = std::mem::size_of::<(u64, usize, usize, usize)>();
    let (max_len, max_key_length, max_tree_length, max_tree_count, version_len) = unsafe {
        let ptr = mmap[0..].as_ptr() as *const (usize, usize, usize, usize, usize);
        ptr.read()
    };
    mmap_check_index(max_len, index)?;
    let item_len = prefix + max_key_length + max_tree_length * max_tree_count;
    let (tmp, key_tmp, val_tmp) = unsafe {
        let ptr = mmap[info_len + version_len + index * item_len..].as_ptr()
            as *const (u64, usize, usize, usize);
        // //index,key_length,sub-code-count,sub-code-len,
        let tmp: (u64, usize, usize, usize) = ptr.read();
        // 读取20字节长度数据
        let key_start = index * item_len + info_len + version_len + prefix;
        let key_end = tmp.1;
        let key_tmp = String::from_utf8_lossy(&mmap[key_start..key_start + key_end]).to_string();
        let mut val_tmp = Vec::with_capacity(tmp.2);
        if tmp.2 > 0 {
            for val_i in 0..=tmp.2 - 1 {
                let val_start = index * item_len
                    + info_len
                    + version_len
                    + prefix
                    + max_key_length
                    + val_i * tmp.3;
                let val_end = tmp.3;
                let sub_val =
                    String::from_utf8_lossy(&mmap[val_start..val_start + val_end]).to_string();
                val_tmp.push(sub_val);
            }
        }
        (tmp, key_tmp, val_tmp)
    };
    Ok((tmp.0, key_tmp, val_tmp))
}

fn mmap_code_tree_childs(mmap: &Mmap, index: &str) -> Option<Vec<(String, bool)>> {
    let info_len = std::mem::size_of::<(usize, usize, usize, usize, usize)>();
    let prefix = std::mem::size_of::<(u64, usize, usize, usize)>();
    let (max_len, max_key_length, max_tree_length, max_tree_count, version_len) = unsafe {
        let ptr = mmap[0..].as_ptr() as *const (usize, usize, usize, usize, usize);
        ptr.read()
    };
    let item_len = prefix + max_key_length + max_tree_length * max_tree_count;

    if max_len == 0 {
        return None;
    }

    let find_start = index.parse::<u64>().unwrap_or(0);
    let (start_index, start_code) = if find_start > 0 {
        let mut start = max_len / 2;

        loop {
            if start > 0 {
                mmap_check_index(max_len, start).ok()?;
                let code = unsafe {
                    let ptr =
                        mmap[info_len + version_len + start * item_len..].as_ptr() as *const u64;
                    ptr.read()
                };
                if code == 0 {
                    break (0, 0);
                }
                if code > find_start {
                    start /= 2;
                    continue;
                } else {
                    break (start, code);
                }
            }
            break (0, 0);
        }
    } else {
        (0, 0)
    };
    if start_index > 0 && start_code == find_start {
        let mut prev_index = start_index;
        loop {
            let (code, key, val) = mmap_find_code_tree(mmap, prev_index).ok()?;
            if key.as_str() == index {
                return Some(
                    val.into_iter()
                        .map(|t| {
                            let next = mmap_code_tree_childs(mmap, &t)
                                .map(|tt| tt.is_empty())
                                .unwrap_or(true);
                            (t, next)
                        })
                        .collect(),
                );
            }
            if prev_index == 0 || find_start > code {
                break;
            }
            prev_index -= 1;
        }
    }
    for i in start_index..=max_len - 1 {
        let (code, key, val) = mmap_find_code_tree(mmap, i).ok()?;
        if code > find_start {
            break;
        }
        if key.as_str() == index {
            return Some(
                val.into_iter()
                    .map(|t| {
                        let next = mmap_code_tree_childs(mmap, &t)
                            .map(|tt| tt.is_empty())
                            .unwrap_or(true);
                        (t, next)
                    })
                    .collect(),
            );
        }
    }
    None
}

impl AreaCodeIndexTree for AreaCodeIndexTreeDisk {
    fn clear(&mut self) -> AreaResult<()> {
        self.data = HashMap::new();
        self.mmap = None;
        Ok(())
    }
    fn add(&mut self, code_data: Vec<&str>) -> AreaResult<()> {
        let mut perv = self.data.entry("".to_string());
        for ddd in code_data {
            let code = ddd.to_string();
            match perv {
                Entry::Occupied(mut tmp) => {
                    if !tmp.get().contains(&code) {
                        tmp.get_mut().push(code.clone());
                    }
                }
                Entry::Vacant(tmp) => {
                    tmp.insert(vec![code.clone()]);
                }
            };
            perv = self.data.entry(code);
        }
        Ok(())
    }
    fn childs(&self, code_data: &[&str]) -> Option<Vec<(String, bool)>> {
        let index = code_data.join("");
        if let Some(mmap) = &self.mmap {
            return mmap_code_tree_childs(mmap, &index);
        }
        None
    }
    fn save(&mut self, version: &str) -> AreaResult<()> {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.path)?;
        let mut max_key_length = 0;
        let mut max_tree_length = 0;
        let mut max_tree_count = 0;
        let max_len = self.data.len();
        let mut vec_data = Vec::with_capacity(max_len);
        for (key, value) in self.data.iter() {
            if key.len() > max_key_length {
                max_key_length = key.len();
            }
            if value.len() > max_tree_count {
                max_tree_count = value.len();
            }
            let max_str_len = value.iter().map(|e| e.len()).max().unwrap_or(0);
            if max_str_len > max_tree_length {
                max_tree_length = max_str_len;
            }
            let index = key.parse::<u64>().unwrap_or(0);
            vec_data.push((index, key, value, max_tree_count, max_tree_length));
        }
        vec_data.sort_by(|a, b| a.0.cmp(&b.0));

        //max_len,max_key_length,max_tree_length,max_tree_count,version_len
        let info_len = std::mem::size_of::<(usize, usize, usize, usize, usize)>() + version.len();
        //index,key_length,sub-code-count,sub-code-len,
        let prefix = std::mem::size_of::<(u64, usize, usize, usize)>();
        let item_len = prefix + max_key_length + max_tree_length * max_tree_count;
        file.set_len((info_len + item_len * max_len) as u64)?;
        file.seek(SeekFrom::Start(0))?;
        let mut mmap = unsafe { MmapMut::map_mut(&file)? };
        let ptr = mmap.as_mut_ptr();
        let tmp = &(
            max_len,
            max_key_length,
            max_tree_length,
            max_tree_count,
            version.len(),
        ) as *const (usize, usize, usize, usize, usize) as *const u8;
        unsafe {
            std::ptr::copy_nonoverlapping(tmp, ptr, info_len);
            std::ptr::copy_nonoverlapping(
                version.as_bytes().as_ptr(),
                ptr.add(info_len),
                version.len(),
            );
        }

        for (i, (index, key, value, max_tree_count, max_tree_length)) in
            vec_data.into_iter().enumerate()
        {
            //index,key_length,sub-code-count,sub-code-len,
            let tmp = &(index, key.len(), max_tree_count, max_tree_length)
                as *const (u64, usize, usize, usize) as *const u8;
            let ptr = mmap.as_mut_ptr();
            unsafe {
                // println!(
                //     "{:?},{}",
                //     (index, key.len(), max_tree_count, max_tree_length),
                //     i * item_len + info_len
                // );
                std::ptr::copy_nonoverlapping(tmp, ptr.add(i * item_len + info_len), prefix);
                let key_start = i * item_len + info_len + prefix;
                std::ptr::copy_nonoverlapping(
                    key.as_bytes().as_ptr(),
                    ptr.add(key_start),
                    key.len(),
                );
                for (i_val, tmp_val) in value.iter().enumerate() {
                    //理论上code等长
                    let val_start =
                        i * item_len + info_len + prefix + max_key_length + i_val * max_tree_length;
                    std::ptr::copy_nonoverlapping(
                        tmp_val.as_bytes().as_ptr(),
                        ptr.add(val_start),
                        tmp_val.len(),
                    );
                }
            }
        }
        mmap.flush()?;
        file.seek(SeekFrom::Start(0))?;
        let mmap = unsafe { Mmap::map(&file)? };
        self.data = HashMap::new();
        self.mmap = Some(mmap);
        Ok(())
    }
    fn version(&self) -> String {
        self.mmap
            .as_ref()
            .map(|e| {
                mmap_find_version(e, std::mem::size_of::<(usize, usize, usize, usize)>())
                    .unwrap_or_default()
            })
            .unwrap_or_default()
    }
}

pub struct AreaCodeTantivyDisk {
    path: PathBuf,
    index_size: usize,
}

impl AreaCodeTantivyDisk {
    pub fn new(path: PathBuf, index_size: usize) -> Self {
        Self { path, index_size }
    }
}

impl AreaCodeTantivy for AreaCodeTantivyDisk {
    fn create_index(&self, schema: Schema) -> AreaResult<Index> {
        Ok(IndexBuilder::new()
            .schema(schema)
            .settings(IndexSettings {
                docstore_compression: Compressor::None,
                ..IndexSettings::default()
            })
            .open_or_create(MmapDirectory::open(&self.path)?)?)
    }
    fn index_writer(&self, index: &Index) -> AreaResult<IndexWriter> {
        Ok(index.writer(self.index_size)?)
    }
}
#[derive(Default)]
pub struct DiskAreaCodeProvider {}

impl AreaCodeProvider for DiskAreaCodeProvider {
    type CD = AreaCodeIndexDataDisk;
    type CT = AreaCodeIndexTreeDisk;
    type TT = AreaCodeTantivyDisk;
}

pub struct DiskAreaGeoProvider {
    center_data: Vec<(u64, String, Point)>,
    polygon_data: HashMap<u64, (LineString, Vec<LineString>)>,
    mmap: Option<Mmap>,
    path: PathBuf,
}
impl DiskAreaGeoProvider {
    pub fn new(path: PathBuf) -> Self {
        Self {
            mmap: None,
            path,
            polygon_data: HashMap::new(),
            center_data: vec![],
        }
    }
}

fn mmap_find_polygon_data(mmap: &Mmap, index: usize) -> Option<(u64, AreaGeoIndexInfo)> {
    let info_len = std::mem::size_of::<(usize, usize, usize, usize, usize, usize)>();

    let (center_max_len, max_center_length, polygon_prefix_len, _, max_polygon_size, version_len) = unsafe {
        let ptr = mmap[0..].as_ptr() as *const (usize, usize, usize, usize, usize, usize);
        ptr.read()
    };
    if center_max_len == 0 {
        return None;
    }
    let center_prefix = std::mem::size_of::<(u64, usize, f64, f64)>();
    let mmap_center_size = (center_prefix + max_center_length) * center_max_len;
    let info_all_len = info_len + version_len + mmap_center_size;

    let polygon_prefix_item = std::mem::size_of::<usize>();
    let polygon_prefix = std::mem::size_of::<u64>() + polygon_prefix_item * polygon_prefix_len;
    let polygon_geo_size = std::mem::size_of::<(f64, f64)>();
    let item_len = polygon_prefix + max_polygon_size * polygon_geo_size;

    let polygon_tmp_prefix = std::mem::size_of::<(u64, usize, usize)>();
    let (out_index, exterior, interiors) = unsafe {
        let ptr = mmap[info_all_len + index * item_len..].as_ptr() as *const (u64, usize, usize);
        // //index,外框元素长度,内框数量
        let (i, wlen_tmp, ilen_tmp): (u64, usize, usize) = ptr.read();
        let mut wout = Vec::with_capacity(wlen_tmp);
        if wlen_tmp > 0 {
            for sub_i in 0..=ilen_tmp - 1 {
                let tmp_ptr = mmap
                    [info_all_len + index * item_len + polygon_prefix + polygon_geo_size * sub_i..]
                    .as_ptr() as *const (f64, f64);
                let tmp_data = tmp_ptr.read();
                wout.push(Point::new(tmp_data.0, tmp_data.1))
            }
        }
        let wline_str = LineString::from(wout);

        let mut iline_start =
            info_all_len + index * item_len + polygon_prefix + polygon_geo_size * wlen_tmp;
        let mut iline_str_vec = Vec::with_capacity(ilen_tmp);
        if ilen_tmp > 0 {
            for sub_i in 0..=ilen_tmp - 1 {
                let ptr = mmap[info_all_len
                    + index * item_len
                    + polygon_tmp_prefix
                    + polygon_prefix_item * sub_i..]
                    .as_ptr() as *const usize;
                let ilen = ptr.read();
                let mut iline_str = Vec::with_capacity(ilen);
                for sub_ii in 0..=ilen - 1 {
                    let tmp_ptr = mmap[iline_start + sub_ii * polygon_geo_size..].as_ptr()
                        as *const (f64, f64);
                    let tmp_data = tmp_ptr.read();
                    iline_str.push(Point::new(tmp_data.0, tmp_data.1))
                }
                iline_str_vec.push(LineString::from(iline_str));
                iline_start += ilen * polygon_geo_size;
            }
        }
        (i, wline_str, iline_str_vec)
    };

    Some((out_index, AreaGeoIndexInfo::new(exterior, interiors)))
}

impl AreaGeoProvider for DiskAreaGeoProvider {
    fn clear(&mut self) -> AreaResult<()> {
        self.center_data = vec![];
        self.polygon_data = HashMap::new();
        Ok(())
    }
    fn add_center_data(&mut self, index: u64, center: &str, geo: Point) -> AreaResult<()> {
        self.center_data.push((index, center.to_string(), geo));
        Ok(())
    }
    fn get_center_data(&self) -> AreaResult<Vec<(u64, String, Point)>> {
        let mmap = match self.mmap.as_ref() {
            Some(t) => t,
            None => return Ok(vec![]),
        };
        let info_len = std::mem::size_of::<(usize, usize, usize, usize, usize, usize)>();

        let prefix = std::mem::size_of::<(u64, usize, f64, f64)>();

        let (center_max_len, max_center_length, _, _, _, version_len) = unsafe {
            let ptr = mmap[0..].as_ptr() as *const (usize, usize, usize, usize, usize, usize);
            ptr.read()
        };

        if center_max_len == 0 {
            return Ok(vec![]);
        }

        let item_len = prefix + max_center_length;
        let out = unsafe {
            let mut out = Vec::with_capacity(center_max_len);
            for index in 0..=center_max_len - 1 {
                let ptr = mmap[info_len + version_len + index * item_len..].as_ptr()
                    as *const (u64, usize, f64, f64);
                // //index,key_length,sub-code-count,sub-code-len,
                let tmp: (u64, usize, f64, f64) = ptr.read();
                // 读取20字节长度数据
                let key_start = index * item_len + info_len + version_len + prefix;
                let key_end = tmp.1;
                let key_tmp =
                    String::from_utf8_lossy(&mmap[key_start..key_start + key_end]).to_string();
                let point = Point::new(tmp.2, tmp.3);
                out.push((tmp.0, key_tmp, point))
            }
            out
        };
        Ok(out)
    }
    fn set_polygon_data(
        &mut self,
        i: u64,
        exterior: LineString,
        interiors: Vec<LineString>,
    ) -> AreaResult<()> {
        self.polygon_data.insert(i, (exterior, interiors));
        Ok(())
    }
    fn get_polygon_data(&self, index: &u64) -> Option<AreaGeoIndexInfo> {
        let mmap = match self.mmap.as_ref() {
            Some(t) => t,
            None => return None,
        };

        let info_len = std::mem::size_of::<(usize, usize, usize, usize, usize, usize)>();

        let (
            center_max_len,
            max_center_length,
            polygon_prefix_len,
            max_len,
            max_polygon_size,
            version_len,
        ) = unsafe {
            let ptr = mmap[0..].as_ptr() as *const (usize, usize, usize, usize, usize, usize);
            ptr.read()
        };
        if max_len == 0 {
            return None;
        }
        let center_prefix = std::mem::size_of::<(u64, usize, f64, f64)>();
        let mmap_center_size = (center_prefix + max_center_length) * center_max_len;
        let info_all_len = info_len + version_len + mmap_center_size;

        let polygon_prefix_item = std::mem::size_of::<usize>();
        let polygon_prefix = std::mem::size_of::<u64>() + polygon_prefix_item * polygon_prefix_len;
        let polygon_geo_size = std::mem::size_of::<(f64, f64)>();
        let item_len = polygon_prefix + max_polygon_size * polygon_geo_size;

        let find_start = *index;
        let (start_index, start_code) = if find_start > 0 {
            let mut start = max_len / 2;
            loop {
                if start > 0 {
                    mmap_check_index(max_len, start).ok()?;
                    let code = unsafe {
                        let ptr = mmap[info_all_len + start * item_len..].as_ptr() as *const u64;
                        ptr.read()
                    };
                    if code == 0 {
                        break (0, 0);
                    }
                    if code > find_start {
                        start /= 2;
                        continue;
                    } else {
                        break (start, code);
                    }
                }
                break (0, 0);
            }
        } else {
            (0, 0)
        };
        if start_index > 0 && start_code == find_start {
            let mut prev_index = start_index;
            loop {
                let (code, info) = mmap_find_polygon_data(mmap, prev_index)?;
                if code == *index {
                    return Some(info);
                }
                if prev_index == 0 || find_start > code {
                    break;
                }
                prev_index -= 1;
            }
        }
        for i in start_index..=max_len - 1 {
            let (code, info) = mmap_find_polygon_data(mmap, i)?;
            if code > find_start {
                break;
            }
            if code == *index {
                return Some(info);
            }
        }
        None
    }
    fn save(&mut self, version: &str) -> AreaResult<()> {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.path)?;

        let mut max_center_length = 0;
        let center_max_len = self.center_data.len();
        let mut center_data = Vec::with_capacity(center_max_len);
        for (index, value, point) in self.center_data.iter() {
            if value.len() > max_center_length {
                max_center_length = value.len();
            }
            center_data.push((index, value, (point.0.x, point.0.y)));
        }
        center_data.sort_by(|a, b| a.0.cmp(b.0));

        //index,lat,lng,key_len
        let center_prefix = std::mem::size_of::<(u64, usize, f64, f64)>();
        let center_item_len = center_prefix + max_center_length;

        let mut max_polygon_size = 0;
        let polygon_max_len = self.polygon_data.len();
        let mut polygon_data = Vec::with_capacity(polygon_max_len);
        let mut polygon_prefix_len = 0;
        for tmp in self.polygon_data.iter() {
            let tmp_len = tmp.1 .0 .0.len() + tmp.1 .1.iter().map(|t| t.0.len()).sum::<usize>();
            if max_polygon_size < tmp_len {
                max_polygon_size = tmp_len;
            }
            let tmp_str_len = 1 + tmp.1 .1.len();
            if polygon_prefix_len < tmp_str_len {
                polygon_prefix_len = tmp_str_len;
            }
            let mut data_vec = Vec::with_capacity(tmp_len);
            for tmp1 in &tmp.1 .0 .0 {
                data_vec.push((tmp1.x, tmp1.y));
            }
            for tmp2 in &tmp.1 .1 {
                for tmp3 in &tmp2.0 {
                    data_vec.push((tmp3.x, tmp3.y));
                }
            }
            polygon_data.push((
                tmp.1 .0 .0.len(),
                tmp.1 .1.len(),
                tmp.1 .1.iter().map(|t| t.0.len()).collect::<Vec<usize>>(),
                data_vec,
            ));
        }

        //center_max_len,max_center_length,polygon_prefix_len,polygon_max_len,max_polygon_size,version_len
        let info_len =
            std::mem::size_of::<(usize, usize, usize, usize, usize, usize)>() + version.len();
        let mmap_center_size = center_item_len * center_max_len;

        //index,1+n,(lat,lng)*m
        let polygon_prefix_item = std::mem::size_of::<usize>();
        let polygon_prefix = std::mem::size_of::<u64>() + polygon_prefix_item * polygon_prefix_len;
        let polygon_geo_size = std::mem::size_of::<(f64, f64)>();
        let polygon_item_size = polygon_prefix + max_polygon_size * polygon_geo_size;
        let mmap_polygon_size = polygon_item_size * polygon_max_len;

        file.set_len((info_len + mmap_center_size + mmap_polygon_size) as u64)?;
        file.seek(SeekFrom::Start(0))?;

        let mut mmap = unsafe { MmapMut::map_mut(&file)? };
        let ptr = mmap.as_mut_ptr();
        let tmp = &(
            center_max_len,
            max_center_length,
            polygon_prefix_len,
            polygon_max_len,
            max_polygon_size,
            version.len(),
        ) as *const (usize, usize, usize, usize, usize, usize) as *const u8;
        unsafe {
            std::ptr::copy_nonoverlapping(tmp, ptr, info_len);
            std::ptr::copy_nonoverlapping(
                version.as_bytes().as_ptr(),
                ptr.add(info_len),
                version.len(),
            );
        }
        //write center data  mmap
        for (i, (index, key, item)) in center_data.into_iter().enumerate() {
            //index,lat,lng,key_len
            let tmp = &(index.to_owned(), item.0, item.1, key.len())
                as *const (u64, f64, f64, usize) as *const u8;
            let ptr = mmap.as_mut_ptr();
            unsafe {
                std::ptr::copy_nonoverlapping(
                    tmp,
                    ptr.add(i * center_item_len + info_len),
                    center_prefix,
                );
                let key_start = i * center_item_len + info_len + center_prefix;
                std::ptr::copy_nonoverlapping(
                    key.as_bytes().as_ptr(),
                    ptr.add(key_start),
                    key.len(),
                );
            }
        }
        //write polygon data to mmap
        for (i, (wlen, ilen, idata, pdata)) in polygon_data.into_iter().enumerate() {
            //index,lat,lng,key_len
            let tmp = &(wlen, ilen) as *const (usize, usize) as *const u8;
            let tmp_size = std::mem::size_of::<(usize, usize)>();
            let ptr = mmap.as_mut_ptr();
            unsafe {
                std::ptr::copy_nonoverlapping(
                    tmp,
                    ptr.add(i * polygon_item_size + mmap_center_size + info_len),
                    tmp_size,
                );

                let tmpii_size = std::mem::size_of::<usize>();
                for (iit, iitmp) in idata.iter().enumerate() {
                    let tmp1 = iitmp as *const usize as *const u8;
                    std::ptr::copy_nonoverlapping(
                        tmp1,
                        ptr.add(
                            i * polygon_item_size
                                + mmap_center_size
                                + info_len
                                + tmp_size
                                + iit * tmpii_size,
                        ),
                        tmpii_size,
                    );
                }
                let tmpii_size = std::mem::size_of::<(f64, f64)>();
                for (iit, iitmp) in pdata.iter().enumerate() {
                    let tmp1 = iitmp as *const (f64, f64) as *const u8;
                    std::ptr::copy_nonoverlapping(
                        tmp1,
                        ptr.add(
                            i * polygon_item_size
                                + mmap_center_size
                                + info_len
                                + polygon_prefix
                                + iit * tmpii_size,
                        ),
                        tmpii_size,
                    );
                }
            }
        }
        mmap.flush()?;
        file.seek(SeekFrom::Start(0))?;
        let mmap = unsafe { Mmap::map(&file)? };
        self.center_data = vec![];
        self.polygon_data = HashMap::new();
        self.mmap = Some(mmap);
        Ok(())
    }
    fn version(&self) -> String {
        self.mmap
            .as_ref()
            .map(|e| {
                mmap_find_version(
                    e,
                    std::mem::size_of::<(usize, usize, usize, usize, usize)>(),
                )
                .unwrap_or_default()
            })
            .unwrap_or_default()
    }
}

pub struct AreaStoreDisk {
    dir: PathBuf,
    index_size: usize,
}
impl AreaStoreDisk {
    pub fn new(dir: PathBuf, index_size: Option<usize>) -> Self {
        Self {
            dir,
            index_size: index_size.unwrap_or(500_000_000),
        }
    }
}

impl Default for AreaStoreDisk {
    fn default() -> Self {
        let dir = std::env::temp_dir();
        Self {
            dir,
            index_size: 500_000_000,
        }
    }
}

impl AreaStoreProvider for AreaStoreDisk {
    type C = DiskAreaCodeProvider;
    type G = DiskAreaGeoProvider;
    fn create_code(&self) -> AreaResult<AreaCode<Self::C>> {
        let mut tantivy_dir = self.dir.clone();
        tantivy_dir.push("area_data_tantivy/");
        if std::fs::metadata(&tantivy_dir).is_err() {
            std::fs::create_dir(&tantivy_dir)?;
        }
        let mut info_dir = self.dir.clone();
        info_dir.push("area_data_info.bin");
        let mut tree_dir = self.dir.clone();
        tree_dir.push("area_data_tree.bin");
        AreaCode::new(
            AreaCodeTantivyDisk::new(tantivy_dir, self.index_size),
            AreaCodeIndexDataDisk::new(info_dir),
            AreaCodeIndexTreeDisk::new(tree_dir),
        )
    }
    fn create_geo(&self) -> AreaResult<AreaGeo<Self::G>> {
        let mut geo_dir = self.dir.clone();
        geo_dir.push("area_data_geo.bin");
        Ok(AreaGeo::new(DiskAreaGeoProvider::new(geo_dir)))
    }
}
