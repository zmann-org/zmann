import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App.tsx";
import { GeistProvider, CssBaseline, Themes } from "altea";

const toyboxTheme = Themes.createFromDark({
  type: "toybox",
  palette: {
    accents_1: "#282729",
    background: "#121113",
  },
});

ReactDOM.createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <GeistProvider themes={[toyboxTheme]} themeType="toybox">
      <CssBaseline />
      <App />
    </GeistProvider>
    <style jsx global>{`
      body {
        color-scheme: dark;
        user-select: none;
        overflow: hidden;
        background: #121113;
      }
      // Custom select dropdown
      .select-dropdown {
        background: #2c2c2c !important;
      }
      .select-dropdown .option:not(:hover) {
        background: inherit !important;
        border-radius: 4px;
      }
      .select-dropdown .option:hover {
        background: #9f9f9f !important;
        border-radius: 4px;
        color: #000 !important;
      }
      .select {
        background: #212023 !important;
      }
    `}</style>
  </React.StrictMode>
);
