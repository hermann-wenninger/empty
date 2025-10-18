import { useState } from 'react'
import './Connection.css'

function Connection({ from, to, onDelete, temporary }) {
  const [hovered, setHovered] = useState(false)

  // Calculate control points for a smooth curve
  const dx = to.x - from.x
  const dy = to.y - from.y
  
  // Control points for bezier curve
  const cp1x = from.x + dx * 0.5
  const cp1y = from.y
  const cp2x = from.x + dx * 0.5
  const cp2y = to.y

  const path = `M ${from.x} ${from.y} C ${cp1x} ${cp1y}, ${cp2x} ${cp2y}, ${to.x} ${to.y}`

  return (
    <g 
      className={`connection ${temporary ? 'temporary' : ''}`}
      onMouseEnter={() => setHovered(true)}
      onMouseLeave={() => setHovered(false)}
    >
      {/* Invisible wider path for easier clicking */}
      {!temporary && (
        <path
          d={path}
          fill="none"
          stroke="transparent"
          strokeWidth="20"
          style={{ pointerEvents: 'stroke', cursor: 'pointer' }}
          onClick={(e) => {
            e.stopPropagation()
            if (onDelete) onDelete()
          }}
        />
      )}
      
      {/* Visible path */}
      <path
        d={path}
        fill="none"
        stroke={temporary ? '#94a3b8' : hovered ? '#ef4444' : '#667eea'}
        strokeWidth={hovered ? '4' : '3'}
        strokeDasharray={temporary ? '5,5' : 'none'}
        markerEnd={temporary ? '' : 'url(#arrowhead)'}
        style={{ transition: 'all 0.2s' }}
      />

      {/* Arrow marker definition */}
      {!temporary && (
        <defs>
          <marker
            id="arrowhead"
            markerWidth="10"
            markerHeight="10"
            refX="9"
            refY="3"
            orient="auto"
          >
            <polygon
              points="0 0, 10 3, 0 6"
              fill={hovered ? '#ef4444' : '#667eea'}
              style={{ transition: 'all 0.2s' }}
            />
          </marker>
        </defs>
      )}

      {hovered && !temporary && (
        <text
          x={(from.x + to.x) / 2}
          y={(from.y + to.y) / 2 - 10}
          fill="#ef4444"
          fontSize="12"
          textAnchor="middle"
          style={{ pointerEvents: 'none' }}
        >
          Click to delete
        </text>
      )}
    </g>
  )
}

export default Connection
