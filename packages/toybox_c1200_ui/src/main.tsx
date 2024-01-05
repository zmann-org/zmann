import React from 'react'
import ReactDOM from 'react-dom/client'
import App from './App.tsx'
import './index.css'
import StyledJsxRegistry from '@himalaya-ui/core/esm/use-context/registry'
import ConfigProvider from '@himalaya-ui/core/esm/use-context/config-provider'
import '@fontsource-variable/instrument-sans'

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <StyledJsxRegistry>
      <ConfigProvider>
          <App />
      </ConfigProvider>
    </StyledJsxRegistry>
  </React.StrictMode>,
)
