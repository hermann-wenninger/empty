# LLM Agent Chain Builder - User Guide

## 🚀 Getting Started

### Installation
```bash
cd llm-agent-builder
npm install
```

### Running the Application
```bash
npm run dev
```

The application will open in your browser at `http://localhost:3001`

### Building for Production
```bash
npm run build
npm run preview
```

## 📖 How to Use

### 1. Adding Nodes
Click any of the node type buttons in the toolbar:
- **📝 Input Node**: Create a starting point for your workflow
- **🤖 LLM Agent**: Add an AI agent that processes information
- **🔧 Tool Node**: Add a tool or function execution step
- **🔀 Decision**: Add branching logic
- **📤 Output**: Define the final output format

### 2. Positioning Nodes
- Click and drag any node to reposition it on the canvas
- Nodes can be moved anywhere to organize your workflow

### 3. Connecting Nodes
To create a connection between two nodes:
1. Click on the **output port** (right side) of the first node
2. Click on the **input port** (left side) of the second node
3. A connection line will be drawn between them

### 4. Configuring Nodes
Click on any node to open the configuration panel where you can:

#### For LLM Agent Nodes:
- **Select Model**: Choose from GPT-4, GPT-3.5, Claude-3, Claude-2, Llama-2, or Gemini-Pro
- **Set Temperature**: Adjust from 0 (precise) to 1 (creative)
- **System Prompt**: Define the instructions for the LLM

#### For Other Node Types:
- **Input Node**: Define the initial prompt or question
- **Tool Node**: Configure tool parameters
- **Decision Node**: Define decision criteria
- **Output Node**: Specify output format

### 5. Managing Connections
- **Delete a connection**: Click on any connection line to remove it
- Connections are directional (from output to input)
- You cannot create duplicate connections

### 6. Deleting Nodes
1. Click on a node to select it
2. Click the **🗑️ Delete Node** button in the configuration panel
3. All connections to/from the node will also be removed

## 💡 Tips & Best Practices

### Building Effective Workflows
1. **Start with an Input Node**: Begin your workflow with clear input
2. **Chain LLM Agents**: Connect multiple LLM agents for complex reasoning
3. **Use Decision Nodes**: Add branching logic for conditional workflows
4. **End with Output**: Always conclude with an output node

### Example Workflows

#### Simple Question-Answer
```
Input Node → LLM Agent → Output Node
```

#### Multi-Step Reasoning
```
Input Node → LLM Agent (Analysis) → LLM Agent (Synthesis) → Output Node
```

#### Conditional Processing
```
Input Node → LLM Agent → Decision Node → {Path A, Path B} → Output Node
```

#### Tool-Augmented Agent
```
Input Node → LLM Agent → Tool Node → LLM Agent → Output Node
```

## 🎨 Interface Guide

### Canvas Area
- **Grid Background**: Visual grid for alignment
- **Gradient Background**: Purple gradient for aesthetic appeal
- **Infinite Canvas**: Scroll to see more space

### Node Elements
- **Input Port** (⚪ left): Where connections come in
- **Output Port** (⚪ right): Where connections go out
- **Node Header**: Shows node type and icon
- **Node Body**: Displays node configuration summary

### Configuration Panel
- Appears on the right when a node is selected
- Shows all configurable options for the selected node
- Save changes with the **💾 Save Changes** button
- Scrollable for long content

## 🔧 Advanced Features

### Keyboard Navigation
- Use mouse to drag and position nodes
- Click anywhere on canvas to deselect nodes

### Visual Feedback
- **Hover Effects**: Nodes and ports highlight on hover
- **Selection**: Selected nodes have a glow effect
- **Connection Preview**: See temporary line while connecting

### State Management
- All changes are stored in component state
- Node positions are remembered
- Connections persist until deleted

## 🐛 Troubleshooting

### Node not moving?
- Make sure you're clicking on the node body, not the ports
- The cursor should change to a move cursor

### Connection not creating?
- Make sure you click output port first, then input port
- You cannot connect a node to itself
- Duplicate connections are prevented

### Configuration not saving?
- Click the **💾 Save Changes** button after making changes
- Changes are applied immediately to the node

## 🌟 Use Cases

- **Educational**: Learn about agent orchestration and LangChain concepts
- **Prototyping**: Quickly design and visualize agent workflows
- **Documentation**: Create visual representations of AI agent systems
- **Planning**: Plan complex multi-agent systems before coding
- **Demonstration**: Show how agents can be chained together

## 📚 Related Concepts

This application is inspired by:
- **LangChain**: Framework for chaining LLM operations
- **Node-RED**: Visual programming for IoT
- **Zapier**: No-code automation platform
- **n8n**: Workflow automation tool

## 🔮 Future Enhancements

Potential features for future versions:
- Export/Import workflow definitions (JSON)
- Execute workflows (integrate with actual LLM APIs)
- Save/Load multiple projects
- Collaboration features
- More node types (Memory, Vector Store, etc.)
- Workflow validation and testing
- Performance metrics and monitoring

## 📞 Support

For issues or questions:
1. Check this user guide
2. Review the main README.md
3. Open an issue in the repository

---

**Enjoy building your AI agent workflows! 🚀**
