"use client";

import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { APP_CATEGORIES, type AppCategory } from "@/lib/categories";

interface CategorySelectProps {
  id?: string;
  value: AppCategory;
  onChange: (value: AppCategory) => void;
  disabled?: boolean;
}

export function CategorySelect({
  id,
  value,
  onChange,
  disabled,
}: CategorySelectProps) {
  return (
    <Select
      value={value}
      onValueChange={(nextValue) => onChange(nextValue as AppCategory)}
      disabled={disabled}
    >
      <SelectTrigger id={id} className="w-full">
        <SelectValue />
      </SelectTrigger>
      <SelectContent>
        <SelectGroup>
          {APP_CATEGORIES.map((option) => (
            <SelectItem key={option.value} value={option.value}>
              {option.label}
            </SelectItem>
          ))}
        </SelectGroup>
      </SelectContent>
    </Select>
  );
}
