# Moira Rule Engine — Programming Specification

> Extracted and parsed from `sample_s.rule` (UTF-16LE) and `sample2_s.rule` (UTF-16BE) of the original Moira v1.50 Java application.
> These files define the complete 七政四馀 traditional Chinese astrology evaluation engine.

---

## 1. Architecture Overview

The rule engine is a **declarative evaluation system** with three layers:

```
┌─────────────────────────────────────────┐
│  Layer 3: 流年盘 (Annual Transit)        │  ← Dynamic yearly reset
│  - 小限 → 限宫 → 限度 → 同/冲/刑/合     │
├─────────────────────────────────────────┤
│  Layer 2: 十二宫分析 (12 Houses)         │
│  - 命/财/兄/田/男/仆/妻/疾/迁/官/福/相   │
│  - 宫神煞 + 化星照                      │
├─────────────────────────────────────────┤
│  Layer 1: 命盘基础 (Chart Foundation)     │
│  - 立命宫/安身宫                          │
│  - 星力系统 (升殿/失躔/入垣/失垣/季星)    │
│  - 难恩仇用星                             │
│  - 生克星同宫/同度/对照/拱照              │
└─────────────────────────────────────────┘
```

---

## 2. 命盘基础 — Chart Foundation (Layer 1)

### 2.1 难恩仇用星

Each of the 11 bodies (|七政| + |四余|) is classified as one of:

| Type | Character | Meaning |
|------|-----------|---------|
| 难星 | 难 | Difficult star (克命度主) |
| 仇星 | 仇 | Enemy star (克命宫主) |
| 恩星 | 恩 | Benevolent star (生命度主) |
| 用星 | 用 | Useful star (生命宫主) |

**Implementation**: Based on the relationship between each star's element and the 命度主/命宫主 elements.
- 生 = generating (element produces target)
- 克 = overcoming (element overcomes target)

### 2.2 星力系统 (Star Power Score)

Each of the 11 bodies gets a **power score** that compounds through the chart:

```
Power = 0
  + (if 升殿)        +1    // Planet is in own mansion degree
  + (if 入垣)        +1    // Planet is in own zodiac sign
  + (if 季星)        +1    // Planet is seasonal star
  + (if 生星同度)    +N    // N = count of generating stars in same degree
  + (if 生星同宫)    +N    // N = count of generating stars in same house
  + (if 生星对照,    ×0.7) // Generating stars in opposite house (180°)
  + (if 生星拱照,    ×0.3) // Generating stars in trine (120°)
  - (if 失躔)        -1    // Planet is in hostile mansion degree
  - (if 失垣)        -1    // Planet is in hostile zodiac sign
  - (if 克星同度)    -N    // Overcoming stars in same degree
  - (if 克星同宫)    -N    // Overcoming stars in same house
  - (if 克星对照,    ×0.7) // Overcoming stars opposite
  - (if 克星拱照,    ×0.3) // Overcoming stars in trine
```

#### 2.2.1 Definitions

**升殿 (Ascending Hall)**: Planet is in its preferred mansion degree position. Each planet has specific mansion-degree relationships where it is considered "升殿".

**失躔 (Lost Tracking)**: Planet is in a mansion degree that conflicts with its nature (克度).

**入垣 (Entering Wall)**: Planet is in its own zodiac sign (e.g., 火星 in 白羊). Defined by the planet-sign mapping.

**失垣 (Lost Wall)**: Planet is in a hostile zodiac sign (sign that克 the planet).

**季星 (Seasonal Star)**: Each season has a ruling star:
| Season | Stars |
|--------|-------|
| Spring (春) | 木 + 炁 |
| Summer (夏) | 火 + 罗 |
| Autumn (秋) | 金 |
| Winter (冬) | 水 + 孛 |

#### 2.2.2 生克星 (Generating/Overcoming Stars)

Each of the 11 bodies has a set of **生星** (stars that generate it) and **克星** (stars that overcome it), based on the Five Elements (五行) cycle:

```
木生火, 火生土, 土生金, 金生水, 水生木  (Generating)
木克土, 土克水, 水克火, 火克金, 金克木  (Overcoming)
```

**Element mapping for the 11 bodies**:

