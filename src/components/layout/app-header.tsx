import { APP_DISPLAY_NAME } from "@/lib/branding";

export function AppHeader() {
  return (
    <header className="flex items-center gap-3 border-b pb-4">
      <img
        src="/icon.png"
        alt={APP_DISPLAY_NAME}
        width={40}
        height={40}
        className="size-10 shrink-0 object-contain"
      />
      <div className="flex min-w-0 flex-col gap-1">
        <h1 className="text-2xl font-semibold tracking-tight">
          {APP_DISPLAY_NAME}
        </h1>
        <p className="text-sm text-muted-foreground">
          Turn an AppImage into a normal desktop application with a launcher,
          icon, and folder in ~/Applications.
        </p>
      </div>
    </header>
  );
}
