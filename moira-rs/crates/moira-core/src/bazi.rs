use serde::{Deserialize, Serialize};

pub const HEAVENLY_STEMS: [&str; 10] = ["甲", "乙", "丙", "丁", "戊", "己", "庚", "辛", "壬", "癸"];
pub const EARTHLY_BRANCHES: [&str; 12] = ["子", "丑", "寅", "卯", "辰", "巳", "午", "未", "申", "酉", "戌", "亥"];

const SHI_SHEN: [[&str; 10]; 10] = [
    // 日干\他干: 甲  乙  丙  丁  戊  己  庚  辛  壬  癸
    /* 甲 */ ["比肩","劫财","食神","伤官","偏财","正财","七杀","正官","偏印","正印"],
    /* 乙 */ ["劫财","比肩","伤官","食神","正财","偏财","正官","七杀","正印","偏印"],
    /* 丙 */ ["偏印","正印","比肩","劫财","食神","伤官","偏财","正财","七杀","正官"],
    /* 丁 */ ["正印","偏印","劫财","比肩","伤官","食神","正财","偏财","正官","七杀"],
    /* 戊 */ ["七杀","正官","偏印","正印","比肩","劫财","食神","伤官","偏财","正财"],
    /* 己 */ ["正官","七杀","正印","偏印","劫财","比肩","伤官","食神","正财","偏财"],
    /* 庚 */ ["偏财","正财","七杀","正官","偏印","正印","比肩","劫财","食神","伤官"],
    /* 辛 */ ["正财","偏财","正官","七杀","正印","偏印","劫财","比肩","伤官","食神"],
    /* 壬 */ ["食神","伤官","偏财","正财","七杀","正官","偏印","正印","比肩","劫财"],
    /* 癸 */ ["伤官","食神","正财","偏财","正官","七杀","正印","偏印","劫财","比肩"],
];

/// 长生十二宫: 每种五行天干对应的地支状态
const LIFE_CYCLE: [[usize; 12]; 10] = [
    // 甲木: 亥(长生),子(沐浴),丑(冠带),寅(临官),卯(帝旺),辰(衰),巳(病),午(死),未(墓),申(绝),酉(胎),戌(养)
    [11,0,1,2,3,4,5,6,7,8,9,10],
    // 乙木: 午(长生),巳(沐浴),辰(冠带),卯(临官),寅(帝旺),丑(衰),子(病),亥(死),戌(墓),酉(绝),申(胎),未(养)
    [7,6,5,4,3,2,1,0,11,10,9,8],
    // 丙火: 寅(长生),卯(沐浴),辰(冠带),巳(临官),午(帝旺),未(衰),申(病),酉(死),戌(墓),亥(绝),子(胎),丑(养)
    [2,3,4,5,6,7,8,9,10,11,0,1],
    // 丁火: 酉(长生),申(沐浴),未(冠带),午(临官),巳(帝旺),辰(衰),卯(病),寅(死),丑(墓),子(绝),亥(胎),戌(养)
    [10,9,8,7,6,5,4,3,2,1,0,11],
    // 戊土(同丙): 寅(长生),卯(沐浴),辰(冠带),巳(临官),午(帝旺),未(衰),申(病),酉(死),戌(墓),亥(绝),子(胎),丑(养)
    [2,3,4,5,6,7,8,9,10,11,0,1],
    // 己土(同丁): 酉(长生),申(沐浴),未(冠带),午(临官),巳(帝旺),辰(衰),卯(病),寅(死),丑(墓),子(绝),亥(胎),戌(养)
    [10,9,8,7,6,5,4,3,2,1,0,11],
    // 庚金: 巳(长生),午(沐浴),未(冠带),申(临官),酉(帝旺),戌(衰),亥(病),子(死),丑(墓),寅(绝),卯(胎),辰(养)
    [5,6,7,8,9,10,11,0,1,2,3,4],
    // 辛金: 子(长生),亥(沐浴),戌(冠带),酉(临官),申(帝旺),未(衰),午(病),巳(死),辰(墓),卯(绝),寅(胎),丑(养)
    [0,11,10,9,8,7,6,5,4,3,2,1],
    // 壬水: 申(长生),酉(沐浴),戌(冠带),亥(临官),子(帝旺),丑(衰),寅(病),卯(死),辰(墓),巳(绝),午(胎),未(养)
    [8,9,10,11,0,1,2,3,4,5,6,7],
    // 癸水: 卯(长生),寅(沐浴),丑(冠带),子(临官),亥(帝旺),戌(衰),酉(病),申(死),未(墓),午(绝),巳(胎),辰(养)
    [3,2,1,0,11,10,9,8,7,6,5,4],
];

