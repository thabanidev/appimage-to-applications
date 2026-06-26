"use client";

import { useEffect, useState } from "react";
import { toast } from "sonner";

import { Button } from "@/components/ui/button";
import { Checkbox } from "@/components/ui/checkbox";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { Label } from "@/components/ui/label";
import { removeInstalledApp } from "@/lib/tauri/commands";
import type { InstalledApp } from "@/lib/tauri/types";

interface RemoveAppDialogProps {
  app: InstalledApp | null;
  onClose: () => void;
  onRemoved: () => void;
}

export function RemoveAppDialog({
  app,
  onClose,
  onRemoved,
}: RemoveAppDialogProps) {
  const [isRemoving, setIsRemoving] = useState(false);
  const [confirmed, setConfirmed] = useState(false);

  useEffect(() => {
    if (!app) {
      setConfirmed(false);
      setIsRemoving(false);
    }
  }, [app]);

  async function handleRemove() {
    if (!app) {
      return;
    }

    setIsRemoving(true);
    try {
      const result = await removeInstalledApp(app.slug);
      if (result.success) {
        toast.success(result.message);
        onRemoved();
        onClose();
      } else {
        toast.error(result.message);
      }
    } catch (error) {
      toast.error(error instanceof Error ? error.message : "Removal failed");
    } finally {
      setIsRemoving(false);
    }
  }

  return (
    <Dialog open={Boolean(app)} onOpenChange={(open) => !open && onClose()}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Remove {app?.name}?</DialogTitle>
          <DialogDescription>
            This permanently deletes the application folder and removes it from
            your applications menu. This cannot be undone.
          </DialogDescription>
        </DialogHeader>

        <div className="flex items-start gap-3">
          <Checkbox
            id="remove-confirm"
            checked={confirmed}
            onCheckedChange={(checked) => setConfirmed(checked === true)}
            disabled={isRemoving}
          />
          <Label htmlFor="remove-confirm" className="font-normal leading-snug">
            I understand that {app?.name} will be removed from my computer.
          </Label>
        </div>

        <DialogFooter>
          <Button variant="outline" onClick={onClose} disabled={isRemoving}>
            Cancel
          </Button>
          <Button
            variant="destructive"
            disabled={isRemoving || !confirmed}
            onClick={() => void handleRemove()}
          >
            {isRemoving ? "Removing..." : "Yes, remove application"}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
