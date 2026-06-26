"use client";

import { useState } from "react";

import { cn } from "@/lib/utils";
import { getAppInitials } from "@/lib/app-icon";

interface AppIconImageProps {
  src?: string;
  name: string;
  size?: number;
  className?: string;
}

export function AppIconImage({
  src,
  name,
  size = 48,
  className,
}: AppIconImageProps) {
  const [hasError, setHasError] = useState(false);

  if (!src || hasError) {
    return (
      <div
        className={cn(
          "flex shrink-0 items-center justify-center bg-muted text-sm font-medium text-muted-foreground",
          className,
        )}
        style={{ width: size, height: size }}
      >
        {getAppInitials(name)}
      </div>
    );
  }

  return (
    <img
      src={src}
      alt={`${name} icon`}
      width={size}
      height={size}
      className={cn("shrink-0 object-contain", className)}
      onError={() => setHasError(true)}
    />
  );
}
