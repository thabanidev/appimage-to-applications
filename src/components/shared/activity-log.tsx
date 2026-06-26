import { ScrollArea } from "@/components/ui/scroll-area";

interface ActivityLogProps {
  entries: string[];
}

export function ActivityLog({ entries }: ActivityLogProps) {
  if (entries.length === 0) {
    return (
      <p className="text-sm text-muted-foreground">
        Actions will appear here as they complete.
      </p>
    );
  }

  return (
    <ScrollArea className="h-40 rounded-md border p-3">
      <ul className="flex flex-col gap-2 font-mono text-sm">
        {entries.map((entry, index) => (
          <li key={`${entry}-${index}`}>{entry}</li>
        ))}
      </ul>
    </ScrollArea>
  );
}
