# LLM Agent Chain Builder

A no-code platform for creating and connecting LLM agent elements, inspired by LangChain workflows.

## Features

- 🎯 **Visual Workflow Builder**: Drag and drop interface to create agent workflows
- 🤖 **Multiple Node Types**: Input, LLM Agent, Tool, Decision, and Output nodes
- 🔗 **Connect Nodes**: Link nodes together like LangChain to create complex agent flows
- ⚙️ **Configure Agents**: Set LLM models, temperature, prompts, and more
- 🎨 **Modern UI**: Beautiful gradient background with smooth animations
- 💾 **Real-time Updates**: Changes are reflected immediately

## Node Types

1. **📝 Input Node**: Starting point for user input
2. **🤖 LLM Agent**: Process data with various AI models (GPT-4, Claude, Llama, etc.)
3. **🔧 Tool Node**: Execute specific tools or functions
4. **🔀 Decision Node**: Branch workflow based on conditions
5. **📤 Output Node**: Final output of the workflow

## How to Use

1. **Add Nodes**: Click the buttons in the toolbar to add different node types
2. **Position Nodes**: Drag nodes around the canvas to organize your workflow
3. **Connect Nodes**: Click on a node's output port (right side), then click on another node's input port (left side) to create a connection
4. **Configure Nodes**: Click on a node to open the configuration panel
5. **Delete Connections**: Click on a connection line to remove it
6. **Delete Nodes**: Use the delete button in the configuration panel

## Getting Started

```bash
# Install dependencies
npm install

# Run development server
npm run dev

# Build for production
npm run build

# Preview production build
npm run preview
```

## Technology Stack

- React 19
- Vite
- CSS3 with modern animations
- SVG for connection lines

## Use Cases

- Create multi-step AI agent workflows
- Build conversational AI systems
- Design data processing pipelines
- Prototype LangChain-style agent chains
- Educational tool for understanding agent orchestration

## License

ISC
