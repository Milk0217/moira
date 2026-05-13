import { useState } from "react";
import { Stage, Layer, Arc, Circle, Line, Rect, Text, Group } from "react-konva";

const BODY_SYMBOLS: Record<string, string> = {
  太阳: "☉", 太阴: "☽", 水星: "☿", 金星: "♀",
  火星: "♂", 木星: "♃", 土星: "♄",
  天王星: "♅", 海王星: "♆", 冥王星: "♇",
  罗睺: "☊", 计都: "☋", 月孛: "⚸", 紫炁: "✦",
};

const BODY_COLORS: Record<string, string> = {
  太阳: "#FFD700", 太阴: "#C0C0C0", 水星: "#B5A5D5", 金星: "#FF69B4",
  火星: "#FF4444", 木星: "#DAA520", 土星: "#CD853F",
  天王星: "#7ECBA1", 海王星: "#4B70DD", 冥王星: "#8B7355",
  罗睺: "#9370DB", 计都: "#8B0000", 月孛: "#2F4F4F", 紫炁: "#FFE4B5",
};

const BRANCHES = ["子", "丑", "寅", "卯", "辰", "巳", "午", "未", "申", "酉", "戌", "亥"];

const MANSIONS: { name: string; dir: string; width: number }[] = [
  { name: "角", dir: "青龙", width: 12 }, { name: "亢", dir: "青龙", width: 9 },
  { name: "氐", dir: "青龙", width: 15 }, { name: "房", dir: "青龙", width: 5 },
  { name: "心", dir: "青龙", width: 5 }, { name: "尾", dir: "青龙", width: 18 },
  { name: "箕", dir: "青龙", width: 11 }, { name: "斗", dir: "玄武", width: 26 },
  { name: "牛", dir: "玄武", width: 8 }, { name: "女", dir: "玄武", width: 12 },
  { name: "虚", dir: "玄武", width: 10 }, { name: "危", dir: "玄武", width: 17 },
  { name: "室", dir: "玄武", width: 16 }, { name: "壁", dir: "玄武", width: 9 },
  { name: "奎", dir: "白虎", width: 16 }, { name: "娄", dir: "白虎", width: 12 },
  { name: "胃", dir: "白虎", width: 14 }, { name: "昴", dir: "白虎", width: 11 },
  { name: "毕", dir: "白虎", width: 16 }, { name: "觜", dir: "白虎", width: 2 },
  { name: "参", dir: "白虎", width: 9 }, { name: "井", dir: "朱雀", width: 33 },
  { name: "鬼", dir: "朱雀", width: 4 }, { name: "柳", dir: "朱雀", width: 15 },
  { name: "星", dir: "朱雀", width: 7 }, { name: "张", dir: "朱雀", width: 18 },
  { name: "翼", dir: "朱雀", width: 18 }, { name: "轸", dir: "朱雀", width: 17 },
];

const DIR_COLORS: Record<string, string> = {
  青龙: "rgba(0,168,107,0.08)", 玄武: "rgba(54,94,184,0.08)",
  白虎: "rgba(192,192,192,0.06)", 朱雀: "rgba(230,57,70,0.08)",
};

const TOTAL_WIDTH = MANSIONS.reduce((s, m) => s + m.width, 0);

const MANSION_START_ANGLES: number[] = [];
{
  let cum = 0;
  for (const m of MANSIONS) {
    MANSION_START_ANGLES.push((cum / TOTAL_WIDTH) * 360);
    cum += m.width;
  }
}

const ASPECT_ANGLES: [number, string, string][] = [
  [0, "合相", "#FFD700"],
  [30, "半六分相", "#88CC88"],
  [60, "六分相", "#44AA44"],
  [90, "四分相", "#E63946"],
  [120, "三分相", "#4A90D9"],
  [150, "半四分相", "#888"],
  [180, "对分相", "#CC66CC"],
];

const ASPECT_ORB = 8; // 允许误差度

interface BodyEntry {
  name: string;
  longitude: number;
  detail?: string;
}

