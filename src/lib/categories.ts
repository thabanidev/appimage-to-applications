export const APP_CATEGORIES = [
  { value: "AudioVideo", label: "Audio & Video" },
  { value: "Development", label: "Development" },
  { value: "Education", label: "Education" },
  { value: "Game", label: "Games" },
  { value: "Graphics", label: "Graphics" },
  { value: "Network", label: "Internet & Network" },
  { value: "Office", label: "Office" },
  { value: "Science", label: "Science" },
  { value: "Settings", label: "Settings" },
  { value: "System", label: "System" },
  { value: "Utility", label: "Utility" },
] as const;

export type AppCategory = (typeof APP_CATEGORIES)[number]["value"];

export const DEFAULT_CATEGORY: AppCategory = "Utility";

export function getCategoryLabel(value: string): string {
  return APP_CATEGORIES.find((category) => category.value === value)?.label ?? value;
}

export function suggestCategory(name: string): AppCategory {
  const lower = name.toLowerCase();

  if (/(lmms|audacity|spotify|vlc|ffmpeg|ardour|reaper)/.test(lower)) {
    return "AudioVideo";
  }
  if (/(godot|unity|unreal|blender|gimp|inkscape|krita)/.test(lower)) {
    return "Graphics";
  }
  if (/(code|cursor|vscode|idea|dev|git|docker)/.test(lower)) {
    return "Development";
  }
  if (/(game|steam|minecraft|lutris)/.test(lower)) {
    return "Game";
  }
  if (/(firefox|chrome|browser|discord|slack)/.test(lower)) {
    return "Network";
  }
  if (/(libreoffice|onlyoffice|writer|calc)/.test(lower)) {
    return "Office";
  }

  return DEFAULT_CATEGORY;
}
