interface Props {
  version: string;
}

export function StatusBar({ version }: Props) {
  const now = new Date().toLocaleTimeString([], {
    hour: "2-digit",
    minute: "2-digit",
  });

  return (
    <header className="codeos-status-bar">
      <span>{now}</span>
      <span className="codeos-status-center">CodeOS</span>
      <span>{version}</span>
    </header>
  );
}
