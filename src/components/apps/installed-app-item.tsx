"use client";

import { EditAppDialog } from "@/components/apps/edit-app-dialog";
import { RemoveAppDialog } from "@/components/apps/remove-app-dialog";
import { AppIconImage } from "@/components/shared/app-icon-image";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { getAppIconSrc } from "@/lib/app-icon";
import { getCategoryLabel } from "@/lib/categories";
import type { InstalledApp } from "@/lib/tauri/types";

interface InstalledAppItemProps {
  app: InstalledApp;
  onEdit: () => void;
  onRemove: () => void;
}

export function InstalledAppItem({
  app,
  onEdit,
  onRemove,
}: InstalledAppItemProps) {
  const iconSrc = getAppIconSrc(app.iconPath);

  return (
    <Card>
      <CardHeader className="pb-4">
        <div className="flex items-start gap-4">
          <AppIconImage
            src={iconSrc}
            name={app.name}
            size={48}
          />

          <div className="flex min-w-0 flex-1 flex-col gap-1">
            <div className="flex flex-wrap items-start justify-between gap-2">
              <CardTitle className="truncate">{app.name}</CardTitle>
              <div className="flex flex-wrap gap-2">
                {app.version ? (
                  <Badge variant="outline">v{app.version}</Badge>
                ) : null}
                {app.categories ? (
                  <Badge variant="secondary">
                    {getCategoryLabel(app.categories)}
                  </Badge>
                ) : null}
              </div>
            </div>

            {app.description ? (
              <CardDescription className="line-clamp-2">
                {app.description}
              </CardDescription>
            ) : (
              <CardDescription>No description</CardDescription>
            )}
          </div>
        </div>
      </CardHeader>

      <CardContent className="flex flex-wrap gap-2">
        {app.hasDesktopFile ? (
          <Button variant="outline" onClick={onEdit}>
            Edit
          </Button>
        ) : null}
        <Button variant="destructive" onClick={onRemove}>
          Remove
        </Button>
      </CardContent>
    </Card>
  );
}

interface InstalledAppDialogsProps {
  appToEdit: InstalledApp | null;
  appToRemove: InstalledApp | null;
  onCloseEdit: () => void;
  onCloseRemove: () => void;
  onRefresh: () => void;
}

export function InstalledAppDialogs({
  appToEdit,
  appToRemove,
  onCloseEdit,
  onCloseRemove,
  onRefresh,
}: InstalledAppDialogsProps) {
  return (
    <>
      <EditAppDialog
        app={appToEdit}
        onClose={onCloseEdit}
        onSaved={onRefresh}
      />

      <RemoveAppDialog
        app={appToRemove}
        onClose={onCloseRemove}
        onRemoved={() => {
          onCloseRemove();
          onRefresh();
        }}
      />
    </>
  );
}
