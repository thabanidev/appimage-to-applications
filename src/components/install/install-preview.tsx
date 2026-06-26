import { Check } from "lucide-react";

import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";

interface InstallPreviewProps {
  steps: { label: string; path: string }[];
  visible: boolean;
}

export function InstallPreviewPanel({ steps, visible }: InstallPreviewProps) {
  if (!visible) {
    return null;
  }

  return (
    <Card>
      <CardHeader>
        <CardTitle>What will happen</CardTitle>
        <CardDescription>
          Review these steps before installing. Nothing is written until you
          confirm.
        </CardDescription>
      </CardHeader>
      <CardContent>
        <ul className="flex flex-col gap-4">
          {steps.map((step) => (
            <li key={step.label} className="flex flex-col gap-1">
              <div className="flex items-center gap-2 text-sm font-medium">
                <Check className="size-4 text-primary" />
                {step.label}
              </div>
              <p className="pl-6 font-mono text-sm text-muted-foreground">
                {step.path}
              </p>
            </li>
          ))}
        </ul>
      </CardContent>
    </Card>
  );
}
