"use client";

import { useEffect, useMemo, useState } from "react";
import { toast } from "sonner";

import { CategorySelect } from "@/components/shared/category-select";
import { InstallPreviewPanel } from "@/components/install/install-preview";
import { ActivityLog } from "@/components/shared/activity-log";
import { FilePickerField } from "@/components/shared/file-picker-field";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Textarea } from "@/components/ui/textarea";
import { DEFAULT_CATEGORY, type AppCategory } from "@/lib/categories";
import { buildInstallPreviewPaths } from "@/lib/paths";
import {
  installAppimage,
  parseAppName,
  suggestAppCategory,
} from "@/lib/tauri/commands";

interface InstallFormProps {
  onInstalled: () => void;
}

export function InstallForm({ onInstalled }: InstallFormProps) {
  const [appImagePath, setAppImagePath] = useState("");
  const [iconPath, setIconPath] = useState("");
  const [name, setName] = useState("");
  const [description, setDescription] = useState("");
  const [category, setCategory] = useState<AppCategory>(DEFAULT_CATEGORY);
  const [activityLog, setActivityLog] = useState<string[]>([]);
  const [isInstalling, setIsInstalling] = useState(false);

  useEffect(() => {
    if (!appImagePath) {
      return;
    }

    void parseAppName(appImagePath)
      .then(setName)
      .catch(() => {
        toast.error("Could not suggest a name from the AppImage filename");
      });
  }, [appImagePath]);

  useEffect(() => {
    if (!name.trim()) {
      return;
    }

    void suggestAppCategory(name.trim())
      .then((value) => setCategory(value as AppCategory))
      .catch(() => {
        // Keep current category if suggestion fails.
      });
  }, [name]);

  const preview = useMemo(() => {
    if (!name.trim() || !iconPath) {
      return null;
    }

    return buildInstallPreviewPaths("~", name.trim(), category);
  }, [name, category]);

  const canInstall =
    appImagePath.length > 0 &&
    iconPath.length > 0 &&
    name.trim().length > 0 &&
    !isInstalling;

  async function handleInstall() {
    if (!canInstall) {
      return;
    }

    setIsInstalling(true);
    try {
      const result = await installAppimage({
        appImagePath,
        iconPath,
        name: name.trim(),
        description: description.trim(),
        category,
      });

      if (result.success) {
        setActivityLog((current) => [...current, ...result.log]);
        toast.success(result.message);
        setAppImagePath("");
        setIconPath("");
        setName("");
        setDescription("");
        setCategory(DEFAULT_CATEGORY);
        onInstalled();
      } else {
        toast.error(result.message);
      }
    } catch (error) {
      toast.error(
        error instanceof Error ? error.message : "Installation failed",
      );
    } finally {
      setIsInstalling(false);
    }
  }

  return (
    <div className="flex flex-col gap-6">
      <FilePickerField
        id="appimage"
        label="AppImage"
        value={appImagePath}
        placeholder="Select an AppImage file"
        accept={["AppImage", "appimage"]}
        onChange={setAppImagePath}
        disabled={isInstalling}
      />

      <FilePickerField
        id="icon"
        label="Icon"
        value={iconPath}
        placeholder="Select an icon (PNG, SVG, WebP, JPEG)"
        accept={["png", "svg", "webp", "jpg", "jpeg"]}
        onChange={setIconPath}
        disabled={isInstalling}
      />

      <div className="flex flex-col gap-2">
        <Label htmlFor="name">Application name</Label>
        <Input
          id="name"
          value={name}
          onChange={(event) => setName(event.target.value)}
          placeholder="Godot"
          disabled={isInstalling}
        />
      </div>

      <div className="flex flex-col gap-2">
        <Label htmlFor="description">Description</Label>
        <Textarea
          id="description"
          value={description}
          onChange={(event) => setDescription(event.target.value)}
          placeholder="Short description for the application menu"
          disabled={isInstalling}
        />
      </div>

      <div className="flex flex-col gap-2">
        <Label htmlFor="category">Menu category</Label>
        <CategorySelect
          id="category"
          value={category}
          onChange={setCategory}
          disabled={isInstalling}
        />
        <p className="text-sm text-muted-foreground">
          Controls where the app appears in your applications menu (for example
          Audio &amp; Video for LMMS).
        </p>
      </div>

      <InstallPreviewPanel
        visible={Boolean(preview)}
        steps={preview?.steps ?? []}
      />

      <Button disabled={!canInstall} onClick={() => void handleInstall()}>
        {isInstalling ? "Installing..." : "Install application"}
      </Button>

      <div className="flex flex-col gap-2">
        <Label>Activity log</Label>
        <ActivityLog entries={activityLog} />
      </div>
    </div>
  );
}
