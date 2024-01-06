"use client";
import { ConfigProvider, StyledJsxRegistry } from "@himalaya-ui/core";
import "@fontsource-variable/instrument-sans";
import "./globals.css";
import VSTHeader from "@/lib/components/Header";
import { useEffect, useState } from "react";

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  // const [sliderValue, setSliderValue] = useState(0);
  const [presetValue, setpresetValue] = useState("");

  useEffect(() => {
    (window as any).onPluginMessage = (msg: any) => {
      switch (msg.type) {
        // case "param_change": {
        //   setSliderValue(msg.value);
        //   break;
        // }
        case "instrument_change": {
          setpresetValue(msg.value);
          break;
        }
      }
    };
  }, []);
  return (
    <html lang="en">
      <body onContextMenu={(e) => e.preventDefault()}>
        <StyledJsxRegistry>
          <ConfigProvider>
            <VSTHeader
              preset_value={presetValue}
              //@ts-ignore
              preset_changed={sendToPlugin({
                type: "SetPreset",
                preset: presetValue,
              })}
            />
            {children}
          </ConfigProvider>
        </StyledJsxRegistry>
      </body>
    </html>
  );
}
