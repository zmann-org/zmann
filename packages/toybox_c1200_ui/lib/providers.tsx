"use client";
import ConfigProvider from "@himalaya-ui/core/esm/use-context";
import React from "react";

export function Providers({
  children,
  defaultTheme,
}: {
  children: React.ReactNode;
  defaultTheme: string;
}) {
  return (
    <ConfigProvider detectTheme={true} themeType={defaultTheme}>
      {children}
    </ConfigProvider>
  );
}
