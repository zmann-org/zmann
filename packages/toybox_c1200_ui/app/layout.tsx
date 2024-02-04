"use client";
import { ConfigProvider, StyledJsxRegistry } from "@himalaya-ui/core";
import "@fontsource-variable/instrument-sans";
import "./globals.css";
import Script from "next/script";

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html>
      <body onContextMenu={(e) => e.preventDefault()}>
        <StyledJsxRegistry>
          <ConfigProvider themeType={"dark"}>{children}</ConfigProvider>
        </StyledJsxRegistry>
        <Script
          src="/input-knobs.js"
          strategy="beforeInteractive"
        />
      </body>
    </html>
  );
}

export const runtime = "nodejs";