| Body | Element | 生星 (Generators) | 克星 (Overcomers) |
|------|---------|-------------------|-------------------|
| 太阳/火星/冥王星/罗睺 | 火 | 木,炁 | 水,孛 |
| 太阴/水星/海王星/月孛 | 水 | 金 | 土,计 |
| 金星/天王星 | 金 | 土,计 | 火,罗 |
| 木星/紫炁 | 木 | 水,孛 | 金 |
| 土星/计都 | 土 | 火,罗 | 木,炁 |

#### 2.2.3 Aspect Modifiers

| Aspect | Angle | Modifier | Rule syntax |
|--------|-------|----------|-------------|
| 同宫 (Same house) | 0° (same 30° house) | ×1.0 | `同宫` |
| 同度 (Same degree) | 0° (within same mansion degree) | ×1.0 | `同度` |
| 对照 (Opposition) | 180° | ×0.7 | `对照` |
| 拱照 (Trine) | 120° | ×0.3 | `拱照` |

---

## 3. 十二宫分析 — House Analysis (Layer 2)

### 3.1 Houses and Chinese Names

| Index | Name | Domain |
|-------|------|--------|
| 1 | 命宫 | Life/self |
| 2 | 财帛 | Wealth |
| 3 | 兄弟 | Siblings |
| 4 | 田宅 | Property |
| 5 | 男女 | Children |
| 6 | 奴仆 | Servants/health |
| 7 | 夫妻 | Spouse |
| 8 | 疾厄 | Illness |
| 9 | 迁移 | Travel |
| 10 | 官禄 | Career |
| 11 | 福德 | Fortune |
| 12 | 相貌 | Appearance |

### 3.2 宫神煞 (House Spirit Stars)

Two sets of spirits check each house for presence:

**吉神 (16 auspicious)**:
```
红鸾, 禄勋, 唐符, 咸池, 天贵, 斗杓, 喜神,
文昌, 卦气, 玉贵, 天喜, 国印, 驿马, 紫微, 天德
```

**凶煞 (20 inauspicious)**:
```
病符, 寡宿, 劫杀, 绞杀, 飞刃, 天狗, 阳刃, 阴刃,
天耗, 剑锋, 孤辰, 孤虚, 飞廉, 天雄, 大耗, 小耗,
阑干, 血刃, 空亡, 的杀
```

**Implementation**: Each house has a set of 宫神煞 (house spirit stars). For a given house, compute:
- `$宫神 = intersection(${house_神煞}, $吉神)` → pick top 4
- `$宫煞 = intersection(${house_神煞}, $凶煞)` → pick top 4

### 3.3 化星照 (Transformation Star Shining)

For each house, check which of the 11 bodies fall into it. For each body present:
- Apply 难恩 classification
- Apply 化星 (transformation based on 十干化曜)

### 3.4 长生注 (Life Cycle Annotations)

When a house contains specific Life Cycle stages (长生, 绝, 死, 病, 帝旺, 临官, 沐浴), it gets annotated:
```
命中带长生地, 绝地, 死地, 病地, 帝旺地, 临官地, 沐浴地
```

---

## 4. 流年盘 — Annual Transit (Layer 3)

### 4.1 Structure

```
本年: {流年}年{流岁}岁
流年柱: {流年柱} (year pillar of current year)
小限: {小限} (annual limit palace)
限宫: {限宫} (current transit palace)
限度: {限度} (current transit degree)
```

### 4.2 小限 (Annual Limit)

Calculated from 命宫 (Ascendant branch) + current age:
```
小限branch = (asc_branch + current_age) % 12
限宫 = 小限branch 对应的宫位
```

### 4.3 限度分析 (Degree Analysis)

For each degree in the current transit, analyze aspects from all 11 bodies:

| Aspect | Angle | Orb | Formula |
|--------|-------|-----|---------|
| 同躔 (Conjunction) | 0° | exact | `body == 限degree` |
| 冲 (Opposition) | 180° | 6° + 角距 | `\|body - 限\| ≈ 180° in [174-角距, 186+角距]` |
| 刑 (Square) | 90° | 3° + 角距 | `\|body - 限\| ≈ 90° in [87-角距, 93+角距]` |
| 合 (Trine) | 120° | 4° + 角距 | `\|body - 限\| ≈ 120° in [116-角距, 124+角距]` |

