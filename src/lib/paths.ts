export function slugify(name: string): string {
  return name
    .toLowerCase()
    .split(/[^a-z0-9]+/)
    .filter(Boolean)
    .join("-");
}

export function buildInstallPreviewPaths(
  homeDir: string,
  name: string,
  category: string,
): {
  appFolder: string;
  appImageFile: string;
  iconFile: string;
  desktopFile: string;
  steps: { label: string; path: string }[];
} {
  const slug = slugify(name);
  const appFolder = `${homeDir}/Applications/${name}`;
  const appImageFile = name;
  const iconFile = "icon.png";
  const desktopFile = `${homeDir}/.local/share/applications/${slug}.desktop`;

  return {
    appFolder,
    appImageFile,
    iconFile,
    desktopFile,
    steps: [
      { label: "Create folder", path: appFolder },
      { label: "Copy application", path: appImageFile },
      { label: "Copy icon", path: iconFile },
      { label: "Set menu category", path: category },
      { label: "Create desktop launcher", path: desktopFile },
    ],
  };
}
