export interface Unit {
  id: number;
  entity_type: string;
  last_name: string | null;
  first_name: string | null;
  middle_name: string | null;
  pet_name: string | null;
  gender: string | null;
  photo_path: string | null;

  birth_year: number | null;
  birth_year_qual: string | null;
  birth_month: number | null;
  birth_month_qual: string | null;
  birth_day: number | null;
  birth_day_qual: string | null;

  death_year: number | null;
  death_year_qual: string | null;
  death_month: number | null;
  death_month_qual: string | null;
  death_day: number | null;
  death_day_qual: string | null;
}