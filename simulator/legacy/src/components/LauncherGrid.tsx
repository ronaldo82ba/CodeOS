const APPS = [
  { id: "com.codeos.launcher", name: "Launcher", icon: "🏠" },
  { id: "com.codeos.settings", name: "Settings", icon: "⚙️" },
];

export function LauncherGrid() {
  return (
    <div className="codeos-launcher-grid">
      {APPS.map((app) => (
        <button key={app.id} className="codeos-app-icon" type="button">
          <span className="codeos-app-icon-emoji">{app.icon}</span>
          <span className="codeos-app-icon-label">{app.name}</span>
        </button>
      ))}
    </div>
  );
}
