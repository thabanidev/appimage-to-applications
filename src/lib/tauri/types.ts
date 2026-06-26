export interface InstallRequest {
  appImagePath: string;
  iconPath: string;
  name: string;
  description: string;
  category: string;
}

export interface InstallPreviewStep {
  label: string;
  path: string;
}

export interface InstallPreview {
  appFolder: string;
  appImageFile: string;
  iconFile: string;
  desktopFile: string;
  steps: InstallPreviewStep[];
}

export interface UpdateAppRequest {
  slug: string;
  name: string;
  description: string;
  category: string;
}

export interface InstalledApp {
  slug: string;
  name: string;
  description: string;
  execPath: string;
  iconPath: string;
  version: string | null;
  appFolder: string;
  desktopFile: string;
  categories: string;
  startupWmClass: string | null;
  needsDockFix: boolean;
  managed: boolean;
  hasDesktopFile: boolean;
}

export interface CommandResult {
  success: boolean;
  message: string;
  log: string[];
}
