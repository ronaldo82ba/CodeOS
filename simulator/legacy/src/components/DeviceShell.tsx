import { ReactNode } from "react";

interface Props {
  children: ReactNode;
}

export function DeviceShell({ children }: Props) {
  return <div className="codeos-device">{children}</div>;
}
