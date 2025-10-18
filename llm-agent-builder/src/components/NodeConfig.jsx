import { useState } from 'react'
import './NodeConfig.css'

function NodeConfig({ node, onUpdate, onDelete, onClose }) {
  const [prompt, setPrompt] = useState(node.prompt || '')
  const [model, setModel] = useState(node.model || 'GPT-4')
  const [temperature, setTemperature] = useState(node.temperature || 0.7)

  const handleSave = () => {
    onUpdate({ prompt, model, temperature })
  }

  const models = ['GPT-4', 'GPT-3.5', 'Claude-3', 'Claude-2', 'Llama-2', 'Gemini-Pro']

  return (
    <div className="node-config">
      <div className="config-header">
        <h3>⚙️ Configure Node #{node.id}</h3>
        <button className="close-btn" onClick={onClose}>✕</button>
      </div>

      <div className="config-body">
        <div className="config-section">
          <label className="config-label">Node Type:</label>
          <div className="config-value">{node.type}</div>
        </div>

        {node.type === 'llm' && (
          <>
            <div className="config-section">
              <label className="config-label">
                LLM Model:
              </label>
              <select 
                className="config-select"
                value={model} 
                onChange={(e) => setModel(e.target.value)}
              >
                {models.map(m => (
                  <option key={m} value={m}>{m}</option>
                ))}
              </select>
            </div>

            <div className="config-section">
              <label className="config-label">
                Temperature: {temperature}
              </label>
              <input
                type="range"
                min="0"
                max="1"
                step="0.1"
                value={temperature}
                onChange={(e) => setTemperature(parseFloat(e.target.value))}
                className="config-slider"
              />
              <div className="slider-labels">
                <span>Precise</span>
                <span>Creative</span>
              </div>
            </div>
          </>
        )}

        <div className="config-section">
          <label className="config-label">
            {node.type === 'input' ? 'Input Prompt:' : 
             node.type === 'llm' ? 'System Prompt:' :
             node.type === 'tool' ? 'Tool Configuration:' :
             node.type === 'decision' ? 'Decision Logic:' :
             'Output Format:'}
          </label>
          <textarea
            className="config-textarea"
            value={prompt}
            onChange={(e) => setPrompt(e.target.value)}
            placeholder={
              node.type === 'input' ? 'Enter the initial prompt or question...' :
              node.type === 'llm' ? 'Enter system prompt for the LLM...' :
              node.type === 'tool' ? 'Configure tool parameters...' :
              node.type === 'decision' ? 'Define decision criteria...' :
              'Define output format...'
            }
            rows={6}
          />
        </div>

        <div className="config-actions">
          <button className="save-btn" onClick={handleSave}>
            💾 Save Changes
          </button>
          <button className="delete-btn" onClick={onDelete}>
            🗑️ Delete Node
          </button>
        </div>

        <div className="config-info">
          <p><strong>💡 Tips:</strong></p>
          <ul>
            <li>Drag nodes to reposition them</li>
            <li>Click output port and then input port to connect nodes</li>
            <li>Click on a connection to delete it</li>
            <li>Chain nodes together like LangChain workflows</li>
          </ul>
        </div>
      </div>
    </div>
  )
}

export default NodeConfig
