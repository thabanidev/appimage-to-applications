"use client";

import { useState } from "react";

import { InstalledAppsList } from "@/components/apps/installed-apps-list";
import { InstallForm } from "@/components/install/install-form";
import { AppHeader } from "@/components/layout/app-header";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";

export function AppShell() {
  const [refreshToken, setRefreshToken] = useState(0);

  return (
    <div className="mx-auto flex min-h-screen w-full max-w-3xl flex-col gap-6 p-6">
      <AppHeader />

      <Tabs defaultValue="install" className="w-full">
        <TabsList>
          <TabsTrigger value="install">Install</TabsTrigger>
          <TabsTrigger value="installed">Installed</TabsTrigger>
        </TabsList>

        <TabsContent value="install">
          <Card>
            <CardHeader>
              <CardTitle>Install AppImage</CardTitle>
              <CardDescription>
                Choose an AppImage and icon, review what will happen, then
                install.
              </CardDescription>
            </CardHeader>
            <CardContent>
              <InstallForm
                onInstalled={() => setRefreshToken((value) => value + 1)}
              />
            </CardContent>
          </Card>
        </TabsContent>

        <TabsContent value="installed">
          <InstalledAppsList refreshToken={refreshToken} />
        </TabsContent>
      </Tabs>
    </div>
  );
}
