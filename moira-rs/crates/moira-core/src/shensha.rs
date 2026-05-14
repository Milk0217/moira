use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ShenSha {
    pub name: String,
    pub category: String,
    pub quality: String,
}

const YEAR_BRANCH_SPIRITS: [(&str, &str, &str, &str); 12] = [
    ("将星", "吉", "华盖", "吉"),
    ("暗曜", "凶", "福星", "吉"),
    ("岁前", "凶", "丧门", "凶"),
    ("勾神", "凶", "绞神", "凶"),
    ("孤辰", "凶", "寡宿", "凶"),
    ("福星", "吉", "天德", "吉"),
    ("岁殿", "吉", "游奕", "凶"),
    ("孤辰", "凶", "寡宿", "凶"),
    ("天喜", "吉", "红鸾", "吉"),
    ("勾神", "凶", "绞神", "凶"),
    ("空亡", "凶", "擎天", "凶"),
    ("天喜", "吉", "红鸾", "吉"),
];

/// 根据出生年份的地支推算年支神煞
pub fn calculate_year_shensha(year: i32) -> Vec<ShenSha> {
    let branch = ((year - 1984) % 12 + 12) as usize % 12;
    let stem = ((year - 1984) % 10 + 10) as usize % 10;

    let mut list = vec![
        ShenSha { name: format!("太岁 {}", crate::bazi::EARTHLY_BRANCHES[branch]), category: "年支".into(), quality: "凶".into() },
        ShenSha { name: format!("岁破 {}", crate::bazi::EARTHLY_BRANCHES[(branch + 6) % 12]), category: "年支".into(), quality: "凶".into() },
    ];

    let lookup = |table: &[[usize; 4]]| -> Option<usize> {
        for &g in table {
            if g[..3].contains(&branch) { return Some(g[3]); }
        }
        None
    };

    if let Some(v) = lookup(&[[2, 6, 10, 8], [3, 9, 1, 11], [8, 0, 4, 2], [11, 3, 7, 5]]) {
        list.push(ShenSha { name: format!("驿马 {}", crate::bazi::EARTHLY_BRANCHES[v]), category: "年支".into(), quality: "吉".into() });
    }
    if let Some(v) = lookup(&[[2, 6, 10, 3], [3, 9, 1, 6], [8, 0, 4, 9], [11, 3, 7, 0]]) {
        list.push(ShenSha { name: format!("桃花 {}", crate::bazi::EARTHLY_BRANCHES[v]), category: "年支".into(), quality: "吉".into() });
    }
    if let Some(v) = lookup(&[[8, 0, 4, 5], [2, 6, 10, 11], [3, 9, 1, 2], [11, 3, 7, 8]]) {
        list.push(ShenSha { name: format!("劫煞 {}", crate::bazi::EARTHLY_BRANCHES[v]), category: "年支".into(), quality: "凶".into() });
    }
    if let Some(v) = lookup(&[[8, 0, 4, 6], [2, 6, 10, 0], [3, 9, 1, 3], [11, 3, 7, 9]]) {
        list.push(ShenSha { name: format!("灾煞 {}", crate::bazi::EARTHLY_BRANCHES[v]), category: "年支".into(), quality: "凶".into() });
    }

    let (s1, q1, s2, q2) = YEAR_BRANCH_SPIRITS[branch];
    list.push(ShenSha { name: format!("{} {}", s1, crate::bazi::EARTHLY_BRANCHES[branch]), category: "年支".into(), quality: q1.into() });
    list.push(ShenSha { name: format!("{} {}", s2, crate::bazi::EARTHLY_BRANCHES[branch]), category: "年支".into(), quality: q2.into() });

    list.push(ShenSha { name: format!("禄神 {}", crate::bazi::HEAVENLY_STEMS[stem]), category: "年干".into(), quality: "吉".into() });
    let tian_yi = match stem { 0 | 4 => "丑未", 1 | 5 => "子申", 2 | 6 => "酉亥", 3 | 7 => "卯巳", 8 | 9 => "午寅", _ => "" };
    list.push(ShenSha { name: format!("天乙贵人 {}", tian_yi), category: "年干".into(), quality: "吉".into() });

    list
}

const TIAN_DE_BY_DAY: [&str; 10] = ["寅", "酉", "巳", "子", "申", "申", "寅", "巳", "巳", "巳"];
const YUE_DE_BY_DAY: [&str; 10] = ["丙", "申", "壬", "甲", "午", "壬", "丙", "甲", "巳", "壬"];
const JIN_YU_BY_DAY: [&str; 10] = ["辰", "巳", "未", "申", "丑", "寅", "卯", "亥", "午", "未"];
const HONG_YAN_BY_DAY: [&str; 10] = ["午", "申", "寅", "亥", "未", "辰", "戌", "酉", "子", "巳"];

const SHEN_SHA_NAMES: [&str; 4] = ["天德", "月德", "金舆", "红艳"];

pub fn calculate_day_stem_shensha(day_stem_index: usize) -> Vec<ShenSha> {
    let idx = day_stem_index % 10;
    let values = [TIAN_DE_BY_DAY[idx], YUE_DE_BY_DAY[idx], JIN_YU_BY_DAY[idx], HONG_YAN_BY_DAY[idx]];
    let mut result: Vec<ShenSha> = (0..4)
        .map(|i| ShenSha { name: format!("{} {}", SHEN_SHA_NAMES[i], values[i]), category: "日干".into(), quality: "吉".into() })
        .collect();

    if matches!(idx, 0 | 2 | 6 | 7 | 8 | 9) {
        result.push(ShenSha { name: "文昌".into(), category: "日干".into(), quality: "吉".into() });
    }
    result.push(ShenSha { name: "天赦".into(), category: "日干".into(), quality: "吉".into() });
    result
}

const HOUR_SHENSHA: [&str; 12] = [
    "青龙", "明堂", "天刑", "朱雀", "金匮", "天德",
    "青龙", "明堂", "天刑", "朱雀", "金匮", "天德",
];

pub fn calculate_hour_shensha(hour_branch_index: usize) -> Vec<ShenSha> {
    vec![ShenSha { name: HOUR_SHENSHA[hour_branch_index % 12].to_string(), category: "时辰".into(), quality: "吉".into() }]
}

/// 计算全部神煞（含年支、日干、时辰）
pub fn calculate_all_shensha(year: i32, month: u8, day_stem_index: usize, hour_branch_index: usize) -> Vec<ShenSha> {
    let _ = month;
    let mut result = calculate_year_shensha(year);
    result.extend(calculate_day_stem_shensha(day_stem_index));
    result.extend(calculate_hour_shensha(hour_branch_index));
    let mut seen_names = Vec::new();
    result.retain(|s| {
        if seen_names.contains(&s.name) { false }
        else { seen_names.push(s.name.clone()); true }
    });
    result
}
