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

export interface Pillar {
  heavenly_stem: string;
  earthly_branch: string;
  stem_index: number;
  branch_index: number;
}

export interface DayunPillar {
  age: number;
  heavenly_stem: string;
  earthly_branch: string;
  stem_index: number;
  branch_index: number;
}

export interface ShishenItem {
  pillar_name: string;
  stem: string;
  shishen: string;
}

export interface HiddenStemInfo {
  branch_name: string;
  stems: string[];
}

export interface LifeCycleItem {
  branch_name: string;
  stage: string;
}

export interface BaziData {
  year_pillar: Pillar;
  month_pillar: Pillar;
  day_pillar: Pillar;
  hour_pillar: Pillar;
  dayun: DayunPillar[];
  shishen: ShishenItem[];
  hidden_stems: HiddenStemInfo[];
  life_cycle: LifeCycleItem[];
  taiyuan: Pillar;
}

export interface ChartData {
  timestamp: string;
  bodies: CelestialBody[];
  extra_bodies: ExtraBody[];
  aspects: Aspect[];
  houses: HouseData[];
  shen_sha: ShenSha[];
  ascendant: number;
  midheaven: number;
  part_of_fortune: number;
  bazi: BaziData;
  shiganhuayao: [string, string][];
  ming_zhu: string;
  shen_zhu: string;
  xijige: [string, string][];
  xiaoxian_result: [string, number];
  yuexian_result: [string, number];
  dongweifeixian_result: [number, string, string][];
  zodiac_type: string;
  ayanamsa: number;
  dst_applied: boolean;
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
  dst_applied: boolean;
}

export interface ElectionalInfo extends BirthInfo {
  event_name?: string;
  event_type?: string;
}

export interface HouseData {
  index: number;
  longitude: number;
  mansion_name: string;
  mansion_degree: number;
}