interface HouseEntry {
  index: number;
  longitude: number;
}

interface Props {
  bodies: BodyEntry[];
  houses?: HouseEntry[];
  size?: number;
  centerText?: string;
  stageRef?: React.RefObject<any>;
}

function degToVec(deg: number, radius: number, cx: number, cy: number) {
  const a = ((deg - 90) * Math.PI) / 180;
  return { x: cx + radius * Math.cos(a), y: cy + radius * Math.sin(a) };
}

/* 重叠星曜展开：将8°以内星曜分散到不同半径 */
function layoutBodies(bodies: BodyEntry[], baseR: number, cx: number, cy: number) {
  const sorted = [...bodies].sort((a, b) => a.longitude - b.longitude);
  const CLUSTER_DEG = 8;
  const RADII = [baseR, baseR * 0.88, baseR * 1.12];

  const groups: BodyEntry[][] = [];
  for (const b of sorted) {
    if (groups.length === 0) {
      groups.push([b]);
    } else {
      const last = groups[groups.length - 1];
      const lastLon = last[last.length - 1].longitude;
      if (Math.abs(b.longitude - lastLon) < CLUSTER_DEG) {
        last.push(b);
      } else {
        groups.push([b]);
      }
    }
  }

  const result: { name: string; symbol: string; color: string; detail?: string; x: number; y: number; dotR: number; hitR: number }[] = [];

  for (const g of groups) {
    for (let i = 0; i < g.length; i++) {
      const b = g[i];
      const r = g.length === 1 ? baseR : RADII[Math.min(i, RADII.length - 1)];
      const pos = degToVec(b.longitude, r, cx, cy);
      result.push({
        name: b.name,
        symbol: BODY_SYMBOLS[b.name] || b.name[0],
        color: BODY_COLORS[b.name] || "#c9a0dc",
        detail: b.detail,
        x: pos.x,
        y: pos.y,
        dotR: 7,
        hitR: 14,
      });
    }
  }

  return result;
}

