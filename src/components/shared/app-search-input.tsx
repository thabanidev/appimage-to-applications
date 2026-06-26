"use client";

import { SearchIcon } from "lucide-react";

import { Input } from "@/components/ui/input";

interface AppSearchInputProps {
  value: string;
  onChange: (value: string) => void;
  placeholder?: string;
}

export function AppSearchInput({
  value,
  onChange,
  placeholder = "Search applications...",
}: AppSearchInputProps) {
  return (
    <div className="relative">
      <SearchIcon className="pointer-events-none absolute top-1/2 left-3 size-4 -translate-y-1/2 text-muted-foreground" />
      <Input
        className="pl-9"
        placeholder={placeholder}
        value={value}
        onChange={(event) => onChange(event.target.value)}
      />
    </div>
  );
}
