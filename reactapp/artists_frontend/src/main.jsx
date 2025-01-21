import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import './index.css'
import App from './App.jsx'
import Artists from './components/Artists.jsx'

createRoot(document.getElementById('root')).render(
  <StrictMode>
    <App />
    <Artists />
  </StrictMode>,
)
