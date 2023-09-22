const CLEAR_WORD: &[&str] = &[
    "-",
    "社区居委会",
    "居委会",
    "片区街道",
    "街道办",
    "委会",
    "民委员会",
];
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
    "片区街道",
    "街道办",
];

pub(crate) fn key_word_clear(name: &str) -> &str {
    let name = name.trim();
    if name.chars().count() <= 2 {
        return name;
    }
    for tmps in [BAD_WORD, CLEAR_WORD] {
        for tmp in tmps {
            if name.ends_with(tmp) {
                return name.trim_end_matches(tmp);
            }
        }
    }
    name
}

pub(crate) fn name_clear(name: &str) -> String {
    let mut tmp = name.trim();
    for tcc in CLEAR_WORD {
        if tmp.ends_with(tcc) {
            tmp = tmp.trim_end_matches(tcc);
        }
    }
    tmp.to_owned()
}
