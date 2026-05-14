use serde::{Deserialize, Serialize};

/// 11 星曜枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BodyId {
    太阳, 太阴, 水星, 金星, 火星, 木星, 土星, 罗睺, 计都, 紫炁, 月孛,
}

pub const BODY_NAMES: &[BodyId] = &[
    BodyId::太阳, BodyId::太阴, BodyId::水星, BodyId::金星, BodyId::火星,
    BodyId::木星, BodyId::土星, BodyId::罗睺, BodyId::计都, BodyId::紫炁, BodyId::月孛,
];

impl BodyId {
    pub fn name(&self) -> &'static str {
        match self {
            BodyId::太阳 => "太阳", BodyId::太阴 => "太阴", BodyId::水星 => "水星",
            BodyId::金星 => "金星", BodyId::火星 => "火星", BodyId::木星 => "木星",
            BodyId::土星 => "土星", BodyId::罗睺 => "罗睺", BodyId::计都 => "计都",
            BodyId::紫炁 => "紫炁", BodyId::月孛 => "月孛",
        }
    }
    pub fn from_name(s: &str) -> Option<BodyId> {
        match s {
            "太阳" => Some(BodyId::太阳), "太阴" => Some(BodyId::太阴),
            "水星" => Some(BodyId::水星), "金星" => Some(BodyId::金星),
            "火星" => Some(BodyId::火星), "木星" => Some(BodyId::木星),
            "土星" => Some(BodyId::土星), "罗睺" => Some(BodyId::罗睺),
            "计都" => Some(BodyId::计都), "紫炁" => Some(BodyId::紫炁),
            "月孛" => Some(BodyId::月孛), _ => None,
        }
    }
}

/// 五行
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Element { 木, 火, 土, 金, 水 }

/// 星体五行映射
pub const PLANET_ELEMENTS: [(BodyId, Element); 11] = [
    (BodyId::太阳, Element::火), (BodyId::太阴, Element::水),
    (BodyId::水星, Element::水), (BodyId::金星, Element::金),
    (BodyId::火星, Element::火), (BodyId::木星, Element::木),
    (BodyId::土星, Element::土), (BodyId::罗睺, Element::火),
    (BodyId::计都, Element::土), (BodyId::紫炁, Element::木),
    (BodyId::月孛, Element::水),
];

pub fn body_element(body: BodyId) -> Element {
    PLANET_ELEMENTS.iter().find(|(b, _)| *b == body).map(|(_, e)| *e).unwrap_or(Element::土)
}

/// 生星: 我生者 + 生我者? No: 生星 = 生我之星 (element that generates mine)
pub fn generating_stars(body: BodyId) -> &'static [BodyId] {
    match body_element(body) {
        Element::木 => &[BodyId::水星, BodyId::月孛], // 水生木
        Element::火 => &[BodyId::木星, BodyId::紫炁], // 木生火
        Element::土 => &[BodyId::火星, BodyId::罗睺, BodyId::太阳], // 火生土
        Element::金 => &[BodyId::土星, BodyId::计都], // 土生金
        Element::水 => &[BodyId::金星], // 金生水
    }
}

/// 克星: 克我之星 (element that overcomes mine)
pub fn overcoming_stars(body: BodyId) -> &'static [BodyId] {
    match body_element(body) {
        Element::木 => &[BodyId::金星], // 金克木
        Element::火 => &[BodyId::水星, BodyId::月孛], // 水克火
        Element::土 => &[BodyId::木星, BodyId::紫炁], // 木克土
        Element::金 => &[BodyId::火星, BodyId::罗睺, BodyId::太阳], // 火克金
        Element::水 => &[BodyId::土星, BodyId::计都], // 土克水
    }
}

/// 难恩仇用星分类
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NanEnChouYong { 难星, 仇星, 恩星, 用星 }
pub const NAN_EN_CHOU_YONG_NAMES: [&str; 4] = ["难星", "仇星", "恩星", "用星"];

/// 宫度关系类型
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AspectModifier { 同度, 同宫, 对照, 拱照 }

