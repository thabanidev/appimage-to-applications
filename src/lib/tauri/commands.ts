import { invoke } from "@tauri-apps/api/core";

import type {
  CommandResult,
  InstallPreview,
  InstallRequest,
  InstalledApp,
  UpdateAppRequest,
} from "./types";

export async function parseAppName(path: string): Promise<string> {
  return invoke<string>("parse_app_name", { path });
}

export async function suggestAppCategory(name: string): Promise<string> {
  return invoke<string>("suggest_app_category", { name });
}

export async function previewInstall(
  appImagePath: string,
  iconPath: string,
  name: string,
): Promise<InstallPreview> {
  return invoke<InstallPreview>("preview_install", {
    appImagePath,
    iconPath,
    name,
  });
}

export async function installAppimage(
  request: InstallRequest,
): Promise<CommandResult> {
  return invoke<CommandResult>("install_appimage", { request });
}

export async function listInstalledApps(): Promise<InstalledApp[]> {
  return invoke<InstalledApp[]>("list_installed_apps");
}

export async function removeInstalledApp(slug: string): Promise<CommandResult> {
  return invoke<CommandResult>("remove_installed_app", { slug });
}

export async function fixDockGrouping(slug: string): Promise<CommandResult> {
  return invoke<CommandResult>("fix_dock_grouping", { slug });
}

export async function updateInstalledApp(
  request: UpdateAppRequest,
): Promise<CommandResult> {
  return invoke<CommandResult>("update_installed_app", { request });
}
