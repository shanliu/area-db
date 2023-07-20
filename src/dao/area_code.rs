use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    vec,
};

use crate::{AreaError, AreaResult};
use rayon::iter::ParallelIterator;
use rayon::slice::ParallelSlice;

#[derive(Debug)]
pub struct AreaCodeItem {
    pub code: String,
    pub name: String,
    pub leaf: bool,
}

#[derive(Debug)]
pub struct AreaCodeDetailItem {
    pub selected: bool,
    pub item: AreaCodeItem,
}

#[derive(Debug)]
struct AreaCodeIndex {
    code: String,
    childs: HashMap<String, AreaCodeIndex>,
}

#[derive(Debug)]
pub struct AreaSearchItem {
    pub item: Vec<AreaCodeItem>,
    pub key_word: String,
}

//HashMap<Code,Vec<Key word>>

#[derive(Debug)]
struct AreaSearchMoreIndex {
    code: String,
    detail_vec: String,
}

#[derive(Debug)]
struct AreaSearchQueueItem<'t> {
    sim: usize,
    code: &'t str,
    key_word: &'t str,
}

impl<'t> PartialEq for AreaSearchQueueItem<'t> {
    fn eq(&self, other: &Self) -> bool {
        self.code == other.code
    }
}
impl<'t> Eq for AreaSearchQueueItem<'t> {}
impl<'t> Ord for AreaSearchQueueItem<'t> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.sim.cmp(&other.sim)
    }
}

impl<'t> PartialOrd for AreaSearchQueueItem<'t> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.sim.cmp(&other.sim))
    }
}

const CLEAR_WORD: &[&str] = &["-", "社区居委会", "居委会", "片区街道", "街道办", "委会"];
const BAD_WORD: &[&str] = &[
    "-",
    "开发区",
    "自治县",
    "自治州",
    "自治区",
    "特别行政区",
    "自治区",
    "直辖县级行政区划",
    "村委会",
    "居委会",
    "省",
    "市",
    "县",
    "区",
    "镇",
    "乡",
    "村",
    "片区街道",
    "街道办",
];

pub struct AreaCode {
    code_data: HashMap<String, (bool, String)>,
    code_area_data: HashMap<String, AreaCodeIndex>,
    search_area_data: Vec<(usize, HashMap<String, Vec<String>>)>,
    search_detail_data: Vec<AreaSearchMoreIndex>,
}

#[derive(Debug)]
pub struct AreaCodeData {
    pub code: String,
    pub hide: bool,
    pub name: String,
    pub key_word: Vec<String>,
}