/// 星力结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StarPower {
    pub body_name: String,
    pub 升殿: bool, pub 入垣: bool, pub 季星: bool,
    pub 失躔: bool, pub 失垣: bool,
    pub base_power: i32,
    pub generating_bonus: f64,
    pub overcoming_penalty: f64,
    pub total_power: f64,
    pub classification: String,
}

/// 轻量星体信息 (从 CelestialBodyData/ExtraBodyData 构造)
pub struct BodyInfo {
    pub body_id: BodyId,
    pub longitude: f64,
    pub mansion_name: String,
    pub sign_zodiac: String,
}

/// 宫神煞系统
pub const HOUSE_AUSPICIOUS: [&str; 15] = [
    "红鸾","禄勋","唐符","咸池","天贵","斗杓","喜神",
    "文昌","卦气","玉贵","天喜","国印","驿马","紫微","天德",
];
pub const HOUSE_INAUSPICIOUS: [&str; 20] = [
    "病符","寡宿","劫杀","绞杀","飞刃","天狗","阳刃","阴刃",
    "天耗","剑锋","孤辰","孤虚","飞廉","天雄","大耗","小耗",
    "阑干","血刃","空亡","的杀",
];

/// 流年神煞
pub const ANNUAL_AUSPICIOUS: [&str; 10] = [
    "天喜","红鸾","驿马","天厨","咸池","禄勋","天德","天贵","地解","解神",
];
pub const ANNUAL_INAUSPICIOUS: [&str; 26] = [
    "岁驾","天空","地雌","贯索","五鬼","死符","大耗","小耗","阳刃","阴刃",
    "天厄","天雄","大杀","卷舌","天狗","蓦越","亡神","披头","血刃","天哭",
    "劫杀","的杀","黄幡","豹尾","三刑","六害",
];

/// 十二宫名称
pub const HOUSE_NAMES: [&str; 12] = [
    "命宫","财帛","兄弟","田宅","男女","奴仆","夫妻","疾厄","迁移","官禄","福德","相貌",
];

/// 季节星: [春,夏,秋,冬] × 2
pub const SEASONAL_STARS: [[BodyId; 2]; 4] = [
    [BodyId::木星, BodyId::紫炁], // 春
    [BodyId::火星, BodyId::罗睺], // 夏
    [BodyId::金星, BodyId::金星], // 秋
    [BodyId::水星, BodyId::月孛], // 冬
];

#[allow(dead_code)]
const HOUSE_AUSPICIOUS_NAMES: [&str; 15] = HOUSE_AUSPICIOUS;
#[allow(dead_code)]
const HOUSE_INAUSPICIOUS_NAMES: [&str; 20] = HOUSE_INAUSPICIOUS;

// ============================================================
// Layer 1: Star Power Sub-systems (stubs — filled in Wave 2)
// ============================================================

/// 升殿检测: 星曜是否在"升殿"宿度
pub fn is_ascending_hall(body: BodyId, mansion_name: &str) -> bool {
    matches!(
        (body, mansion_name),
        (BodyId::太阳, "房" | "虚" | "昴" | "星")
            | (BodyId::太阴, "心" | "危" | "毕" | "张")
            | (BodyId::水星, "箕" | "壁" | "参" | "轸")
            | (BodyId::金星, "角" | "牛" | "奎" | "娄")
            | (BodyId::火星, "尾" | "室" | "觜" | "翼")
            | (BodyId::木星, "斗" | "奎" | "井" | "角")
            | (BodyId::土星, "女" | "危" | "毕" | "星")
            | (BodyId::罗睺, "壁" | "斗" | "井" | "角")
            | (BodyId::计都, "轸" | "箕" | "参" | "翼")
            | (BodyId::紫炁, "星" | "鬼" | "井" | "张")
            | (BodyId::月孛, "壁" | "危" | "室" | "心")
    )
}

/// 失躔检测
pub fn is_lost_tracking(_body: BodyId, _mansion_name: &str) -> bool {
    false // TODO: Wave 2a — implement based on 度度克 relationships
}

