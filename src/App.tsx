import { AppShell } from "@/components/layout/app-shell";
import { Toaster } from "@/components/ui/sonner";

function App() {
  return (
    <>
      <AppShell />
      <Toaster richColors closeButton />
    </>
  );
}

export default App;
