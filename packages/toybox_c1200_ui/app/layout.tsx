"use client";
import { ConfigProvider, StyledJsxRegistry } from "@himalaya-ui/core";
import "@fontsource-variable/instrument-sans";
import "./globals.css";
import VSTHeader from "@/lib/components/Header";
import { useEffect, useState } from "react";

declare global {
  interface Window {
    onPluginMessage: (msg: any) => void;
    onPluginMessageInternal: (msg: any) => void;
    sendToPlugin: (msg: any) => void;
    ipc: any;
  }
}

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  // const [sliderValue, setSliderValue] = useState(0);
  const [presetValue, setpresetValue] = useState("");

  const sendToPlugin = (msg: any) => {
    window.ipc.postMessage(JSON.stringify(msg));
  };

  useEffect(() => {
    window.sendToPlugin = sendToPlugin;

    window.onPluginMessageInternal = function(msg) {
      const json = JSON.parse(msg);
      window.onPluginMessage && window.onPluginMessage(json);
    }

    window.onPluginMessage = (msg: any) => {
      switch (msg.type) {
        // case "param_change": {
          //   setSliderValue(msg.value);
          //   break;
          // }
          case "preset_change": {
          setpresetValue(msg.value);
          break;
        }
      }
    };

    sendToPlugin({ type: "Init" });
  }, []);
  return (
    <html lang="en">
      <body onContextMenu={(e) => e.preventDefault()}>
        <StyledJsxRegistry>
          <ConfigProvider>
            <VSTHeader
              preset_value={presetValue}
              preset_changed={(value) => sendToPlugin({
                type: "SetPreset",
                preset: value,
              })}
            />
            {children}
          </ConfigProvider>
        </StyledJsxRegistry>
      </body>
    </html>
  );
}

export const runtime = "nodejs";