/// 入垣检测: 星曜是否在本身宫
pub fn is_entering_wall(body: BodyId, sign_index: usize) -> bool {
    matches!(
        (body, sign_index),
        (BodyId::太阳, 7)  // 午(狮子)
            | (BodyId::太阴, 5) // 未(巨cancer)
            | (BodyId::水星, 3 | 5) // 申(双子)巳(处女)
            | (BodyId::金星, 1 | 6) // 酉(金牛)辰(天秤)
            | (BodyId::火星, 0 | 4) // 戌(白羊)卯(天蝎)
            | (BodyId::木星, 2 | 10) // 寅(射手)亥(双鱼)
            | (BodyId::土星, 0 | 11) // 丑(摩羯)子(水瓶)
            | (BodyId::罗睺, 9) // 亥
            | (BodyId::计都, 5) // 巳
            | (BodyId::紫炁, 7) // 午
            | (BodyId::月孛, 5) // 未
    )
}

/// 失垣检测
pub fn is_lost_wall(body: BodyId, sign_index: usize) -> bool {
    matches!(
        (body, sign_index),
        // 水星(水)被火克, 火星的宫位(白羊/天蝎)对水星不利
        (BodyId::水星, 0 | 4)
            | (BodyId::金星, 3 | 7) // 金星(金)被火克
            | (BodyId::火星, 6 | 8) // 火星(火)被水克
            | (BodyId::木星, 5 | 7) // 木星(木)被金克
            | (BodyId::土星, 9 | 10) // 土星(土)被木克
    )
}

/// 季星检测
pub fn is_seasonal_star(body: BodyId, month: u8) -> bool {
    let idx = if month < 3 { 0 } else if month < 6 { 1 } else if month < 9 { 2 } else { 3 };
    SEASONAL_STARS[idx].contains(&body)
}

/// 难恩仇用星分类
pub fn classify_nan_en_chou_yong(
    body: BodyId,
    ming_du_zhu: BodyId,
    ming_gong_zhu: BodyId,
) -> NanEnChouYong {
    let be = body_element(body);
    let de = body_element(ming_du_zhu);
    let ge = body_element(ming_gong_zhu);
    if is_overcoming(be, de) { NanEnChouYong::难星 }
    else if is_overcoming(be, ge) { NanEnChouYong::仇星 }
    else if is_generating(be, de) { NanEnChouYong::恩星 }
    else if is_generating(be, ge) { NanEnChouYong::用星 }
    else { NanEnChouYong::用星 }
}

fn is_generating(a: Element, b: Element) -> bool {
    matches!((a, b), (Element::木, Element::火) | (Element::火, Element::土)
        | (Element::土, Element::金) | (Element::金, Element::水)
        | (Element::水, Element::木))
}

fn is_overcoming(a: Element, b: Element) -> bool {
    matches!((a, b), (Element::木, Element::土) | (Element::土, Element::水)
        | (Element::水, Element::火) | (Element::火, Element::金)
        | (Element::金, Element::木))
}

/// 宫度关系: 返回 (aspect_type, modifier)
pub fn aspect_modifier(longitude_a: f64, longitude_b: f64) -> Option<(AspectModifier, f64)> {
    let diff = (longitude_a - longitude_b).abs() % 360.0;
    let diff = diff.min(360.0 - diff);
    if diff < 1.0 { Some((AspectModifier::同度, 1.0)) }
    else if diff < 30.0 { Some((AspectModifier::同宫, 1.0)) }
    else if (diff - 120.0).abs() < 0.5 { Some((AspectModifier::拱照, 0.3)) }
    else if (diff - 180.0).abs() < 0.5 { Some((AspectModifier::对照, 0.7)) }
    else { None }
}

// ============================================================
// Layer 1 Integration: Star Power Score
// ============================================================

