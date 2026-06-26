"use client";

import { useCallback, useEffect, useState } from "react";
import { toast } from "sonner";

import {
  InstalledAppDialogs,
  InstalledAppItem,
} from "@/components/apps/installed-app-item";
import { Card, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { Skeleton } from "@/components/ui/skeleton";
import { listInstalledApps } from "@/lib/tauri/commands";
import type { InstalledApp } from "@/lib/tauri/types";

interface InstalledAppsListProps {
  refreshToken: number;
}

function InstalledAppsListSkeleton() {
  return (
    <div className="flex flex-col gap-4">
      {Array.from({ length: 3 }).map((_, index) => (
        <Card key={index}>
          <CardHeader>
            <div className="flex items-start gap-4">
              <Skeleton className="size-12" />
              <div className="flex flex-1 flex-col gap-2">
                <Skeleton className="h-5 w-40" />
                <Skeleton className="h-4 w-full max-w-sm" />
              </div>
            </div>
          </CardHeader>
        </Card>
      ))}
    </div>
  );
}

export function InstalledAppsList({ refreshToken }: InstalledAppsListProps) {
  const [apps, setApps] = useState<InstalledApp[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [appToEdit, setAppToEdit] = useState<InstalledApp | null>(null);
  const [appToRemove, setAppToRemove] = useState<InstalledApp | null>(null);
  const [appToFixDock, setAppToFixDock] = useState<InstalledApp | null>(null);

  const loadApps = useCallback(async () => {
    setIsLoading(true);
    try {
      const installed = await listInstalledApps();
      setApps(installed);
    } catch (error) {
      toast.error(
        error instanceof Error ? error.message : "Could not load installed apps",
      );
    } finally {
      setIsLoading(false);
    }
  }, []);

  useEffect(() => {
    void loadApps();
  }, [loadApps, refreshToken]);

  if (isLoading) {
    return <InstalledAppsListSkeleton />;
  }

  if (apps.length === 0) {
    return (
      <Card>
        <CardHeader>
          <CardTitle>No installed applications</CardTitle>
          <CardDescription>
            Apps you install will appear here so you can manage them.
          </CardDescription>
        </CardHeader>
      </Card>
    );
  }

  return (
    <>
      <div className="flex flex-col gap-4">
        {apps.map((app) => (
          <InstalledAppItem
            key={`${app.slug}-${app.desktopFile}`}
            app={app}
            onEdit={() => setAppToEdit(app)}
            onFixDock={() => setAppToFixDock(app)}
            onRemove={() => setAppToRemove(app)}
          />
        ))}
      </div>

      <InstalledAppDialogs
        appToEdit={appToEdit}
        appToFixDock={appToFixDock}
        appToRemove={appToRemove}
        onCloseEdit={() => setAppToEdit(null)}
        onCloseFixDock={() => setAppToFixDock(null)}
        onCloseRemove={() => setAppToRemove(null)}
        onRefresh={() => void loadApps()}
      />
    </>
  );
}
