export interface CelestialBody {
  name: string;
  longitude: number;
  latitude: number;
  speed: number;
  zodiac_sign: [string, number];
  mansion_name: string;
  mansion_degree: number;
}

export interface ExtraBody {
  name: string;
  longitude: number;
  mansion_name: string;
  mansion_degree: number;
}

export interface Aspect {
  point1: string;
  point2: string;
  angle: number;
  aspect_type: string;
  orb: number;
}

export interface ShenSha {
  name: string;
  category: string;
  quality: string;
}

export interface ChartData {
  timestamp: string;
  bodies: CelestialBody[];
  extra_bodies: ExtraBody[];
  aspects: Aspect[];
  houses: HouseData[];
  shen_sha: ShenSha[];
}

export interface BirthInfo {
  year: number;
  month: number;
  day: number;
  hour: number;
  minute: number;
  second: number;
  timezone: number;
  latitude: number;
  longitude: number;
}

export interface HouseData {
  index: number;
  longitude: number;
  mansion_name: string;
  mansion_degree: number;
}
