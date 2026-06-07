const { contextBridge } = require("electron");

contextBridge.exposeInMainWorld("codeos", {
  version: "0.1.0-alpha",
  platform: "simulator",
});
