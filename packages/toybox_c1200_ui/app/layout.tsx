"use client";
import { ConfigProvider, StyledJsxRegistry } from "@himalaya-ui/core";
import "@fontsource-variable/instrument-sans";
import "./globals.css";

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en">
      <body onContextMenu={(e) => e.preventDefault()}>
        <StyledJsxRegistry>
          <ConfigProvider>{children}</ConfigProvider>
        </StyledJsxRegistry>
      </body>
    </html>
  );
}

export const runtime = "nodejs";
