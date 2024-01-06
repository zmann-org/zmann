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
            <nav className="header">Toybox</nav>
            {children}
          </Providers>
        </StyledJsxRegistry>
        <style jsx global>{`
          body {
            overflow: hidden;
            user-select: none;
          }
          .header {
            height: 50px;
            background-color: ${theme.palette.accents_0};
            border-bottom: ${theme.palette.border};
          }
        `}</style>
      </body>
    </html>
  );
}
