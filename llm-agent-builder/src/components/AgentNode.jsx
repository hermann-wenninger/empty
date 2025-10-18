import './AgentNode.css'

function AgentNode({ 
  node, 
  selected, 
  onMouseDown, 
  onClick, 
  onOutputClick, 
  onInputClick 
}) {
  const getNodeIcon = (type) => {
    const icons = {
      input: '📝',
      llm: '🤖',
      tool: '🔧',
      decision: '🔀',
      output: '📤'
    }
    return icons[type] || '🔵'
  }

  const getNodeColor = (type) => {
    const colors = {
      input: '#10b981',
      llm: '#667eea',
      tool: '#f59e0b',
      decision: '#ec4899',
      output: '#8b5cf6'
    }
    return colors[type] || '#6b7280'
  }

  const getNodeTitle = (type) => {
    const titles = {
      input: 'Input Node',
      llm: 'LLM Agent',
      tool: 'Tool Node',
      decision: 'Decision Node',
      output: 'Output Node'
    }
    return titles[type] || 'Node'
  }

  return (
    <div
      className={`agent-node ${selected ? 'selected' : ''}`}
      style={{
        left: `${node.x}px`,
        top: `${node.y}px`,
        borderColor: getNodeColor(node.type)
      }}
      onMouseDown={(e) => onMouseDown(e, node)}
      onClick={(e) => {
        e.stopPropagation()
        onClick(node)
      }}
    >
      {/* Input port */}
      <div
        className="node-port input-port"
        onClick={(e) => onInputClick(e, node.id)}
        title="Input port - connect from another node's output"
      />

      <div className="node-header" style={{ background: getNodeColor(node.type) }}>
        <span className="node-icon">{getNodeIcon(node.type)}</span>
        <span className="node-title">{getNodeTitle(node.type)}</span>
      </div>

      <div className="node-body">
        <div className="node-id">ID: {node.id}</div>
        {node.type === 'llm' && (
          <div className="node-info">
            <div className="node-model">Model: {node.model}</div>
            <div className="node-temp">Temp: {node.temperature}</div>
          </div>
        )}
        {node.prompt && (
          <div className="node-prompt" title={node.prompt}>
            {node.prompt.substring(0, 30)}
            {node.prompt.length > 30 ? '...' : ''}
          </div>
        )}
      </div>

      {/* Output port */}
      <div
        className="node-port output-port"
        onClick={(e) => onOutputClick(e, node.id)}
        title="Output port - connect to another node's input"
      />
    </div>
  )
}

export default AgentNode
