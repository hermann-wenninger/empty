import './Toolbar.css'

function Toolbar({ addNode }) {
  const nodeTypes = [
    { type: 'input', label: '📝 Input Node', description: 'Start with user input' },
    { type: 'llm', label: '🤖 LLM Agent', description: 'Process with AI' },
    { type: 'tool', label: '🔧 Tool Node', description: 'Execute a tool' },
    { type: 'decision', label: '🔀 Decision', description: 'Branch logic' },
    { type: 'output', label: '📤 Output', description: 'Final result' },
  ]

  return (
    <div className="toolbar">
      <div className="toolbar-title">Add Nodes:</div>
      <div className="toolbar-buttons">
        {nodeTypes.map(nodeType => (
          <button
            key={nodeType.type}
            className="toolbar-button"
            onClick={() => addNode(nodeType.type)}
            title={nodeType.description}
          >
            {nodeType.label}
          </button>
        ))}
      </div>
      <div className="toolbar-hint">
        💡 Click on nodes to configure them. Drag from output port to input port to connect.
      </div>
    </div>
  )
}

export default Toolbar