Where `角距` (orb bonus) is a configurable parameter.

### 4.4 流年神煞 (Annual Transit Spirits)

**流吉神 (10 auspicious)**:
```
天喜, 红鸾, 驿马, 天厨, 咸池, 禄勋, 天德, 天贵, 地解, 解神
```

**流凶煞 (25 inauspicious)**:
```
岁驾, 天空, 地雌, 贯索, 五鬼, 死符, 大耗, 小耗, 阳刃, 阴刃,
天厄, 天雄, 大杀, 卷舌, 天狗, 蓦越, 亡神, 披头, 血刃, 天哭,
劫杀, 的杀, 黄幡, 豹尾, 三刑, 六害
```

---

## 5. 星曜诗赋数据库 — Star Poetry Database

### 5.1 Data Structure

From `sample_s.rule`, three types of rules:

```
Type 1: 星曜躔度歌 (Mansion Poetry)
  {%planet[0]=mansion}:[rank]  verse
  Example: {%日[0]=井}:[2.11.3]  日躔井宿号天弼...

Type 2: 星曜入宫歌 (Zodiac Poetry)  
  {@planet[0]=sign}:[rank]  verse
  Example: {@日[0]=丑}:[2.11.2]  太阳居丑号天幽...

Type 3: 星曜照宫歌 (House Poetry)
  {@planet=(at)house}:[rank]  verse
  Example: {@日=(at)命宫}:[2.11.1]  太阳临命性融和...
```

### 5.2 Rank Format `[X.Y.Z]`

| Component | Meaning | Values |
|-----------|---------|--------|
| X | Unknown grouping | 1-2 |
| Y | Planet ID | See table below |
| Z | Priority (higher = first) | 3=mansion, 2=sign, 1=house |

**Planet ID mapping**:
```
1=月孛, 2=紫炁, 3=罗睺, 4=计都, 5=土星
6=火星, 7=水星, 8=木星, 9=金星, 10=太阴, 11=太阳
```

### 5.3 Database Implementation

Create a static lookup table for all 3 types:

**躔度歌 (Mansion)**: 11 planets × 28 mansions = 308 entries
```
Key: (planet_name, mansion_name) → {rank, verse, title}
```

**入宫歌 (Sign)**: 11 planets × 12 signs = 132 entries
```
Key: (planet_name, sign_name) → {rank, verse, title}  
```

**照宫歌 (House)**: 11 planets × 12 houses = 132 entries
```
Key: (planet_name, house_name) → {rank, verse, title}
```

---

## 6. 化曜系统 — Transformation System

### 6.1 十干化曜 (Ten Stems Transformation)

Based on the Day Stem (日干), each of the 5 elements cycles through:

| Day Stem Pair | Transformed Element |
|---------------|-------------------|
| 甲 + 己 | 土 |
| 乙 + 庚 | 金 |
| 丙 + 辛 | 水 |
| 丁 + 壬 | 木 |
| 戊 + 癸 | 火 |

### 6.2 化星 (Transformed Star)

Each body can be "transformed" by the Day Stem's element. The transformation status affects the body's power score through `&化星`.

---

## 7. 命身安星 — Life & Body Setting

### 7.1 命宫 (Life Palace)

命宫 is determined by the Ascendant (上升点) of the birth chart. In Equal House system:
```
命宫 = House 1 (Ascendant所在的宫)
命度 = Specific degree of Ascendant within the mansion
```

### 7.2 身宫 (Body Palace)

身宫 is determined by the birth hour:
```
身宫 = House determined by hour branch
```

### 7.3 命身歧 (Life-Body Divergence)

When 命宫 and 身宫 fall in different houses or different mansion degrees:
```
命坐宫歧 = 命宫 and 命度 are in different houses
命坐宿歧 = 命度主 is different from 命宫主
身坐宫歧 = Same for 身宫
```

---

## 8. Implementation Roadmap

### Phase A: Star Power Engine (紧迫)
1. Implement `StarPowerSystem` struct → compute power for each body
2. Implement 升殿/失躔/入垣/失垣 detection tables
3. Implement 生克星 interaction matrix (11×11)
4. Implement aspect modifiers (同宫/同度/对照/拱照)

