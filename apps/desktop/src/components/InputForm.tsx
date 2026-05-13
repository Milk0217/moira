import { useState, FormEvent } from "react";
import { BirthInfo } from "../types/chart";

interface Props {
  onCalculate: (info: BirthInfo) => void;
  loading: boolean;
}

export default function InputForm({ onCalculate, loading }: Props) {
  const [year, setYear] = useState(1990);
  const [month, setMonth] = useState(1);
  const [day, setDay] = useState(1);
  const [hour, setHour] = useState(12);
  const [minute, setMinute] = useState(0);
  const [second, setSecond] = useState(0);
  const [timezone, setTimezone] = useState(8);
  const [latitude, setLatitude] = useState(39.9);
  const [longitude, setLongitude] = useState(116.4);
  const [dst, setDst] = useState(false);

  function handleSubmit(e: FormEvent) {
    e.preventDefault();
    onCalculate({ year, month, day, hour, minute, second, timezone, latitude, longitude, dst_applied: dst });
  }

  return (
    <form onSubmit={handleSubmit}>
      <Field label="出生年">
        <input
          type="number"
          value={year}
          onChange={(e) => setYear(Number(e.target.value))}
          style={inputStyle}
        />
      </Field>

      <div style={{ display: "flex", gap: 8 }}>
        <Field label="月">
          <input
            type="number"
            value={month}
            min={1}
            max={12}
            onChange={(e) => setMonth(Number(e.target.value))}
            style={{ ...inputStyle, width: "100%" }}
          />
        </Field>
        <Field label="日">
          <input
            type="number"
            value={day}
            min={1}
            max={31}
            onChange={(e) => setDay(Number(e.target.value))}
            style={{ ...inputStyle, width: "100%" }}
          />
        </Field>
      </div>

      <div style={{ display: "flex", gap: 8 }}>
        <Field label="时">
          <input
            type="number"
            value={hour}
            min={0}
            max={23}
            onChange={(e) => setHour(Number(e.target.value))}
            style={{ ...inputStyle, width: "100%" }}
          />
        </Field>
        <Field label="分">
          <input
            type="number"
            value={minute}
            min={0}
            max={59}
            onChange={(e) => setMinute(Number(e.target.value))}
            style={{ ...inputStyle, width: "100%" }}
          />
        </Field>
        <Field label="秒">
          <input
            type="number"
            value={second}
            min={0}
            max={59}
            onChange={(e) => setSecond(Number(e.target.value))}
            style={{ ...inputStyle, width: "100%" }}
          />
        </Field>
      </div>

      <div style={{ display: "flex", gap: 8 }}>
        <Field label="纬度 (北+ / 南-)">
          <input
            type="number"
            value={latitude}
            step={0.1}
            min={-90}
            max={90}
            onChange={(e) => setLatitude(Number(e.target.value))}
            style={{ ...inputStyle, width: "100%" }}
          />
        </Field>
        <Field label="经度 (东+ / 西-)">
          <input
            type="number"
            value={longitude}
            step={0.1}
            min={-180}
            max={180}
            onChange={(e) => setLongitude(Number(e.target.value))}
            style={{ ...inputStyle, width: "100%" }}
          />
        </Field>
      </div>

      <Field label="时区 (东+ / 西-)">
        <input
          type="number"
          value={timezone}
          step={0.5}
          min={-12}
          max={14}
          onChange={(e) => setTimezone(Number(e.target.value))}
          style={inputStyle}
        />
      </Field>

      <div style={{ marginBottom: 12 }}>
        <button
          type="button"
          onClick={() => setDst(!dst)}
          style={{
            width: "100%",
            padding: "8px 10px",
            background: dst ? "#3a1a5e" : "#1a1a2e",
            color: dst ? "#c9a0dc" : "#888",
            border: `1px solid ${dst ? "#c9a0dc" : "#333"}`,
            borderRadius: 6,
            fontSize: 14,
            cursor: "pointer",
            textAlign: "center",
          }}
        >
          夏令时间: {dst ? "ON (UTC+1)" : "OFF"}
        </button>
      </div>

      <button
        type="submit"
        disabled={loading}
        style={{
          width: "100%",
          marginTop: 16,
          padding: "10px 0",
          background: loading ? "#555" : "#6a1b9a",
          color: "#fff",
          border: "none",
          borderRadius: 8,
          fontSize: 15,
          fontWeight: "bold",
          cursor: loading ? "not-allowed" : "pointer",
          letterSpacing: 1,
        }}
      >
        {loading ? "计算中..." : "🧮 计算星盘"}
      </button>
    </form>
  );
}

function Field({
  label,
  children,
}: {
  label: string;
  children: React.ReactNode;
}) {
  return (
    <div style={{ marginBottom: 12 }}>
      <label
        style={{
          display: "block",
          fontSize: 12,
          color: "#888",
          marginBottom: 4,
        }}
      >
        {label}
      </label>
      {children}
    </div>
  );
}

const inputStyle: React.CSSProperties = {
  width: "100%",
  padding: "8px 10px",
  background: "#1a1a2e",
  color: "#e0e0e0",
  border: "1px solid #333",
  borderRadius: 6,
  fontSize: 14,
  outline: "none",
  boxSizing: "border-box",
};
