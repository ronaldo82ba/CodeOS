import { DeviceShell } from "./components/DeviceShell";
import { LauncherGrid } from "./components/LauncherGrid";
import { StatusBar } from "./components/StatusBar";

export default function App() {
  const version = window.codeos?.version ?? "0.1.0-alpha";

  return (
    <DeviceShell>
      <StatusBar version={version} />
      <main className="codeos-main">
        <h1 className="codeos-title">CodeOS</h1>
        <p className="codeos-subtitle">Simulator · v0.1</p>
        <LauncherGrid />
      </main>
    </DeviceShell>
  );
}
