# P0 实现方案：核心排盘完整性

## 1. 上升/天顶 (Asc / MC)

### 算法
- **上升 (Asc)**: 使用本地恒星时 (LST) 计算东升点的黄道经度
  - 公式: `Asc = atan2(cos(LST) , -(sin(Obliquity) * tan(Latitude) + cos(Obliquity) * sin(LST)))` + 适当象限调整
- **天顶 (MC)**: 中天点 = atan2(tan(LST), cos(Obliquity))
- **依赖**: `local_sidereal_time()`（已有）+ 黄赤交角（已有）

### 产出
- `angles.rs`: `calculate_ascendant()`, `calculate_mc()`, `calculate_angles()`
- 前端：InputForm 已含经纬度，星盘图中显示 Asc/MC 标记

## 2. 福点 (Part of Fortune)

### 算法
- 日生（太阳在 Asc-Dsc 之南，即上升至下降的黄道南半）：`Fortuna = Asc + Moon - Sun`
- 夜生（太阳在 Asc-Dsc 之北）：`Fortuna = Asc + Sun - Moon`
- 所有角度归一化到 0-360°

### 产出
- `angles.rs`: `calculate_part_of_fortune()`
- 前端：五行星表格增加福点一行

## 3. 四柱八字

### 算法
- **年柱**: 出生年 → 天干地支。天干 = (年 - 4) % 10，地支 = (年 - 4) % 12
- **月柱**: 年干决定月干起始，月支固定（寅=1）
- **日柱**: 公历连续日数 → 日干支
- **时柱**: 日干决定时干起始，时支由出生时辰决定

### 依赖
- **节气分割**: 年柱以立春为界，月柱以节气为界
- 需农历节气数据（节气的公历日期）

### 简化方案（第一期）
- 不使用节气分割，取简单近似（每月初一为月柱分界）
- 节气精确分割作为后续优化

### 产出
- `bazi.rs`: `calculate_year_pillar()`, `calculate_month_pillar()`, `calculate_day_pillar()`, `calculate_hour_pillar()`, `calculate_bazi()`
- 前端：BaziPanel 组件（四柱八字卡片）

## 4. 大运 + 十神

### 算法
- **大运**: 从月柱起，根据年干阴阳决定顺逆排运
  - 阳年男/阴年女：顺行（月柱下一个干支）
  - 阴年男/阳年女：逆行（月柱上一个干支）
  - 起运年龄：从出生日到下一个/上一个节气的时间 ÷ 3 天/年
- **十神**: 日干与其他干支的生克关系
  - 正官(克我阴阳异)、七杀(克我阴阳同)
  - 正印(生我阴阳异)、偏印(生我阴阳同)
  - 比肩(同我阴阳同)、劫财(同我阴阳异)
  - 食神(我生阴阳同)、伤官(我生阴阳异)
  - 正财(我克阴阳异)、偏财(我克阴阳同)

### 简化方案（第一期）
- 大运不分顺逆，默认顺排（后续完善）
- 起运年龄从出生到下一个节气计算
- 十神完整实现

### 产出
- `bazi.rs`: `calculate_dayun()`, `calculate_shishen()`
- 前端：DayunPanel 组件

## 5. 长生十二宫 + 胎元 + 藏干

### 算法
- **长生十二宫**: 从年干看地支的状态
  - 顺序: 长生 → 沐浴 → 冠带 → 临官 → 帝旺 → 衰 → 病 → 死 → 墓 → 绝 → 胎 → 养
  - 不同天干对应不同的起始宫（五行不同）
- **胎元**: 月柱天干+1, 月柱地支+3（顺数三位）
- **藏干**: 地支的主气、中气、余气
  - 例如: 子藏癸，丑藏己癸辛，寅藏甲丙戊...

### 产出
- `bazi.rs` 或 `zodiac_signs.rs` 新增函数
- 前端：LifeCyclePanel 组件

## 新增模块结构

```
moira-rs/crates/moira-core/src/
├── lib.rs               # 导出 angles, bazi 模块
├── angles.rs            # 新增: asc, mc, fortune
├── bazi.rs              # 新增: 八字 + 大运 + 十神 + 藏干 + 长生 + 胎元
```

## 前端新增

```
apps/desktop/src/components/
├── BaziPanel.tsx        # 四柱八字 + 大运 + 十神
├── LifeCyclePanel.tsx   # 长生十二宫 + 胎元 + 藏干
```

### API 响应格式变更

`AstrologyData` 结构体新增字段:
```rust
pub struct AstrologyData {
    // ... 现有字段
    pub ascendant: f64,
    pub midheaven: f64,
    pub part_of_fortune: f64,
    pub bazi: BaziData,
}

pub struct BaziData {
    pub year_pillar: Pillar,
    pub month_pillar: Pillar,
    pub day_pillar: Pillar,
    pub hour_pillar: Pillar,
    pub dayun: Vec<DayunPillar>,
    pub shishen: Vec<ShishenMap>,
    pub hidden_stems: Vec<HiddenStem>,
    pub life_cycle: Vec<LifeCycleItem>,
    pub taiyuan: Pillar,
}
```