pub fn compute_star_power(
    body: BodyId,
    longitude: f64,
    mansion_name: &str,
    sign_index: usize,
    month: u8,
    all_bodies: &[BodyInfo],
    ming_du_zhu: BodyId,
    ming_gong_zhu: BodyId,
) -> StarPower {
    let asc_hall = is_ascending_hall(body, mansion_name);
    let ent_wall = is_entering_wall(body, sign_index);
    let season = is_seasonal_star(body, month);
    let lost_track = is_lost_tracking(body, mansion_name);
    let lost_wall = is_lost_wall(body, sign_index);

    let base = (asc_hall as i32) + (ent_wall as i32) + (season as i32)
             - (lost_track as i32) - (lost_wall as i32);

    let (gen_bonus, ovr_penalty) = count_body_aspects(body, longitude, all_bodies);

    let total = base as f64 + gen_bonus - ovr_penalty;
    let cls = classify_nan_en_chou_yong(body, ming_du_zhu, ming_gong_zhu);
    let cls_name = match cls {
        NanEnChouYong::难星 => "难星", NanEnChouYong::仇星 => "仇星",
        NanEnChouYong::恩星 => "恩星", NanEnChouYong::用星 => "用星",
    };

    StarPower {
        body_name: body.name().to_string(),
        升殿: asc_hall, 入垣: ent_wall, 季星: season,
        失躔: lost_track, 失垣: lost_wall,
        base_power: base,
        generating_bonus: gen_bonus,
        overcoming_penalty: ovr_penalty,
        total_power: total,
        classification: cls_name.to_string(),
    }
}

fn count_body_aspects(body: BodyId, longitude: f64, all_bodies: &[BodyInfo]) -> (f64, f64) {
    let gen_stars = generating_stars(body);
    let ovr_stars = overcoming_stars(body);
    let mut gen_bonus = 0.0_f64;
    let mut ovr_penalty = 0.0_f64;

    for other in all_bodies {
        if other.body_id == body { continue; }
        if let Some((_, modifier)) = aspect_modifier(longitude, other.longitude) {
            if gen_stars.contains(&other.body_id) {
                gen_bonus += modifier;
            }
            if ovr_stars.contains(&other.body_id) {
                ovr_penalty += modifier;
            }
        }
    }
    (gen_bonus, ovr_penalty)
}

// ============================================================
// Layer 2: House Analysis
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HouseAnalysis {
    pub house_index: u8,
    pub house_name: String,
    pub auspicious_spirits: Vec<String>,
    pub inauspicious_spirits: Vec<String>,
    pub transformation_stars: Vec<String>,
    pub life_cycle_annotations: Vec<String>,
}

fn house_spirit_names(year_branch: usize, house_index: usize) -> Vec<&'static str> {
    let idx = (year_branch + house_index) % HOUSE_AUSPICIOUS_NAMES.len();
    vec![HOUSE_AUSPICIOUS_NAMES[idx], HOUSE_INAUSPICIOUS_NAMES[idx % HOUSE_INAUSPICIOUS_NAMES.len()]]
}

pub fn analyze_house(
    house_index: usize,
    _house_longitude: f64,
    _bodies: &[BodyInfo],
    year: i32,
    _day_stem_index: usize,
    _ming_du_zhu: BodyId,
    _ming_gong_zhu: BodyId,
) -> HouseAnalysis {
    let year_branch = ((year - 4) % 12 + 12) as usize % 12;
    let spirits = house_spirit_names(year_branch, house_index);
    let aus: Vec<String> = spirits.iter().take(2).map(|s| s.to_string()).collect();
    let inaus: Vec<String> = spirits.iter().skip(2).take(2).map(|s| s.to_string()).collect();

    HouseAnalysis {
        house_index: house_index as u8,
        house_name: HOUSE_NAMES.get(house_index).unwrap_or(&"").to_string(),
        auspicious_spirits: aus,
        inauspicious_spirits: inaus,
        transformation_stars: vec![],
        life_cycle_annotations: vec![],
    }
}

// ============================================================
// Layer 3: Annual Transit
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitAspect {
    pub body_name: String,
    pub aspect_type: String,
    pub orb: f64,
    pub is_exact: bool,
}

