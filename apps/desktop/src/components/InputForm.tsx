import { useState, FormEvent } from "react";
import { BirthInfo } from "../types/chart";
import { Field, inputStyle } from "./shared";
import { theme } from "../theme";

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
  const [isMale, setIsMale] = useState(true);
  const [dst, setDst] = useState(false);

  function handleSubmit(e: FormEvent) {
    e.preventDefault();
    onCalculate({ year, month, day, hour, minute, second, timezone, latitude, longitude, dst_applied: dst, isMale });
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

      <div style={{ display: "flex", gap: theme.spacing.sm }}>
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

      <div style={{ display: "flex", gap: theme.spacing.sm }}>
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

      <div style={{ display: "flex", gap: theme.spacing.sm }}>
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

      <div style={{ marginBottom: theme.spacing.md }}>
        <button
          type="button"
          onClick={() => setIsMale(!isMale)}
          style={{
            width: "100%",
            padding: "8px 10px",
            background: isMale ? theme.colors.accent.dark : "transparent",
            color: isMale ? theme.colors.accent.primary : theme.colors.text.secondary,
            border: `1px solid ${isMale ? theme.colors.accent.primary : theme.colors.border.default}`,
            borderRadius: theme.radius.md,
            fontSize: theme.fontSize.lg,
            cursor: "pointer",
            textAlign: "center",
            marginBottom: theme.spacing.sm,
          }}
        >
          性别: {isMale ? "♂ 男" : "♀ 女"}
        </button>
        <button
          type="button"
          onClick={() => setDst(!dst)}
          style={{
            width: "100%",
            padding: "8px 10px",
            background: dst ? theme.colors.accent.dark : theme.colors.bg.input,
            color: dst ? theme.colors.accent.primary : theme.colors.text.secondary,
            border: `1px solid ${dst ? theme.colors.accent.primary : theme.colors.border.default}`,
            borderRadius: theme.radius.md,
            fontSize: theme.fontSize.lg,
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
          marginTop: theme.spacing.lg,
          padding: "10px 0",
          background: loading ? theme.colors.bg.disabled : theme.colors.accent.muted,
          color: theme.colors.text.inverse,
          border: "none",
          borderRadius: theme.radius.lg,
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