const LIFE_STAGES: [&str; 12] = [
    "长生", "沐浴", "冠带", "临官", "帝旺", "衰", "病", "死", "墓", "绝", "胎", "养",
];

/// 地支藏干: 每个地支的主气、中气、余气
const HIDDEN_STEMS: [[(usize, &str); 3]; 12] = [
    [(0, "癸")         , (0, "")   , (0, "")   ],  // 子
    [(5, "己"), (0, "癸"), (8, "辛")],  // 丑
    [(0, "甲"), (4, "丙"), (7, "戊")],  // 寅
    [(0, "乙")         , (0, "")   , (0, "")   ],  // 卯
    [(5, "戊"), (0, "乙"), (3, "癸")],  // 辰
    [(4, "丙"), (0, "庚"), (5, "戊")],  // 巳
    [(1, "丁"), (5, "己")         , (0, "")   ],  // 午
    [(1, "丁"), (0, "己")         , (0, "")   ],  // 未
    [(2, "庚"), (6, "壬"), (5, "戊")],  // 申
    [(0, "辛")         , (0, "")   , (0, "")   ],  // 酉
    [(4, "戊"), (0, "辛"), (1, "丁")],  // 戌
    [(6, "壬"), (0, "甲")         , (0, "")   ],  // 亥
];

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Pillar {
    pub heavenly_stem: String,
    pub earthly_branch: String,
    pub stem_index: u8,
    pub branch_index: u8,
}

