import { CelestialBody, ExtraBody, ShenSha, HouseData, Aspect } from "../types/chart";

interface Props {
  bodies: CelestialBody[];
  extras: ExtraBody[];
  aspects: Aspect[];
  houses: HouseData[];
  shen_sha: ShenSha[];
}

export default function PlanetTable({ bodies, extras, aspects, houses, shen_sha }: Props) {
  return (
    <div style={{ width: "100%", maxWidth: 500 }}>
      <SectionTitle>七政</SectionTitle>
      <table style={tableStyle}>
        <thead>
          <tr style={{ borderBottom: "1px solid #2a2a4a" }}>
            <TH>星曜</TH><TH>黄经</TH><TH>黄纬</TH><TH>宿度</TH>
          </tr>
        </thead>
        <tbody>
          {bodies.map((b) => (
            <tr key={b.name} style={{ borderBottom: "1px solid #222" }}>
              <TD>{b.name}</TD>
              <TD>{b.longitude.toFixed(2)}°</TD>
              <TD>{b.latitude.toFixed(2)}°</TD>
              <MansionTD>{b.mansion_name} {b.mansion_degree.toFixed(1)}°</MansionTD>
            </tr>
          ))}
        </tbody>
      </table>

      <SectionTitle>四馀</SectionTitle>
      <table style={tableStyle}>
        <thead>
          <tr style={{ borderBottom: "1px solid #2a2a4a" }}>
            <TH>隐曜</TH><TH>黄经</TH><TH>宿度</TH>
          </tr>
        </thead>
        <tbody>
          {extras.map((e) => (
            <tr key={e.name} style={{ borderBottom: "1px solid #222" }}>
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
          <ul style={{ listStyle: "none", padding: 0, fontSize: 14 }}>
            {aspects.map((a, i) => (
              <li key={i} style={{ padding: "4px 0", color: "#ccc" }}>
                {a.point1} — {a.point2}: {a.aspect_type} ({a.angle.toFixed(1)}°)
              </li>
            ))}
          </ul>
        </>
      )}

      <SectionTitle>十二宫</SectionTitle>
      <table style={tableStyle}>
        <thead>
          <tr style={{ borderBottom: "1px solid #2a2a4a" }}>
            <TH>宫位</TH><TH>黄经</TH><TH>宿度</TH>
          </tr>
        </thead>
        <tbody>
          {houses.map((h) => (
            <tr key={h.index} style={{ borderBottom: "1px solid #222" }}>
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
          <div style={{ display: "flex", flexWrap: "wrap", gap: 6, fontSize: 13 }}>
            {shen_sha.map((s, i) => (
              <span
                key={i}
                style={{
                  padding: "3px 8px",
                  borderRadius: 4,
                  background: s.quality === "吉" ? "rgba(0,168,107,0.2)" : "rgba(230,57,70,0.2)",
                  color: s.quality === "吉" ? "#00a86b" : "#e63946",
                  border: `1px solid ${s.quality === "吉" ? "rgba(0,168,107,0.3)" : "rgba(230,57,70,0.3)"}`,
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

function SectionTitle({ children }: { children: React.ReactNode }) {
  return (
    <h3 style={{ color: "#c9a0dc", marginTop: 20, marginBottom: 12, fontSize: 16, fontWeight: "bold" }}>
      {children}
    </h3>
  );
}

function TH({ children }: { children: React.ReactNode }) {
  return (
    <th style={{ padding: "6px 8px", textAlign: "left", color: "#888", fontSize: 13 }}>
      {children}
    </th>
  );
}

function TD({ children }: { children: React.ReactNode }) {
  return (
    <td style={{ padding: "6px 8px", color: "#ccc", fontSize: 13 }}>
      {children}
    </td>
  );
}

function MansionTD({ children }: { children: React.ReactNode }) {
  return (
    <td style={{ padding: "6px 8px", color: "#c9a0dc", fontSize: 13 }}>
      {children}
    </td>
  );
}

const tableStyle: React.CSSProperties = {
  width: "100%",
  borderCollapse: "collapse",
};
