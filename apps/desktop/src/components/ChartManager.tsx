import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import type { ChartData } from "../types/chart";

interface Props {
  onLoad: (data: ChartData) => void;
  currentData: ChartData | null;
}

export default function ChartManager({ onLoad, currentData }: Props) {
  const [savedCharts, setSavedCharts] = useState<string[]>([]);
  const [saveName, setSaveName] = useState("");
  const [showManager, setShowManager] = useState(false);

  useEffect(() => { if (showManager) refreshList(); }, [showManager]);

  async function refreshList() {
    const list = await invoke<string[]>("list_charts");
    setSavedCharts(list);
  }

  async function handleSave() {
    if (!currentData || !saveName.trim()) return;
    await invoke("save_chart", { name: saveName.trim(), data: currentData });
    setSaveName("");
    refreshList();
  }

  async function handleLoad(name: string) {
    const data = await invoke<ChartData>("load_chart", { name });
    onLoad(data);
    setShowManager(false);
  }

  async function handleDelete(name: string) {
    await invoke("delete_chart", { name });
    refreshList();
  }

  return (
    <div>
      <button onClick={() => setShowManager(!showManager)} style={toggleBtnStyle}>
        {showManager ? "关闭资料管理" : "资料管理"}
      </button>
      {showManager && (
        <div style={{ marginTop: 12, padding: 12, background: "#1a1a2e", borderRadius: 8, border: "1px solid #333" }}>
          <h4 style={{ color: "#c9a0dc", fontSize: 14, marginBottom: 8 }}>保存当前星盘</h4>
          <div style={{ display: "flex", gap: 8, marginBottom: 16 }}>
            <input
              value={saveName}
              onChange={(e) => setSaveName(e.target.value)}
              placeholder="输入星盘名称..."
              style={inputStyle}
            />
            <button onClick={handleSave} disabled={!saveName.trim() || !currentData} style={actionBtnStyle}>
              保存
            </button>
          </div>
          <h4 style={{ color: "#c9a0dc", fontSize: 14, marginBottom: 8 }}>已保存的星盘</h4>
          {savedCharts.length === 0 ? (
            <p style={{ color: "#666", fontSize: 13 }}>暂无保存的星盘</p>
          ) : (
            <ul style={{ listStyle: "none", padding: 0 }}>
              {savedCharts.map((name) => (
                <li key={name} style={{ display: "flex", justifyContent: "space-between", padding: "6px 0", borderBottom: "1px solid #222" }}>
                  <span style={{ color: "#e0e0e0", fontSize: 13 }}>{name}</span>
                  <div style={{ display: "flex", gap: 6 }}>
                    <button onClick={() => handleLoad(name)} style={smallBtnStyle}>加载</button>
                    <button onClick={() => handleDelete(name)} style={{ ...smallBtnStyle, color: "#e63946", borderColor: "#e63946" }}>删除</button>
                  </div>
                </li>
              ))}
            </ul>
          )}
        </div>
      )}
    </div>
  );
}

const toggleBtnStyle: React.CSSProperties = {
  width: "100%", padding: "6px 0", background: "transparent", color: "#c9a0dc",
  border: "1px dashed #c9a0dc", borderRadius: 6, cursor: "pointer", fontSize: 13, marginTop: 12,
};

const inputStyle: React.CSSProperties = {
  flex: 1, padding: "6px 10px", background: "#0f0f23", color: "#e0e0e0",
  border: "1px solid #333", borderRadius: 4, fontSize: 13, outline: "none",
};

const actionBtnStyle: React.CSSProperties = {
  padding: "6px 14px", background: "#3a1a5e", color: "#c9a0dc",
  border: "1px solid #c9a0dc", borderRadius: 4, cursor: "pointer", fontSize: 12,
};

const smallBtnStyle: React.CSSProperties = {
  padding: "2px 10px", background: "transparent", color: "#c9a0dc",
  border: "1px solid #c9a0dc", borderRadius: 4, cursor: "pointer", fontSize: 11,
};
