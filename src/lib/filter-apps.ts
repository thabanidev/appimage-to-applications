import type { InstalledApp } from "@/lib/tauri/types";

export function filterAppsByQuery(
  apps: InstalledApp[],
  query: string,
): InstalledApp[] {
  const needle = query.trim().toLowerCase();
  if (!needle) {
    return apps;
  }

  return apps.filter(
    (app) =>
      app.name.toLowerCase().includes(needle) ||
      app.description.toLowerCase().includes(needle) ||
      app.categories.toLowerCase().includes(needle),
  );
}
