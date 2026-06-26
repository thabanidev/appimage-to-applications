"use client";

import { FolderOpen } from "lucide-react";
import { useRef } from "react";

import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { cn } from "@/lib/utils";

interface FilePickerFieldProps {
  id: string;
  label: string;
  value: string;
  placeholder: string;
  accept?: string[];
  onChange: (path: string) => void;
  disabled?: boolean;
}

export function FilePickerField({
  id,
  label,
  value,
  placeholder,
  accept,
  onChange,
  disabled,
}: FilePickerFieldProps) {
  const inputRef = useRef<HTMLInputElement>(null);

  async function handleBrowse() {
    const { open } = await import("@tauri-apps/plugin-dialog");
    const selected = await open({
      multiple: false,
      directory: false,
      filters: accept
        ? [{ name: label, extensions: accept }]
        : undefined,
    });

    if (typeof selected === "string") {
      onChange(selected);
    }
  }

  return (
    <div className="flex flex-col gap-2">
      <Label htmlFor={id}>{label}</Label>
      <div className="flex gap-2">
        <Input
          ref={inputRef}
          id={id}
          readOnly
          value={value}
          placeholder={placeholder}
          className={cn("font-mono text-sm")}
        />
        <Button
          type="button"
          variant="outline"
          disabled={disabled}
          onClick={() => void handleBrowse()}
        >
          <FolderOpen data-icon="inline-start" />
          Browse
        </Button>
      </div>
    </div>
  );
}