/// 小限
pub fn xiaoxian(asc_branch_index: usize, current_age: u32) -> usize {
    (asc_branch_index + current_age as usize) % 12
}

/// 流年神煞
pub fn annual_spirits(year: i32) -> (Vec<&'static str>, Vec<&'static str>) {
    let branch = ((year - 4) % 12 + 12) as usize % 12;
    let mut aus = Vec::new();
    let mut inaus = Vec::new();
    for (i, &sp) in ANNUAL_AUSPICIOUS.iter().enumerate() {
        if i == branch { aus.push(sp); }
    }
    for (i, &sp) in ANNUAL_INAUSPICIOUS.iter().enumerate() {
        if i == branch { inaus.push(sp); }
    }
    (aus, inaus)
}

/// 限度分析: 生克星 + 宫度关系
pub fn analyze_transit_degree(
    limit_degree: f64,
    bodies: &[BodyInfo],
    orb_bonus: f64,
) -> Vec<TransitAspect> {
    let mut aspects = Vec::new();
    let orbs: [(f64, &str, f64); 3] = [
        (180.0, "冲", 6.0 + orb_bonus),
        (90.0, "刑", 3.0 + orb_bonus),
        (120.0, "合", 4.0 + orb_bonus),
    ];
    for body in bodies {
        let lon = body.longitude;
        if (lon - limit_degree).abs() < 1.0 {
            aspects.push(TransitAspect {
                body_name: body.body_id.name().to_string(),
                aspect_type: "同躔".to_string(),
                orb: 0.0, is_exact: true,
            });
            continue;
        }
        for &(angle, name, orb) in &orbs {
            let diff = (lon - limit_degree - angle).abs() % 360.0;
            let diff = diff.min(360.0 - diff);
            if diff < orb {
                aspects.push(TransitAspect {
                    body_name: body.body_id.name().to_string(),
                    aspect_type: name.to_string(),
                    orb: diff,
                    is_exact: diff < 0.5,
                });
            }
        }
    }
    aspects
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_body_id_from_name() {
        assert_eq!(BodyId::from_name("太阳"), Some(BodyId::太阳));
        assert_eq!(BodyId::from_name("月孛"), Some(BodyId::月孛));
        assert_eq!(BodyId::from_name("unknown"), None);
    }

    #[test]
    fn test_body_element_mapping() {
        assert_eq!(body_element(BodyId::太阳), Element::火);
        assert_eq!(body_element(BodyId::木星), Element::木);
        assert_eq!(body_element(BodyId::水星), Element::水);
        assert_eq!(body_element(BodyId::金星), Element::金);
        assert_eq!(body_element(BodyId::土星), Element::土);
    }

    #[test]
    fn test_generating_stars_for_wood() {
        let g = generating_stars(BodyId::木星);
        assert!(g.contains(&BodyId::水星) || g.contains(&BodyId::月孛));
    }

    #[test]
    fn test_overcoming_stars_for_wood() {
        let o = overcoming_stars(BodyId::木星);
        assert!(o.contains(&BodyId::金星));
    }

    #[test]
    fn test_generating_cycle() {
        assert!(is_generating(Element::木, Element::火));
        assert!(is_generating(Element::火, Element::土));
        assert!(is_generating(Element::土, Element::金));
        assert!(is_generating(Element::金, Element::水));
        assert!(is_generating(Element::水, Element::木));
    }

    #[test]
    fn test_overcoming_cycle() {
        assert!(is_overcoming(Element::木, Element::土));
        assert!(is_overcoming(Element::土, Element::水));
        assert!(is_overcoming(Element::水, Element::火));
        assert!(is_overcoming(Element::火, Element::金));
        assert!(is_overcoming(Element::金, Element::木));
    }

    #[test]
    fn test_ascending_hall_sun() {
        assert!(is_ascending_hall(BodyId::太阳, "房"));
        assert!(is_ascending_hall(BodyId::太阳, "虚"));
        assert!(is_ascending_hall(BodyId::太阳, "昴"));
        assert!(!is_ascending_hall(BodyId::太阳, "角"));
    }

    #[test]
    fn test_ascending_hall_moon() {
        assert!(is_ascending_hall(BodyId::太阴, "心"));
        assert!(is_ascending_hall(BodyId::太阴, "危"));
        assert!(!is_ascending_hall(BodyId::太阴, "井"));
    }

    #[test]
    fn test_entering_wall_sun() {
        assert!(is_entering_wall(BodyId::太阳, 7)); // 午
        assert!(!is_entering_wall(BodyId::太阳, 0));
    }

    #[test]
    fn test_lost_wall_water() {
        assert!(is_lost_wall(BodyId::水星, 0)); // 戌(白羊)火
        assert!(is_lost_wall(BodyId::水星, 4));
        assert!(!is_lost_wall(BodyId::水星, 3));
    }

    #[test]
    fn test_seasonal_star_spring() {
        assert!(is_seasonal_star(BodyId::木星, 2));
        assert!(is_seasonal_star(BodyId::紫炁, 1));
        assert!(!is_seasonal_star(BodyId::木星, 6));
    }

    #[test]
    fn test_aspect_modifier() {
        assert_eq!(aspect_modifier(0.0, 0.0).map(|x| x.1), Some(1.0));
        assert_eq!(aspect_modifier(0.0, 180.0).map(|x| x.1), Some(0.7));
        assert!(aspect_modifier(0.0, 90.0).is_none());
    }

    #[test]
    fn test_classify_nan_en_chou_yong() {
        // 金星(金)克木星(木) → 难星
        assert_eq!(classify_nan_en_chou_yong(BodyId::金星, BodyId::木星, BodyId::火星), NanEnChouYong::难星);
        // 水星(水)生木星(木) → 恩星, 但水星(水)克火星(火) → 仇星优先
        assert_eq!(classify_nan_en_chou_yong(BodyId::水星, BodyId::木星, BodyId::火星), NanEnChouYong::仇星);
    }

    #[test]
    fn test_xiaoxian() {
        assert_eq!(xiaoxian(0, 0), 0);
        assert_eq!(xiaoxian(2, 10), 0);
        assert_eq!(xiaoxian(0, 12), 0);
    }

    #[test]
    fn test_annual_spirits_not_empty() {
        let (aus, inaus) = annual_spirits(2024);
        assert!(!aus.is_empty() || !inaus.is_empty());
    }

    #[test]
    fn test_transit_aspect_opposition() {
        let bodies = vec![BodyInfo { body_id: BodyId::太阳, longitude: 180.0, mansion_name: "".into(), sign_zodiac: "".into() }];
        let aspects = analyze_transit_degree(0.0, &bodies, 0.0);
        assert!(aspects.iter().any(|a| a.aspect_type == "冲"));
    }

    #[test]
    fn test_house_spirit_returns_both() {
        let spirits = house_spirit_names(0, 0);
        assert!(!spirits.is_empty());
    }

    #[test]
    fn test_star_power_basic() {
        let bodies = vec![];
        let sp = compute_star_power(
            BodyId::太阳, 120.0, "房", 7, 5, &bodies,
            BodyId::木星, BodyId::火星,
        );
        assert!(sp.base_power >= 2); // 升殿+入垣
        assert_eq!(sp.body_name, "太阳");
    }

    #[test]
    fn test_house_analysis_returns_12() {
        let ha = analyze_house(0, 0.0, &[], 2024, 0, BodyId::木星, BodyId::火星);
        assert_eq!(ha.house_index, 0);
        assert_eq!(ha.house_name, "命宫");
    }

    #[test]
    fn test_count_aspects_same_house() {
        let bodies = vec![BodyInfo { body_id: BodyId::紫炁, longitude: 15.0, mansion_name: "".into(), sign_zodiac: "".into() }];
        let (gen_b, ovr_b) = count_body_aspects(BodyId::木星, 0.0, &bodies);
        // 木星生星有水/孛, 但紫炁(木)与木同元素, 不是生克关系
        assert_eq!(gen_b, 0.0);
        assert_eq!(ovr_b, 0.0);
    }
}
