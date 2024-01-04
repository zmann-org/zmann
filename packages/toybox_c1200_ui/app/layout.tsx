"use client";
import { ConfigProvider, StyledJsxRegistry } from "@himalaya-ui/core";
import "./globals.css";
import "@fontsource-variable/instrument-sans";
import { Providers } from "./provider";

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en">
      <body>
        <StyledJsxRegistry>
          <Providers defaultTheme={"light"}>{children}</Providers>
        </StyledJsxRegistry>
      </body>
    </html>
  );
}
