const MONTHS_RU = [
  "января", "февраля", "марта", "апреля", "мая", "июня",
  "июля", "августа", "сентября", "октября", "ноября", "декабря",
];

/**
 * Собирает строку из компонентов даты с учётом квалификаторов.
 * qual: 'exact' | 'approx' | 'unknown' | null
 */
export function formatFuzzyDate(
  year: number | null, yearQual: string | null,
  month: number | null, monthQual: string | null,
  day: number | null, dayQual: string | null,
): string {
  // Год неизвестен или отсутствует — вся дата неизвестна
  if (year == null || yearQual === "unknown") return "неизвестно";

  const y = yearQual === "approx" ? `${year}??` : `${year}`;

  // Месяц неизвестен — показываем только год
  if (month == null || monthQual === "unknown") return y;

  const monthName = MONTHS_RU[month - 1] ?? `месяц ${month}`;
  const m = monthQual === "approx" ? `${monthName}??` : monthName;

  // День неизвестен — показываем месяц и год
  if (day == null || dayQual === "unknown") return `${m} ${y}`;

  const d = dayQual === "approx" ? `${day}??` : `${day}`;
  return `${d} ${m} ${y}`;
}