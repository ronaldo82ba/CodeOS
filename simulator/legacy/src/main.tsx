import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import "./styles/codeos.css";

ReactDOM.createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);

declare global {
  interface Window {
    codeos?: { version: string; platform: string };
  }
}