### Phase B: House Analysis (中)  
5. Implement 宫神煞 per house (spirit/ghost detection)
6. Implement 化星照 per house
7. Implement 长生注 detection

### Phase C: Annual Transit (低)
8. Implement 小限/限宫/限度 calculation
9. Implement 同/冲/刑/合 aspects on limits
10. Implement 流年神煞 detection

### Phase D: Poetry Database (参考)
11. Import all 308 躔度歌 entries as static data
12. Import all 132 入宫歌 entries
13. Import all 132 照宫歌 entries
14. Render matching verses in UI based on planet positions

---

## 9. Data Tables for Implementation

### 9.1 入垣 (Planet-Sign Mapping)

Each planet is strongest in its own sign(s):

| Planet | Own Signs | 升殿 (Mansion) |
|--------|-----------|---------------|
| 太阳 | 狮子(午) | 房/虚/昴/星 |
| 太阴 | 巨蟹(未) | 心/危/毕/张 |
| 水星 | 双子(申), 处女(巳) | 箕/壁/参/轸 |
| 金星 | 金牛(酉), 天秤(辰) | 角/牛/奎/娄 |
| 火星 | 白羊(戌), 天蝎(卯) | 尾/室/觜/翼 |
| 木星 | 射手(寅), 双鱼(亥) | 斗/奎/井/角 |
| 土星 | 摩羯(丑), 水瓶(子) | 女/危/毕/星 |
| 罗睺 | (no own sign) | 在亥为入垣 |
| 计都 | (no own sign) | 在巳为入垣 |
| 紫炁 | (no own sign) | 在午为入垣 |
| 月孛 | (no own sign) | 在未为入垣 |

### 9.2 季节星 (Seasonal Stars)

```rust
const SEASONAL_STARS: [[&str; 2]; 4] = [
    ["木", "炁"],  // 春(寅卯辰)
    ["火", "罗"],  // 夏(巳午未) 
    ["金", "金"],  // 秋(申酉戌)
    ["水", "孛"],  // 冬(亥子丑)
];

fn seasonal_star(body_name: &str, month: u8) -> bool {
    let season = match month {
        1..=3 => 0,  // 春
        4..=6 => 1,  // 夏
        7..=9 => 2,  // 秋
        _ => 3,      // 冬
    };
    SEASONAL_STARS[season].contains(&body_name)
}
```

### 9.3 Orbit/Aspect Tolerance

| Aspect | Angle | Base Orb | Configurable Add |
|--------|-------|----------|-----------------|
| 冲 (Opposition) | 180° | 6° | +$限度角距 |
| 刑 (Square) | 90° | 3° | +$限度角距 |
| 合 (Trine) | 120° | 4° | +$限度角距 |

### 9.4 Gender-Based Calculation

```
阳男阴女 (Yang Male, Yin Female): 大运/洞微飞限顺排
阴男阳女 (Yin Male, Yang Female): 大运/洞微飞限逆排

阳干年: 甲/丙/戊/庚/壬 (stem_index % 2 == 0)
阴干年: 乙/丁/己/辛/癸 (stem_index % 2 == 1)
```

---

## 10. Original Rule File Syntax Reference

| Symbol | Meaning | Example |
|--------|---------|---------|
| `{$var}` | Variable substitution | `{$流年}` → current year |
| `{%var}` | Mansion-degree variable | `{%命[0]}` → life mansion degree |
| `{@var}` | House-level variable | `{@命[0]}` → life palace |
| `(=)` | Conditional check | `{?[a]殿}` → if star is 升殿 |
| `(&fn)` | Function call | `{&星力([])}` → compute star power |
| `$` | Rule terminator | Separates rules |
| `{?t}:` | Template header | Starts a template section |
| `[X.Y.Z]` | Priority/rank | Higher Z → displayed first |
| `#` | Comment | Ignored by engine |
| `+Rule1:` | Named rule (debuggable) | Can be traced |

---

## Appendix: File Origin

- `sample_s.rule` (650 lines, UTF-16LE) — Star poetry database (躔度/入宫/照宫歌)
- `sample2_s.rule` (~650 lines, UTF-16BE) — Rule engine logic (星力/生克/宫神煞/流年盘)
- Original Moira version: 1.50
- Compiled with: GraalVM SubstrateVM native-image
- Rule format: Custom DSL parser in the Java application
