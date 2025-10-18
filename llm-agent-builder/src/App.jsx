import { useState } from 'react'
import Canvas from './components/Canvas'
import Toolbar from './components/Toolbar'
import './App.css'

function App() {
  const [nodes, setNodes] = useState([])
  const [connections, setConnections] = useState([])
  const [selectedNode, setSelectedNode] = useState(null)
  const [nextId, setNextId] = useState(1)

  const addNode = (type) => {
    const newNode = {
      id: nextId,
      type: type,
      x: 100 + Math.random() * 200,
      y: 100 + Math.random() * 200,
      prompt: '',
      model: 'GPT-4',
      temperature: 0.7,
    }
    setNodes([...nodes, newNode])
    setNextId(nextId + 1)
  }

  const updateNode = (id, updates) => {
    setNodes(nodes.map(node => 
      node.id === id ? { ...node, ...updates } : node
    ))
  }

  const deleteNode = (id) => {
    setNodes(nodes.filter(node => node.id !== id))
    setConnections(connections.filter(
      conn => conn.from !== id && conn.to !== id
    ))
    if (selectedNode?.id === id) {
      setSelectedNode(null)
    }
  }

  const addConnection = (fromId, toId) => {
    // Prevent duplicate connections
    const exists = connections.some(
      conn => conn.from === fromId && conn.to === toId
    )
    if (!exists && fromId !== toId) {
      setConnections([...connections, { from: fromId, to: toId }])
    }
  }

  const deleteConnection = (fromId, toId) => {
    setConnections(connections.filter(
      conn => !(conn.from === fromId && conn.to === toId)
    ))
  }

  return (
    <div className="app">
      <header className="app-header">
        <h1>🤖 LLM Agent Chain Builder</h1>
        <p>Create, connect, and orchestrate AI agents - A no-code platform</p>
      </header>
      
      <Toolbar addNode={addNode} />
      
      <Canvas
        nodes={nodes}
        connections={connections}
        selectedNode={selectedNode}
        setSelectedNode={setSelectedNode}
        updateNode={updateNode}
        deleteNode={deleteNode}
        addConnection={addConnection}
        deleteConnection={deleteConnection}
      />
    </div>
  )
}

export default App
