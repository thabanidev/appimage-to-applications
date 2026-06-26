"use client";

import { InfoIcon } from "lucide-react";
import { useState } from "react";
import { toast } from "sonner";

import { Alert, AlertDescription, AlertTitle } from "@/components/ui/alert";
import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { fixDockGrouping } from "@/lib/tauri/commands";
import type { InstalledApp } from "@/lib/tauri/types";

interface FixDockDialogProps {
  app: InstalledApp | null;
  onClose: () => void;
  onFixed: () => void;
}

export function FixDockDialog({ app, onClose, onFixed }: FixDockDialogProps) {
  const [isFixing, setIsFixing] = useState(false);

  async function handleFix() {
    if (!app) {
      return;
    }

    setIsFixing(true);
    try {
      const result = await fixDockGrouping(app.slug);
      if (result.success) {
        toast.success(result.message);
        onFixed();
        onClose();
      } else {
        toast.error(result.message);
      }
    } catch (error) {
      toast.error(
        error instanceof Error ? error.message : "Could not fix dock grouping",
      );
    } finally {
      setIsFixing(false);
    }
  }

  return (
    <Dialog open={Boolean(app)} onOpenChange={(open) => !open && onClose()}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Fix dock icon for {app?.name}</DialogTitle>
          <DialogDescription>
            Some AppImages open a second dock icon instead of grouping with the
            launcher you pinned.
          </DialogDescription>
        </DialogHeader>

        <div className="flex flex-col gap-4">
          <Alert>
            <InfoIcon />
            <AlertTitle>What is happening?</AlertTitle>
            <AlertDescription>
              Your desktop groups running apps using a window identifier called
              StartupWMClass. If that value does not match how {app?.name}{" "}
              identifies its window, Linux shows a separate dock icon while the
              app is running.
            </AlertDescription>
          </Alert>

          <Alert>
            <InfoIcon />
            <AlertTitle>What this fix does</AlertTitle>
            <AlertDescription>
              We briefly launch {app?.name}, detect the correct window
              identifier, update your launcher, and refresh the applications
              menu. After it finishes, quit {app?.name} completely and open it
              again from your applications menu.
            </AlertDescription>
          </Alert>
        </div>

        <DialogFooter>
          <Button variant="outline" onClick={onClose} disabled={isFixing}>
            Cancel
          </Button>
          <Button disabled={isFixing} onClick={() => void handleFix()}>
            {isFixing ? "Fixing..." : "Launch and fix"}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
