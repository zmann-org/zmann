"use client";
import { StyledJsxRegistry, useTheme } from "@himalaya-ui/core";
import "./globals.css";
import "@fontsource-variable/instrument-sans";
import { Providers } from "@/lib/providers";

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  const theme = useTheme();
  return (
    <html lang="en" onContextMenu={(e) => e.preventDefault()}>
      <body>
        <StyledJsxRegistry>
          <Providers defaultTheme={"dark"}>
            <header>hello</header>
            {children}
          </Providers>
        </StyledJsxRegistry>
        <style jsx global>{`
          body {
            overflow: hidden;
            user-select: none;
          }
          header {
            background-color: ${theme.palette.accents_0};
          }
        `}</style>
      </body>
    </html>
  );
}
