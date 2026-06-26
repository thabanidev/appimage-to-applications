"use client";

import { useEffect, useState } from "react";
import { toast } from "sonner";

import { CategorySelect } from "@/components/shared/category-select";
import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Textarea } from "@/components/ui/textarea";
import { DEFAULT_CATEGORY, type AppCategory } from "@/lib/categories";
import { suggestAppCategory, updateInstalledApp } from "@/lib/tauri/commands";
import type { InstalledApp } from "@/lib/tauri/types";

interface EditAppDialogProps {
  app: InstalledApp | null;
  onClose: () => void;
  onSaved: () => void;
}

export function EditAppDialog({ app, onClose, onSaved }: EditAppDialogProps) {
  const [name, setName] = useState("");
  const [description, setDescription] = useState("");
  const [category, setCategory] = useState<AppCategory>(DEFAULT_CATEGORY);
  const [isSaving, setIsSaving] = useState(false);

  useEffect(() => {
    if (!app) {
      return;
    }

    setName(app.name);
    setDescription(app.description);
    setCategory((app.categories as AppCategory) || DEFAULT_CATEGORY);
  }, [app]);

  useEffect(() => {
    if (!name.trim() || !app) {
      return;
    }

    void suggestAppCategory(name.trim())
      .then((value) => {
        if (app.name === name) {
          return;
        }
        setCategory(value as AppCategory);
      })
      .catch(() => undefined);
  }, [name, app]);

  async function handleSave() {
    if (!app || !name.trim()) {
      return;
    }

    setIsSaving(true);
    try {
      const result = await updateInstalledApp({
        slug: app.slug,
        name: name.trim(),
        description: description.trim(),
        category,
      });

      if (result.success) {
        toast.success(result.message);
        onSaved();
        onClose();
      } else {
        toast.error(result.message);
      }
    } catch (error) {
      toast.error(error instanceof Error ? error.message : "Could not save changes");
    } finally {
      setIsSaving(false);
    }
  }

  return (
    <Dialog open={Boolean(app)} onOpenChange={(open) => !open && onClose()}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Edit application</DialogTitle>
          <DialogDescription>
            Update how this app appears in your applications menu.
          </DialogDescription>
        </DialogHeader>

        <div className="flex flex-col gap-4">
          <div className="flex flex-col gap-2">
            <Label htmlFor="edit-name">Name</Label>
            <Input
              id="edit-name"
              value={name}
              onChange={(event) => setName(event.target.value)}
              disabled={isSaving || !app?.hasDesktopFile}
            />
          </div>

          <div className="flex flex-col gap-2">
            <Label htmlFor="edit-description">Description</Label>
            <Textarea
              id="edit-description"
              value={description}
              onChange={(event) => setDescription(event.target.value)}
              disabled={isSaving || !app?.hasDesktopFile}
            />
          </div>

          <div className="flex flex-col gap-2">
            <Label htmlFor="edit-category">Menu category</Label>
            <CategorySelect
              id="edit-category"
              value={category}
              onChange={setCategory}
              disabled={isSaving || !app?.hasDesktopFile}
            />
          </div>
        </div>

        <DialogFooter>
          <Button variant="outline" onClick={onClose} disabled={isSaving}>
            Cancel
          </Button>
          <Button
            disabled={isSaving || !app?.hasDesktopFile || !name.trim()}
            onClick={() => void handleSave()}
          >
            {isSaving ? "Saving..." : "Save changes"}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
