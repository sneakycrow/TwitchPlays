import { useState } from 'react'
import SVG from 'react-inlinesvg';
import './App.css'

function App() {
  const [count, setCount] = useState(0)

  return (
      <SVG
        src="/controller.svg"
        width={1200}
        height="auto"
        title="React"
        className="controller"
      />
  )
}

export default App
