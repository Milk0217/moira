import { memo } from "react";
import { CelestialBody, ExtraBody, ShenSha, HouseData, Aspect } from "../types/chart";
import { theme } from "../theme";
import { SectionTitle, TH, TD, MansionTD } from "./shared";

interface Props {
  bodies: CelestialBody[];
  extras: ExtraBody[];
  aspects: Aspect[];
  houses: HouseData[];
  shen_sha: ShenSha[];
}

function PlanetTable({ bodies, extras, aspects, houses, shen_sha }: Props) {
  return (
    <div style={{ width: "100%", maxWidth: 500 }}>
      <SectionTitle>七政</SectionTitle>
      <table style={tableStyle}>
        <thead>
          <tr style={{ borderBottom: `1px solid ${theme.colors.border.light}` }}>
            <TH>星曜</TH><TH>星座</TH><TH>黄经</TH><TH>黄纬</TH><TH>行度</TH><TH>宿度</TH>
          </tr>
        </thead>
        <tbody>
          {bodies.map((b) => {
            const zodiacCell = b.zodiac_sign ? `${b.zodiac_sign}${b.zodiac_degree?.toFixed(1) ?? ""}°` : "—";
            const speedVal = b.speed ?? 0;
            const isRetro = speedVal < 0;
            const speedCell = speedVal !== 0
              ? <span style={{ color: isRetro ? "#FF5252" : "#69F0AE" }}>{isRetro ? "逆" : "顺"}{Math.abs(speedVal).toFixed(2)}°/日</span>
              : "—";
            return (
              <tr key={b.name} style={{ borderBottom: `1px solid ${theme.colors.border.subtle}` }}>
                <TD>{b.name}</TD>
                <TD>{zodiacCell}</TD>
                <TD>{b.longitude.toFixed(2)}°</TD>
                <TD>{b.latitude.toFixed(2)}°</TD>
                <TD>{speedCell}</TD>
                <MansionTD>{b.mansion_name} {b.mansion_degree.toFixed(1)}°</MansionTD>
              </tr>
            );
          })}
        </tbody>
      </table>

      <SectionTitle>四馀</SectionTitle>
      <table style={tableStyle}>
        <thead>
          <tr style={{ borderBottom: `1px solid ${theme.colors.border.light}` }}>
            <TH>隐曜</TH><TH>黄经</TH><TH>宿度</TH>
          </tr>
        </thead>
        <tbody>
          {extras.map((e) => (
            <tr key={e.name} style={{ borderBottom: `1px solid ${theme.colors.border.subtle}` }}>
              <TD>{e.name}</TD>
              <TD>{e.longitude.toFixed(2)}°</TD>
              <MansionTD>{e.mansion_name} {e.mansion_degree.toFixed(1)}°</MansionTD>
            </tr>
          ))}
        </tbody>
      </table>

      {aspects.length > 0 && (
        <>
          <SectionTitle>相位</SectionTitle>
          <ul style={{ listStyle: "none", padding: 0, fontSize: theme.fontSize.lg }}>
            {aspects.map((a, i) => (
              <li key={i} style={{ padding: "4px 0", color: theme.colors.table.td }}>
                {a.point1} — {a.point2}: {a.aspect_type} ({a.angle.toFixed(1)}°, 容许度: {a.orb.toFixed(1)}°)
              </li>
            ))}
          </ul>
        </>
      )}

      <SectionTitle>十二宫</SectionTitle>
      <table style={tableStyle}>
        <thead>
          <tr style={{ borderBottom: `1px solid ${theme.colors.border.light}` }}>
            <TH>宫位</TH><TH>黄经</TH><TH>宿度</TH>
          </tr>
        </thead>
        <tbody>
          {houses.map((h) => (
            <tr key={h.index} style={{ borderBottom: `1px solid ${theme.colors.border.subtle}` }}>
              <TD>第{h.index}宫</TD>
              <TD>{h.longitude.toFixed(2)}°</TD>
              <MansionTD>{h.mansion_name}宿 {h.mansion_degree.toFixed(1)}°</MansionTD>
            </tr>
          ))}
        </tbody>
      </table>

      {shen_sha.length > 0 && (
        <>
          <SectionTitle>神煞</SectionTitle>
          <div style={{ display: "flex", flexWrap: "wrap", gap: 6, fontSize: theme.fontSize.md }}>
            {shen_sha.map((s, i) => (
              <span
                key={i}
                style={{
                  padding: "3px 8px",
                  borderRadius: theme.radius.sm,
                  background: s.quality === "吉" ? theme.colors.semantic.successBg : theme.colors.semantic.errorBg,
                  color: s.quality === "吉" ? theme.colors.semantic.success : theme.colors.semantic.error,
                  border: `1px solid ${s.quality === "吉" ? theme.colors.semantic.successBorder : theme.colors.semantic.errorBorder}`,
                }}
              >
                {s.name}
              </span>
            ))}
          </div>
        </>
      )}
    </div>
  );
}

const tableStyle: React.CSSProperties = {
  width: "100%",
  borderCollapse: "collapse",
};

export default memo(PlanetTable);