impl AreaCode {
    pub fn new(area_code_data: &[AreaCodeData]) -> Self {
        let mut code_data = HashMap::with_capacity(area_code_data.len());
        let mut code_area_data = HashMap::<String, AreaCodeIndex>::new();
        for tmp_area in area_code_data.iter() {
            let mut code_name = tmp_area.name.as_str();
            for tcc in CLEAR_WORD {
                if code_name.ends_with(tcc) {
                    code_name = code_name.trim_end_matches(tcc);
                }
            }
            code_data
                .entry(tmp_area.code.to_owned())
                .or_insert((tmp_area.hide, code_name.to_owned()));
            let code_str = Self::code_parse(&tmp_area.code);
            let mut pe_ref = &mut code_area_data;
            for ddd in code_str {
                pe_ref = &mut pe_ref
                    .entry(ddd.to_string())
                    .or_insert(AreaCodeIndex {
                        code: ddd.to_owned(),
                        childs: HashMap::new(),
                    })
                    .childs
            }
        }
        let mut hdata: HashMap<usize, HashMap<String, Vec<String>>> = HashMap::new();
        for te in area_code_data.iter() {
            let match_key_word = te
                .key_word
                .iter()
                .filter(|e| !e.trim().is_empty())
                .map(|e| e.trim().replace(' ', "").to_lowercase())
                .collect::<Vec<_>>();
            match hdata.entry(te.code.len()) {
                std::collections::hash_map::Entry::Occupied(mut htmp) => {
                    for tk in match_key_word.iter() {
                        match htmp.get_mut().entry(tk.to_owned()) {
                            std::collections::hash_map::Entry::Occupied(mut htmp1) => {
                                htmp1.get_mut().push(te.code.to_owned());
                            }
                            std::collections::hash_map::Entry::Vacant(htmp1) => {
                                htmp1.insert(vec![te.code.to_owned()]);
                            }
                        }
                    }
                }
                std::collections::hash_map::Entry::Vacant(htmp) => {
                    let mut add = HashMap::new();
                    for tk in match_key_word.iter() {
                        add.insert(tk.to_owned(), vec![te.code.to_owned()]);
                    }
                    htmp.insert(add);
                }
            };
        }
        let mut search_area_data: Vec<(usize, HashMap<String, Vec<String>>)> =
            hdata.into_iter().collect();
        search_area_data.sort_by_key(|&(k, _)| k);
        let search_detail_data = area_code_data
            .iter()
            .flat_map(|e| {
                if e.code.len() < 6 {
                    return None;
                }
                let code_str = Self::code_parse(&e.code);
                let mut tmp_data = Vec::with_capacity(4);
                let mut pe_ref = &code_area_data;
                let code_len = code_str.len();
                let code_len = if code_len > 0 { code_len - 1 } else { code_len };
                for (idn, ddd) in code_str.into_iter().enumerate() {
                    match pe_ref.get(ddd) {
                        Some(tmp) => {
                            if idn == code_len && !tmp.childs.is_empty() {
                                return None;
                            }
                            if let Some(tmp1) = code_data.get(&tmp.code) {
                                if tmp1.0 {
                                    return None;
                                }
                                tmp_data.push(Self::fix_name(&tmp1.1));
                            }
                            pe_ref = &tmp.childs;
                        }
                        None => return None,
                    }
                }
                Some(AreaSearchMoreIndex {
                    code: e.code.to_owned(),
                    detail_vec: tmp_data.join(""),
                })
            })
            .collect::<Vec<_>>();

        Self {
            code_data,
            code_area_data,
            search_area_data,
            search_detail_data,
        }
    }
    fn fix_name(name: &str) -> String {
        let mut name = name.trim();
        let mut is_i = vec![];
        for keys in [BAD_WORD, CLEAR_WORD] {
            for (i, tmp) in keys.iter().enumerate() {
                if is_i.contains(&i) {
                    continue;
                }
                if name.ends_with(tmp) {
                    name = name.trim_end_matches(tmp);
                    is_i.push(i);
                }
            }
        }
        name.to_owned()
    }
    pub(crate) fn code_parse(code: &str) -> Vec<&str> {
        let code = code.trim();
        if code.is_empty() {
            return vec![];
        }
        let len = code.len();
        let mut search_code = vec![];
        let mut start = 0;
        while len > start {
            if start < 6 {
                let end = start + 2;
                search_code.push(&code[0..if start + 2 < len { end } else { len }]);
                start = end;
            } else if start < 9 {
                let end = start + 3;
                search_code.push(&code[0..if start + 3 < len { end } else { len }]);
                start = end;
            } else {
                search_code.push(code);
                break;
            }
        }
        search_code
    }
    fn code_find<'t>(
        area_data: &'t HashMap<String, AreaCodeIndex>,
        code_data: &[&str],
    ) -> AreaResult<Vec<&'t AreaCodeIndex>> {
        if code_data.is_empty() {
            return Ok(vec![]);
        }
        if area_data.is_empty() {
            if !code_data.is_empty() {
                return Err(AreaError::NonFind(format!(
                    "not find code:{} [empty]",
                    code_data[0]
                )));
            } else {
                return Ok(vec![]);
            }
        }
        match area_data.get(code_data[0]) {
            Some(tmp) => {
                let mut out = vec![];
                out.push(tmp);
                out.extend(Self::code_find(&tmp.childs, &code_data[1..])?);
                Ok(out)
            }
            None => Err(AreaError::NonFind(format!(
                "not find code:{}",
                code_data[0]
            ))),
        }
    }
    /// 列出指定行政区域编码下的可用区域
    pub fn childs(&self, code: &str) -> AreaResult<Vec<AreaCodeItem>> {
        let code_data = Self::code_parse(code);
        if code_data.is_empty() {
            return Ok(self
                .code_area_data
                .iter()
                .flat_map(|(_, e)| {
                    let (c, n) = match self.code_data.get(&e.code) {
                        Some(tmp) => tmp.to_owned(),
                        None => (true, "[-.-]".to_string()),
                    };
                    if !c {
                        Some(AreaCodeItem {
                            code: e.code.to_owned(),
                            name: n,
                            leaf: e.childs.is_empty(),
                        })
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>());
        }
        let mut out = Self::code_find(&self.code_area_data, &code_data)?;
        Ok(out
            .pop()
            .map(|e| {
                e.childs
                    .iter()
                    .flat_map(|(_, e)| {
                        let (c, n) = match self.code_data.get(&e.code) {
                            Some(tmp) => tmp.to_owned(),
                            None => (true, "[-.-]".to_string()),
                        };
                        if !c {
                            Some(AreaCodeItem {
                                code: e.code.to_owned(),
                                name: n,
                                leaf: e.childs.is_empty(),
                            })
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default())
    }

    /// 通过行政区域编码解析出区域
    pub fn find(&self, code: &str) -> AreaResult<Vec<AreaCodeItem>> {
        let code_data = Self::code_parse(code);
        if code_data.is_empty() {
            return Ok(vec![]);
        }
        let out = Self::code_find(&self.code_area_data, &code_data)?;
        Ok(out
            .into_iter()
            .flat_map(|e| {
                let (c, n) = match self.code_data.get(&e.code) {
                    Some(tmp) => tmp.to_owned(),
                    None => (false, "".to_string()),
                };
                if !c {
                    Some(AreaCodeItem {
                        code: e.code.to_owned(),
                        name: n,
                        leaf: e.childs.is_empty(),
                    })
                } else {
                    None
                }
            })
            .collect::<Vec<_>>())
    }

    /// 获取行政区域编码同级区域的信息
    pub fn detail(&self, code: &str) -> AreaResult<Vec<Vec<AreaCodeDetailItem>>> {
        let code_data = Self::code_parse(code);
        if code_data.is_empty() {
            return Ok(vec![]);
        }
        let mut out_list = Vec::with_capacity(5);
        let mut now_list = Some(self.childs("")?);
        for ddd in code_data {
            let mut end = false;
            if let Some(tmp) = now_list {
                out_list.push(
                    tmp.into_iter()
                        .map(|e| {
                            let selected = if e.code == *ddd {
                                end = e.leaf;
                                true
                            } else {
                                false
                            };
                            AreaCodeDetailItem { selected, item: e }
                        })
                        .collect::<Vec<_>>(),
                );
            }
            if end {
                now_list = None;
                break;
            }
            now_list = Some(self.childs(ddd)?);
        }
        if let Some(tmp) = now_list {
            out_list.push(
                tmp.into_iter()
                    .map(|e| AreaCodeDetailItem {
                        selected: false,
                        item: e,
                    })
                    .collect::<Vec<_>>(),
            );
        }
        Ok(out_list)
    }

    /// 通过部分地址获取可能区域
    pub fn search(&self, name: &str, limit: usize) -> AreaResult<Vec<AreaSearchItem>> {
        let name = name.trim();
        if name.is_empty() {
            return Ok(vec![]);
        }
        let num_cpus = num_cpus::get();
        let ename = name.replace(' ', "").to_lowercase();
        let mut heap = BinaryHeap::new();
        for (_, btt) in self.search_area_data.iter() {
            // println!("{}", i);
            if let Some((find_key, find_code)) = btt.get_key_value(&ename) {
                for tcode in find_code.iter() {
                    let sim = match tcode.len() {
                        2 => 0,
                        4 => 1,
                        6 => 2,
                        9 => 3,
                        _ => 4,
                    };
                    heap.push(AreaSearchQueueItem {
                        sim,
                        code: tcode,
                        key_word: find_key,
                    });
                    if heap.len() > limit {
                        break;
                    }
                }
            }
        }
        if heap.len() < limit && !self.search_detail_data.is_empty() {
            let mut check_name = name.to_owned();
            for tccs in [BAD_WORD, CLEAR_WORD] {
                for tcc in tccs {
                    check_name = check_name.replace(tcc, "");
                }
            }
            let check_vec = check_name
                .split_whitespace()
                .flat_map(|word| {
                    if word.chars().all(|c| c.is_ascii()) {
                        vec![word.to_string()]
                    } else {
                        word.chars().map(|c| c.to_string()).collect()
                    }
                })
                .filter(|e| !e.is_empty())
                .collect::<Vec<_>>();
            let match_name_len = check_name.replace(' ', "").len();
            let add_limit = limit - heap.len();
            let result = self
                .search_detail_data
                .par_chunks(self.search_detail_data.len() / num_cpus)
                .map(|tcs| {
                    let mut heap = BinaryHeap::new();

                    for tmp in tcs.iter() {
                        if !check_name.is_empty() {
                            let mut all_find = true;
                            for ttp in check_vec.iter() {
                                if !tmp.detail_vec.contains(ttp) {
                                    all_find = false;
                                    break;
                                }
                            }
                            if !all_find {
                                continue;
                            }
                        }
                        let sim = (i64::abs(tmp.detail_vec.len() as i64 - match_name_len as i64)
                            + 10) as usize;
                        heap.push(AreaSearchQueueItem {
                            sim,
                            code: &tmp.code,
                            key_word: &tmp.detail_vec,
                        });
                        if heap.len() > add_limit {
                            break;
                        }
                    }
                    heap.into_vec()
                })
                .flat_map(|e| e)
                .collect::<Vec<_>>();
            for tmp_heap in result {
                heap.push(tmp_heap);
                if heap.len() > limit {
                    heap.pop();
                }
            }
        }
        let mut out = Vec::with_capacity(heap.len());
        while let Some(max) = heap.pop() {
            if let Ok(item) = self.find(max.code) {
                if item.is_empty() {
                    continue;
                }
                out.push(AreaSearchItem {
                    item,
                    key_word: max.key_word.to_owned(),
                })
            }
        }
        out.reverse();
        Ok(out)
    }
}
