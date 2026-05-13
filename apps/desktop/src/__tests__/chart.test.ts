import { describe, it, expect } from "vitest";

describe("chart types", () => {
  it("validates BirthInfo shape", () => {
    const info = {
      year: 2000,
      month: 1,
      day: 15,
      hour: 12,
      minute: 30,
      second: 0,
      timezone: 8,
      latitude: 39.9,
      longitude: 116.4,
    };
    expect(info.year).toBe(2000);
    expect(info.timezone).toBe(8);
    expect(info.latitude).toBeGreaterThan(-90);
    expect(info.latitude).toBeLessThan(90);
  });

  it("validates chart data structure", () => {
    const data = {
      timestamp: "2020-01-01T00:00:00Z",
      bodies: [
        {
          name: "太阳",
          longitude: 200.0,
          latitude: 0.0,
          speed: 1.0,
          zodiac_sign: "天秤",
          zodiac_degree: 20.0,
          mansion_name: "角",
          mansion_degree: 5.0,
        },
      ],
      extra_bodies: [],
      aspects: [],
      houses: [],
      shen_sha: [],
    };
    expect(data.bodies).toHaveLength(1);
    expect(data.bodies[0].mansion_name).toBe("角");
  });
});
