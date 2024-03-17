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
    `}</style>
  </React.StrictMode>
);
