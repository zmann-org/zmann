'use client';

import { ConfigProvider, LayoutProvider } from '@himalaya-ui/core';
import React from 'react';

export function Providers({ children, defaultTheme }: { children: React.ReactNode; defaultTheme: string }) {
  return (
    <ConfigProvider detectTheme={true} themeType={defaultTheme}>
      <LayoutProvider>
        {children}
      </LayoutProvider>
    </ConfigProvider>
  );
}