import { useRef, useState } from 'react'
import AgentNode from './AgentNode'
import Connection from './Connection'
import NodeConfig from './NodeConfig'
import './Canvas.css'

function Canvas({ 
  nodes, 
  connections, 
  selectedNode,
  setSelectedNode,
  updateNode, 
  deleteNode,
  addConnection,
  deleteConnection
}) {
  const canvasRef = useRef(null)
  const [draggingNode, setDraggingNode] = useState(null)
  const [dragOffset, setDragOffset] = useState({ x: 0, y: 0 })
  const [connectingFrom, setConnectingFrom] = useState(null)
  const [mousePos, setMousePos] = useState({ x: 0, y: 0 })

  const handleCanvasClick = (e) => {
    if (e.target === canvasRef.current) {
      setSelectedNode(null)
      setConnectingFrom(null)
    }
  }

  const handleNodeMouseDown = (e, node) => {
    e.stopPropagation()
    const rect = canvasRef.current.getBoundingClientRect()
    setDragOffset({
      x: e.clientX - rect.left - node.x,
      y: e.clientY - rect.top - node.y
    })
    setDraggingNode(node.id)
  }

  const handleMouseMove = (e) => {
    if (draggingNode !== null) {
      const rect = canvasRef.current.getBoundingClientRect()
      const newX = e.clientX - rect.left - dragOffset.x
      const newY = e.clientY - rect.top - dragOffset.y
      updateNode(draggingNode, { x: newX, y: newY })
    }
    
    if (connectingFrom !== null) {
      const rect = canvasRef.current.getBoundingClientRect()
      setMousePos({
        x: e.clientX - rect.left,
        y: e.clientY - rect.top
      })
    }
  }

  const handleMouseUp = () => {
    setDraggingNode(null)
  }

  const handleNodeClick = (node) => {
    setSelectedNode(node)
  }

  const handleOutputClick = (e, nodeId) => {
    e.stopPropagation()
    if (connectingFrom === null) {
      setConnectingFrom(nodeId)
    } else if (connectingFrom !== nodeId) {
      addConnection(connectingFrom, nodeId)
      setConnectingFrom(null)
    }
  }

  const handleInputClick = (e, nodeId) => {
    e.stopPropagation()
    if (connectingFrom !== null && connectingFrom !== nodeId) {
      addConnection(connectingFrom, nodeId)
      setConnectingFrom(null)
    }
  }

  return (
    <div className="canvas-container">
      <div
        ref={canvasRef}
        className="canvas"
        onMouseMove={handleMouseMove}
        onMouseUp={handleMouseUp}
        onClick={handleCanvasClick}
      >
        {/* Render connections */}
        <svg className="connections-layer">
          {connections.map((conn, idx) => {
            const fromNode = nodes.find(n => n.id === conn.from)
            const toNode = nodes.find(n => n.id === conn.to)
            if (fromNode && toNode) {
              return (
                <Connection
                  key={idx}
                  from={{ x: fromNode.x + 150, y: fromNode.y + 75 }}
                  to={{ x: toNode.x, y: toNode.y + 75 }}
                  onDelete={() => deleteConnection(conn.from, conn.to)}
                />
              )
            }
            return null
          })}
          
          {/* Draw temporary connection line while connecting */}
          {connectingFrom !== null && (
            <Connection
              from={{
                x: nodes.find(n => n.id === connectingFrom).x + 150,
                y: nodes.find(n => n.id === connectingFrom).y + 75
              }}
              to={mousePos}
              temporary
            />
          )}
        </svg>

        {/* Render nodes */}
        {nodes.map(node => (
          <AgentNode
            key={node.id}
            node={node}
            selected={selectedNode?.id === node.id}
            onMouseDown={handleNodeMouseDown}
            onClick={handleNodeClick}
            onOutputClick={handleOutputClick}
            onInputClick={handleInputClick}
          />
        ))}

        {nodes.length === 0 && (
          <div className="canvas-empty">
            <div className="empty-message">
              <h2>👆 Click a button above to add your first agent node</h2>
              <p>Build your AI agent workflow by connecting nodes together</p>
            </div>
          </div>
        )}
      </div>

      {selectedNode && (
        <NodeConfig
          node={selectedNode}
          onUpdate={(updates) => updateNode(selectedNode.id, updates)}
          onDelete={() => deleteNode(selectedNode.id)}
          onClose={() => setSelectedNode(null)}
        />
      )}
    </div>
  )
}

export default Canvas
