"use client";
import { ConfigProvider } from "@himalaya-ui/core";
import NextStyleRegistry from "@himalaya-ui/core/next/registry";
import "@fontsource-variable/instrument-sans";
import Script from "next/script";

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html>
      <body onContextMenu={(e) => e.preventDefault()}>
        <NextStyleRegistry>
          <ConfigProvider themeType={"dark"}>{children}</ConfigProvider>
        </NextStyleRegistry>
        <Script src="/input-knobs.js" strategy="beforeInteractive" />
      </body>
      <style jsx global>{`
        body {
          color-scheme: dark;
          user-select: none;
          overflow: hidden;
        }
      `}</style>
    </html>
  );
}

export const runtime = "nodejs";