export default function AstrologyChart({
  bodies,
  houses,
  size = 500,
  centerText = "命盘",
  stageRef,
}: Props) {
  const cx = size / 2;
  const cy = size / 2;
  const outerR = size / 2 - 20;
  const innerR = outerR * 0.50;
  const mansionR = outerR * 0.94;
  const branchR = outerR * 0.78;
  const planetR = outerR * 0.48;

  const [tooltip, setTooltip] = useState<{
    text: string;
    x: number;
    y: number;
  } | null>(null);
  const [selectedBody, setSelectedBody] = useState<string | null>(null);
  const [activeAngles, setActiveAngles] = useState<Set<number>>(new Set([60, 90, 120, 180]));

  const positionedBodies = layoutBodies(bodies, planetR, cx, cy);

  function toggleAngle(angle: number) {
    setActiveAngles((prev) => {
      const next = new Set(prev);
      if (next.has(angle)) next.delete(angle);
      else next.add(angle);
      return next;
    });
  }

  function getAspectLines(selected: string) {
    const sel = positionedBodies.find((b) => b.name === selected);
    if (!sel) return [];
    const lines: { x1: number; y1: number; x2: number; y2: number; color: string; label: string; angle: number }[] = [];
    for (const other of positionedBodies) {
      if (other.name === selected) continue;
      const selLon = bodies.find((b) => b.name === selected)?.longitude ?? 0;
      const otherLon = bodies.find((b) => b.name === other.name)?.longitude ?? 0;
      let raw = Math.abs(selLon - otherLon);
      if (raw > 180) raw = 360 - raw;
      for (const [target, name, color] of ASPECT_ANGLES) {
        if (Math.abs(raw - target) <= ASPECT_ORB && target !== 0 && activeAngles.has(target)) {
          lines.push({
            x1: sel.x, y1: sel.y,
            x2: other.x, y2: other.y,
            color, label: name, angle: raw,
          });
          break;
        }
      }
    }
    return lines;
  }

  const aspectLines = selectedBody ? getAspectLines(selectedBody) : [];

  return (
    <div style={{ position: "relative", display: "inline-block" }}>
      {selectedBody && (
        <div
          style={{
            position: "absolute",
            top: -30,
            left: "50%",
            transform: "translateX(-50%)",
            display: "flex",
            gap: 4,
            zIndex: 10,
            background: "rgba(15,15,35,0.9)",
            padding: "4px 10px",
            borderRadius: 8,
            border: "1px solid #333",
            whiteSpace: "nowrap",
          }}
        >
          {ASPECT_ANGLES.filter(([a]) => a !== 0).map(([angle, name, color]) => (
            <button
              key={angle}
              onClick={() => toggleAngle(angle)}
              style={{
                padding: "2px 8px",
                fontSize: 11,
                borderRadius: 4,
                border: `1px solid ${color}`,
                background: activeAngles.has(angle) ? color : "transparent",
                color: activeAngles.has(angle) ? "#fff" : color,
                cursor: "pointer",
                fontWeight: activeAngles.has(angle) ? "bold" : "normal",
              }}
            >
              {name}
            </button>
          ))}
        </div>
      )}
    <Stage ref={stageRef} width={size} height={size}>
      <Layer>
        {/* Background - click to deselect */}
        <Circle
          x={cx} y={cy} radius={outerR + 2} fill="#0f0f23"
          onClick={() => setSelectedBody(null)}
          onTap={() => setSelectedBody(null)}
        />

        {/* Outer body circle */}
        <Circle x={cx} y={cy} radius={outerR} stroke="#4a148c" strokeWidth={1.5} />
        <Circle x={cx} y={cy} radius={innerR} stroke="#4a148c" strokeWidth={1.5} />

        {/* 四象 background tint */}
        {[0, 1, 2, 3].map((q) => (
          <Arc
            key={`quad-${q}`}
            x={cx} y={cy}
            innerRadius={innerR}
            outerRadius={outerR}
            angle={90}
            rotation={q * 90 - 90}
            fill={[DIR_COLORS.青龙, DIR_COLORS.玄武, DIR_COLORS.白虎, DIR_COLORS.朱雀][q]}
            listening={false}
          />
        ))}

        {/* 二十八宿 tick marks (outer edge only) */}
        {MANSION_START_ANGLES.map((deg, i) => {
          const start = degToVec(deg, outerR * 0.86, cx, cy);
          const end = degToVec(deg, mansionR, cx, cy);
          return (
            <Line
              key={`mansion-tick-${i}`}
              points={[start.x, start.y, end.x, end.y]}
              stroke="#7b2d8e"
              strokeWidth={i % 7 === 0 ? 2 : 1}
              opacity={i % 7 === 0 ? 0.8 : 0.4}
            />
          );
        })}

        {/* 二十八宿 labels (outermost ring) */}
        {MANSIONS.map((m, i) => {
          const angle = MANSION_START_ANGLES[i] + (m.width / TOTAL_WIDTH) * 180;
          const pos = degToVec(angle, (outerR * 0.86 + mansionR) / 2, cx, cy);
          return (
            <Text
              key={`mansion-${i}`}
              x={pos.x - 7}
              y={pos.y - 6}
              text={m.name}
              fontSize={10}
              fill="#9a5db8"
              width={14}
              height={12}
              align="center"
            />
          );
        })}

        {/* 12 dividing lines */}
        {Array.from({ length: 12 }).map((_, i) => {
          const angle = (i * 30 - 90) * (Math.PI / 180);
          return (
            <Line
              key={`line-${i}`}
              points={[
                cx + innerR * Math.cos(angle),
                cy + innerR * Math.sin(angle),
                cx + outerR * Math.cos(angle),
                cy + outerR * Math.sin(angle),
              ]}
              stroke="#4a148c"
              strokeWidth={1}
              opacity={0.25}
            />
          );
        })}

        {/* Earthly branches (inner ring, clearly separated) */}
        {BRANCHES.map((branch, i) => {
          const angle = (i * 30 - 90) * (Math.PI / 180);
          const x = cx + branchR * Math.cos(angle);
          const y = cy + branchR * Math.sin(angle);
          return (
            <Text
              key={`branch-${i}`}
              x={x - 10}
              y={y - 8}
              text={branch}
              fontSize={14}
              fill="#6a1b9a"
              fontStyle="bold"
              width={20}
              height={16}
              align="center"
            />
          );
        })}

        {/* Planet orbit ring */}
        <Circle x={cx} y={cy} radius={planetR} stroke="#4a148c" strokeWidth={0.5} opacity={0.2} />

        {/* Aspect lines from selected body */}
        {aspectLines.map((line, i) => (
          <Group key={`aspect-${i}`}>
            <Line
              points={[line.x1, line.y1, line.x2, line.y2]}
              stroke={line.color}
              strokeWidth={2}
              dash={[6, 4]}
              opacity={0.6}
            />
            <Text
              x={(line.x1 + line.x2) / 2 - 20}
              y={(line.y1 + line.y2) / 2 - 10}
              text={`${line.label} ${line.angle.toFixed(0)}°`}
              fontSize={10}
              fill={line.color}
              width={60}
              height={16}
              align="center"
            />
          </Group>
        ))}

        {/* Planets with anti-overlap layout */}
        {positionedBodies.map((body) => {
          const isSelected = selectedBody === body.name;
          return (
            <Group
              key={body.name}
              onMouseEnter={() =>
                setTooltip({
                  text: body.detail || body.name,
                  x: body.x + 15,
                  y: body.y - 15,
                })
              }
              onMouseLeave={() => setTooltip(null)}
              onClick={() => setSelectedBody(isSelected ? null : body.name)}
              onTap={() => setSelectedBody(isSelected ? null : body.name)}
            >
              {/* Invisible wide hit area */}
              <Circle x={body.x} y={body.y} radius={body.hitR} fill="transparent" />
              {/* Selection glow */}
              {isSelected && (
                <Circle x={body.x} y={body.y} radius={body.dotR + 3} stroke="#FFD700" strokeWidth={2} />
              )}
              {/* Visible dot */}
              <Circle x={body.x} y={body.y} radius={body.dotR} fill={body.color} opacity={0.85} />
              {/* Symbol */}
              <Text
                x={body.x - 10}
                y={body.y - 13}
                text={body.symbol}
                fontSize={13}
                fill="#fff"
                width={20}
                height={16}
                align="center"
              />
            </Group>
          );
        })}

        {/* House number labels */}
        {houses?.map((h) => {
          const pos = degToVec(h.longitude, innerR * 0.7, cx, cy);
          return (
            <Text
              key={`house-${h.index}`}
              x={pos.x - 5}
              y={pos.y - 5}
              text={String(h.index)}
              fontSize={9}
              fill="#888"
              width={10}
              height={10}
              align="center"
            />
          );
        })}

        {/* Center circle + text */}
        <Circle x={cx} y={cy} radius={40} stroke="#4a148c" strokeWidth={1} fill="#0f0f23" />
        <Text
          x={cx - 30}
          y={cy - 10}
          text={centerText}
          fontSize={18}
          fill="#6a1b9a"
          fontStyle="bold"
          width={60}
          height={20}
          align="center"
        />

        {/* Tooltip */}
        {tooltip && (
          <Group>
            <Rect
              x={tooltip.x}
              y={tooltip.y}
              width={140}
              height={52}
              fill="rgba(0,0,0,0.85)"
              cornerRadius={4}
              stroke="#6a1b9a"
              strokeWidth={1}
            />
            <Text
              x={tooltip.x + 8}
              y={tooltip.y + 6}
              text={tooltip.text}
              fontSize={11}
              fill="#e0e0e0"
              width={124}
              height={44}
            />
          </Group>
        )}
      </Layer>
    </Stage>
    </div>
  );
}