impl Pillar {
    pub fn new(stem_index: usize, branch_index: usize) -> Self {
        Pillar {
            heavenly_stem: HEAVENLY_STEMS[stem_index % 10].to_string(),
            earthly_branch: EARTHLY_BRANCHES[branch_index % 12].to_string(),
            stem_index: (stem_index % 10) as u8,
            branch_index: (branch_index % 12) as u8,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DayunPillar {
    pub age: u8,
    pub heavenly_stem: String,
    pub earthly_branch: String,
    pub stem_index: u8,
    pub branch_index: u8,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ShishenItem {
    pub pillar_name: String,
    pub stem: String,
    pub shishen: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HiddenStemInfo {
    pub branch_name: String,
    pub stems: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LifeCycleItem {
    pub branch_name: String,
    pub stage: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BaziData {
    pub year_pillar: Pillar,
    pub month_pillar: Pillar,
    pub day_pillar: Pillar,
    pub hour_pillar: Pillar,
    pub dayun: Vec<DayunPillar>,
    pub shishen: Vec<ShishenItem>,
    pub hidden_stems: Vec<HiddenStemInfo>,
    pub life_cycle: Vec<LifeCycleItem>,
    pub taiyuan: Pillar,
}

/// 计算儒略日号 (Julian Day Number)
pub fn julian_day_number(year: i32, month: u8, day: u8) -> i64 {
    let a = (14 - month as i32) / 12;
    let y = year as i64 + 4800 - a as i64;
    let m = month as i64 + 12 * a as i64 - 3;
    day as i64 + (153 * m + 2) / 5 + 365 * y + y / 4 - y / 100 + y / 400 - 32045
}

/// 计算年柱 (含年干年支)
pub fn calculate_year_pillar(year: i32) -> Pillar {
    let stem = ((year - 4) % 10 + 10) as usize % 10;
    let branch = ((year - 4) % 12 + 12) as usize % 12;
    Pillar::new(stem, branch)
}

/// 计算月柱 (简化: 以公历月为界，后续用节气修正)
pub fn calculate_month_pillar(year: i32, month: u8) -> Pillar {
    let year_stem = ((year - 4) % 10 + 10) as usize % 10;
    // 月支: 寅=1(正月), 卯=2(二月)... 丑=12(腊月)
    let branch = (month as usize + 1) % 12;
    // 五虎遁: month_stem = (year_stem * 2 + month_branch) % 10
    let stem = (year_stem * 2 + branch) % 10;
    Pillar::new(stem, branch)
}

/// 计算日柱 (基于 Julian Day Number)
pub fn calculate_day_pillar(year: i32, month: u8, day: u8) -> Pillar {
    let jdn = julian_day_number(year, month, day);
    // 已知 JDN 2415021(1900-01-01) = 甲戌, stem=0, branch=10
    let stem = ((jdn + 9) % 10) as usize;
    let branch = ((jdn + 1) % 12) as usize;
    Pillar::new(stem, branch)
}

/// 计算时柱
pub fn calculate_hour_pillar(day_stem_index: usize, hour: u8) -> Pillar {
    let hour_branch = ((hour as usize + 1) / 2) % 12;
    let stem = (day_stem_index * 2 + hour_branch) % 10;
    Pillar::new(stem, hour_branch)
}

/// 计算十神关系
pub fn calculate_shishen(pillars: &[(&str, &Pillar)]) -> Vec<ShishenItem> {
    let day_stem = pillars.iter().find(|(n, _)| *n == "日柱").map(|(_, p)| p.stem_index as usize).unwrap_or(0);
    let mut result = Vec::new();
    for (name, pillar) in pillars {
        let other_stem = pillar.stem_index as usize;
        let ss = SHI_SHEN[day_stem][other_stem];
        result.push(ShishenItem {
            pillar_name: name.to_string(),
            stem: pillar.heavenly_stem.clone(),
            shishen: ss.to_string(),
        });
    }
    result
}

/// 计算大运 (简化: 默认顺排，起运年龄从1岁开始)
pub fn calculate_dayun(month_pillar: &Pillar, _year: i32, _month: u8, _day: u8) -> Vec<DayunPillar> {
    let start_stem = month_pillar.stem_index as usize;
    let start_branch = month_pillar.branch_index as usize;
    let mut result = Vec::new();
    for i in 0..8 {
        let stem = (start_stem + i + 1) % 10;
        let branch = (start_branch + i + 1) % 12;
        result.push(DayunPillar {
            age: (i as u8) * 10 + 1,
            heavenly_stem: HEAVENLY_STEMS[stem].to_string(),
            earthly_branch: EARTHLY_BRANCHES[branch].to_string(),
            stem_index: stem as u8,
            branch_index: branch as u8,
        });
    }
    result
}

/// 计算藏干
pub fn calculate_hidden_stems(branches_info: &[(String, usize)]) -> Vec<HiddenStemInfo> {
    branches_info
        .iter()
        .map(|(name, branch_idx)| {
            let entry = HIDDEN_STEMS[*branch_idx];
            let stems: Vec<String> = entry
                .iter()
                .filter(|(_, s)| !s.is_empty())
                .map(|(_, s)| s.to_string())
                .collect();
            HiddenStemInfo {
                branch_name: name.clone(),
                stems,
            }
        })
        .collect()
}

/// 计算长生十二宫
pub fn calculate_life_cycle(day_stem_index: usize) -> Vec<LifeCycleItem> {
    let mapping = LIFE_CYCLE[day_stem_index];
    EARTHLY_BRANCHES
        .iter()
        .enumerate()
        .map(|(branch_idx, branch_name)| {
            let stage_idx = mapping.iter().position(|&m| m == branch_idx).unwrap_or(0);
            LifeCycleItem {
                branch_name: branch_name.to_string(),
                stage: LIFE_STAGES[stage_idx].to_string(),
            }
        })
        .collect()
}

/// 计算胎元 (月柱天干+1, 月柱地支+3)
pub fn calculate_taiyuan(month_pillar: &Pillar) -> Pillar {
    let stem = (month_pillar.stem_index as usize + 1) % 10;
    let branch = (month_pillar.branch_index as usize + 3) % 12;
    Pillar::new(stem, branch)
}

/// 完整八字计算
pub fn calculate_bazi(year: i32, month: u8, day: u8, hour: u8) -> BaziData {
    let year_pillar = calculate_year_pillar(year);
    let month_pillar = calculate_month_pillar(year, month);
    let day_pillar = calculate_day_pillar(year, month, day);
    let hour_pillar = calculate_hour_pillar(day_pillar.stem_index as usize, hour);

    let dayun = calculate_dayun(&month_pillar, year, month, day);

    let pillars_info = [
        ("年柱", &year_pillar),
        ("月柱", &month_pillar),
        ("日柱", &day_pillar),
        ("时柱", &hour_pillar),
    ];
    let shishen = calculate_shishen(&pillars_info);

    let branches_info = [
        (year_pillar.earthly_branch.clone(), year_pillar.branch_index as usize),
        (month_pillar.earthly_branch.clone(), month_pillar.branch_index as usize),
        (day_pillar.earthly_branch.clone(), day_pillar.branch_index as usize),
        (hour_pillar.earthly_branch.clone(), hour_pillar.branch_index as usize),
    ];
    let hidden_stems = calculate_hidden_stems(&branches_info);

    let life_cycle = calculate_life_cycle(day_pillar.stem_index as usize);
    let taiyuan = calculate_taiyuan(&month_pillar);

    BaziData {
        year_pillar,
        month_pillar,
        day_pillar,
        hour_pillar,
        dayun,
        shishen,
        hidden_stems,
        life_cycle,
        taiyuan,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_year_pillar_2024() {
        let p = calculate_year_pillar(2024);
        // 2024: 甲辰 (stem=0, branch=4)
        assert_eq!(p.heavenly_stem, "甲");
        assert_eq!(p.earthly_branch, "辰");
    }

    #[test]
    fn test_year_pillar_1984() {
        let p = calculate_year_pillar(1984);
        assert_eq!(p.heavenly_stem, "甲");
        assert_eq!(p.earthly_branch, "子");
    }

    #[test]
    fn test_month_pillar() {
        // 2024年正月 (month=1): 甲己年正月丙寅
        let p = calculate_month_pillar(2024, 1);
        assert_eq!(p.heavenly_stem, "丙");
        assert_eq!(p.earthly_branch, "寅");
    }

    #[test]
    fn test_day_pillar_19000101() {
        // 1900-01-01: 甲戌
        let p = calculate_day_pillar(1900, 1, 1);
        assert_eq!(p.heavenly_stem, "甲");
        assert_eq!(p.earthly_branch, "戌");
    }

    #[test]
    fn test_day_pillar_20000101() {
        // 2000-01-01: 戊午
        let p = calculate_day_pillar(2000, 1, 1);
        assert_eq!(p.heavenly_stem, "戊");
        assert_eq!(p.earthly_branch, "午");
    }

    #[test]
    fn test_hour_pillar() {
        // 甲日(0)子时(0): 甲子
        let p = calculate_hour_pillar(0, 0);
        assert_eq!(p.heavenly_stem, "甲");
        assert_eq!(p.earthly_branch, "子");
    }

    #[test]
    fn test_hour_pillar_afternoon() {
        // 甲日(0)午时(12): hour_branch=6 → 庚午
        let p = calculate_hour_pillar(0, 12);
        assert_eq!(p.earthly_branch, "午");
        assert_eq!(p.heavenly_stem, "庚");
    }

    #[test]
    fn test_julian_day_number() {
        let jdn = julian_day_number(2000, 1, 1);
        assert_eq!(jdn, 2451545);
    }

    #[test]
    fn test_taiyuan() {
        let month = Pillar::new(2, 2); // 丙寅
        let ty = calculate_taiyuan(&month);
        assert_eq!(ty.heavenly_stem, "丁");
        assert_eq!(ty.earthly_branch, "巳");
    }

    #[test]
    fn test_life_cycle_甲() {
        let lc = calculate_life_cycle(0);
        // 甲木: 亥(长生), 酉(胎)
        assert_eq!(lc[11].stage, "长生");
        assert_eq!(lc[9].stage, "胎");
    }

    #[test]
    fn test_hidden_stems_子() {
        let hs = calculate_hidden_stems(&[("子".into(), 0)]);
        assert_eq!(hs[0].stems, vec!["癸"]);
    }

    #[test]
    fn test_hidden_stems_寅() {
        let hs = calculate_hidden_stems(&[("寅".into(), 2)]);
        assert_eq!(hs[0].stems, vec!["甲", "丙", "戊"]);
    }

    #[test]
    fn test_shishen_甲日() {
        let day = Pillar::new(0, 0); // 甲子
        let month = Pillar::new(2, 2); // 丙寅
        let items = calculate_shishen(&[("日柱", &day), ("月柱", &month)]);
        // 甲日见丙 = 食神
        assert_eq!(items[1].shishen, "食神");
    }

    #[test]
    fn test_dayun() {
        let month = Pillar::new(2, 2); // 丙寅
        let dy = calculate_dayun(&month, 2024, 1, 1);
        assert_eq!(dy.len(), 8);
        assert_eq!(dy[0].heavenly_stem, "丁");
        assert_eq!(dy[0].age, 1);
    }
}
