import { useState, useCallback } from "react";
import { invoke } from "@tauri-apps/api/core";
import { ChartData, BirthInfo } from "./types/chart";
import InputForm from "./components/InputForm";
import AstrologyChart from "./components/AstrologyChart";
import PlanetTable from "./components/PlanetTable";
import BaziPanel from "./components/BaziPanel";
import ErrorBoundary from "./components/ErrorBoundary";

function App() {
  const [chartData, setChartData] = useState<ChartData | null>(null);
  const [loading, setLoading] = useState(false);
  const [birthInfo, setBirthInfo] = useState<BirthInfo | null>(null);

  const handleCalculate = useCallback(async (info: BirthInfo) => {
    setLoading(true);
    setBirthInfo(info);
    try {
      const data = await invoke<ChartData>("calculate", {
        year: info.year,
        month: info.month,
        day: info.day,
        hour: info.hour,
        minute: info.minute,
        second: info.second,
        timezone: info.timezone,
        latitude: info.latitude,
        longitude: info.longitude,
      });
      setChartData(data);
    } catch (e) {
      console.error("计算失败:", e);
      alert(`计算失败: ${e}`);
    } finally {
      setLoading(false);
    }
  }, []);

  const chartBodies = chartData
    ? [
        ...chartData.bodies.map((b) => ({
          name: b.name,
          longitude: b.longitude,
          detail: `${b.name}\n黄经 ${b.longitude.toFixed(1)}° 黄纬 ${b.latitude.toFixed(1)}°\n${b.mansion_name}宿 ${b.mansion_degree.toFixed(1)}°`,
        })),
        ...chartData.extra_bodies.map((e) => ({
          name: e.name,
          longitude: e.longitude,
          detail: `${e.name}\n黄经 ${e.longitude.toFixed(1)}°\n${e.mansion_name}宿 ${e.mansion_degree.toFixed(1)}°`,
        })),
      ]
    : [];

  async function handleExport() {
    if (!chartData) return;
    const stage = document.querySelector("canvas")?.parentElement;
    if (!stage) return;
    const uri = stage.querySelector("canvas")?.toDataURL("image/png");
    if (!uri) return;
    const a = document.createElement("a");
    a.href = uri;
    a.download = `moira-${birthInfo?.year ?? "chart"}.png`;
    a.click();
  }

  return (
    <div style={{ display: "flex", height: "100vh", width: "100vw" }}>
      <div
        style={{
          width: 320,
          minWidth: 320,
          borderRight: "1px solid #2a2a4a",
          padding: 24,
          overflowY: "auto",
          background: "#16213e",
        }}
      >
        <h1
          style={{
            fontSize: 22,
            fontWeight: "bold",
            marginBottom: 24,
            color: "#c9a0dc",
            letterSpacing: 2,
          }}
        >
          Moira 星盘
        </h1>
        <InputForm onCalculate={handleCalculate} loading={loading} />
      </div>

      <div
        style={{
          flex: 1,
          display: "flex",
          flexDirection: "column",
          alignItems: "center",
          justifyContent: "center",
          padding: 24,
          overflow: "auto",
          background: "#1a1a2e",
        }}
      >
        {chartData ? (
          <div
            style={{
              display: "flex",
              flexDirection: "column",
              alignItems: "center",
              gap: 32,
            }}
          >
            {birthInfo && (
              <div style={{ fontSize: 13, color: "#888", textAlign: "center" }}>
                {birthInfo.year}-{String(birthInfo.month).padStart(2, "0")}-
                {String(birthInfo.day).padStart(2, "0")}{" "}
                {String(birthInfo.hour).padStart(2, "0")}:
                {String(birthInfo.minute).padStart(2, "0")} UTC{birthInfo.timezone >= 0 ? "+" : ""}
                {birthInfo.timezone}
              </div>
            )}
            <ErrorBoundary
              fallback={
                <div style={{ color: "#e63946", padding: 40 }}>星盘渲染失败</div>
              }
            >
              <AstrologyChart bodies={chartBodies} houses={chartData.houses} size={500} />
            </ErrorBoundary>
            <div style={{ display: "flex", gap: 8 }}>
              <button onClick={handleExport} style={btnStyle}>
                导出 PNG
              </button>
            </div>
            <div style={{ display: "flex", gap: 8, alignItems: "center" }}>
              <span style={{ color: "#888", fontSize: 13 }}>星制:</span>
              <button
                onClick={() => {}}
                style={{
                  padding: "4px 12px",
                  fontSize: 12,
                  borderRadius: 4,
                  border: `1px solid ${chartData.zodiac_type === "回归" ? "#c9a0dc" : "#555"}`,
                  background: chartData.zodiac_type === "回归" ? "#3a1a5e" : "transparent",
                  color: chartData.zodiac_type === "回归" ? "#c9a0dc" : "#888",
                  cursor: "pointer",
                }}
              >
                回归制 {chartData.zodiac_type === "回归" ? "✓" : ""}
              </button>
              <span style={{ color: "#555", fontSize: 11 }}>
                岁差: {chartData.ayanamsa.toFixed(2)}°
              </span>
            </div>
            <PlanetTable
              bodies={chartData.bodies}
              extras={chartData.extra_bodies}
              aspects={chartData.aspects}
              houses={chartData.houses}
              shen_sha={chartData.shen_sha}
            />
            <BaziPanel
              bazi={chartData.bazi}
              ascendant={chartData.ascendant}
              midheaven={chartData.midheaven}
              partOfFortune={chartData.part_of_fortune}
              mingZhu={chartData.ming_zhu}
              shenZhu={chartData.shen_zhu}
              shiganhuayao={chartData.shiganhuayao}
              xijige={chartData.xijige}
              xiaoxianResult={chartData.xiaoxian_result}
              yuexianResult={chartData.yuexian_result}
              dongweifeixianResult={chartData.dongweifeixian_result}
            />
          </div>
        ) : (
          <div style={{ color: "#666", fontSize: 16, textAlign: "center" }}>
            <p style={{ marginBottom: 12, fontSize: 48 }}>✦</p>
            <p>输入出生信息，点击"计算星盘"</p>
          </div>
        )}
      </div>
    </div>
  );
}

const btnStyle: React.CSSProperties = {
  padding: "6px 16px",
  background: "transparent",
  color: "#c9a0dc",
  border: "1px solid #c9a0dc",
  borderRadius: 6,
  cursor: "pointer",
  fontSize: 13,
};

export default App;
