# ReAct Implementation Guide

This document outlines how to implement ReAct (Reasoning and Acting) in Mistral.rs using the template system. ReAct combines language model reasoning with action execution to solve complex tasks through an iterative process of thought, action, and observation.

## Overview

ReAct follows a structured approach:
1. **Thought**: Reasoning about the current state and deciding what to do
2. **Action**: Executing a chosen action from available tools
3. **Observation**: Processing the result of the action
4. **Reflection**: Evaluating progress and planning next steps

## Template Implementation

### 1. Basic ReAct Template

```json
{
    "chat_template": "{{ bos_token }}{% for message in messages %}{% if message['role'] == 'system' %}<<SYS>>
You are an AI assistant that follows the ReAct framework to solve tasks:
1. Think: Analyze the situation and plan your approach
2. Act: Choose and execute an appropriate action
3. Observe: Process the results of your action
4. Reflect: Evaluate progress and adjust your plan

Available Actions:
- search(query): Search for information
- calculate(expression): Perform calculations
- lookup(term): Look up definitions
- verify(statement): Fact-check a statement
<</SYS>>{% elif message['role'] == 'user' %}[INST] {{ message['content'] }} [/INST]{% elif message['role'] == 'assistant' %}

Thought: Let me analyze this step by step...
{{ message['content'] }}

Action: [Action name and parameters]

Observation: [Result of action]

Reflection: Based on this result...{{ eos_token }}{% endif %}{% endfor %}"
}
```

### 2. Advanced ReAct Template with Tool Integration

```json
{
    "chat_template": "{{ bos_token }}{% for message in messages %}{% if message['role'] == 'system' %}<<SYS>>
You are an AI assistant with ReAct capabilities and access to the following tools:

TOOLS:
{
  "search": "Search for information online",
  "calculate": "Perform mathematical calculations",
  "code": "Write and execute code",
  "analyze": "Analyze data or text",
  "verify": "Fact-check information"
}

For each step:
1. THOUGHT: Reason about the current state
2. ACTION: Use available tools
3. OBSERVATION: Process results
4. REFLECTION: Evaluate and plan next steps
<</SYS>>{% elif message['role'] == 'user' %}[INST] {{ message['content'] }} [/INST]{% elif message['role'] == 'assistant' %}

THOUGHT: {{ message['content'] }}

ACTION: {
  "tool": "[tool_name]",
  "params": {
    "param1": "value1",
    "param2": "value2"
  }
}

OBSERVATION: [Tool output]

REFLECTION: [Analysis of results and next steps]{{ eos_token }}{% endif %}{% endfor %}"
}
```

## Usage Example: Complex Problem Solving

### Scenario: Research and Analysis Task

```python
messages = [
    {
        "role": "system",
        "content": "You are a research assistant using ReAct to gather and analyze information."
    },
    {
        "role": "user",
        "content": "Research the impact of AI on job markets and provide a summary with data."
    }
]
```

### Expected Response Format

```text
THOUGHT: To answer this question comprehensively, I need to:
1. Search for recent studies on AI and employment
2. Gather statistical data
3. Analyze trends and patterns
4. Verify information from multiple sources

ACTION: {
  "tool": "search",
  "params": {
    "query": "recent studies AI impact job market statistics 2023"
  }
}

OBSERVATION: Found several studies including McKinsey report showing 30% of work hours automated by 2030...

REFLECTION: The search provided good baseline data. Need to:
1. Verify statistics
2. Look for contrary evidence
3. Analyze specific industry impacts

ACTION: {
  "tool": "analyze",
  "params": {
    "data": "employment_statistics.json",
    "type": "trend_analysis"
  }
}

OBSERVATION: Analysis shows varying impact across sectors...

[Continue with iterative ReAct process]
```

## Implementation Guide

### 1. Template Setup

Create a ReAct template file:

```json
{
    "chat_template": "{{ bos_token }}{% for message in messages %}{% if message['role'] == 'system' %}<<SYS>>
You are an AI assistant using ReAct framework.
Available tools:
{{ tools | json_encode }}

Process:
1. Think about the task
2. Choose an action
3. Observe results
4. Reflect and plan next steps
<</SYS>>{% elif message['role'] == 'user' %}[INST] {{ message['content'] }} [/INST]{% elif message['role'] == 'assistant' %}

THOUGHT: {{ message['thought'] }}
ACTION: {{ message['action'] | json_encode }}
OBSERVATION: {{ message['observation'] }}
REFLECTION: {{ message['reflection'] }}{{ eos_token }}{% endif %}{% endfor %}"
}
```

### 2. Code Integration

```rust
use mistralrs::{Config, Model, Tool, Action};

async fn solve_with_react(query: &str, tools: Vec<Tool>) -> Result<String> {
    let config = Config::new()
        .with_template_path("path/to/templates/react.json")
        .with_tools(tools)
        .build()?;
    
    let model = Model::new(config)?;
    
    let messages = vec![
        Message::system("ReAct-enabled assistant with tools."),
        Message::user(query),
    ];
    
    model.chat_with_react(&messages).await
}

// Tool implementation example
#[derive(Tool)]
struct SearchTool {
    async fn execute(&self, params: Value) -> Result<String> {
        // Implementation
    }
}
```

### 3. Deployment Configuration

```yaml
version: '3.8'
services:
  mistral:
    image: tribehealth/mistral-rs-cuda:latest
    volumes:
      - ./templates:/app/templates
      - ./tools:/app/tools
    environment:
      - ENABLE_REACT=true
      - MAX_ITERATIONS=5
      - TOOL_TIMEOUT=30
    deploy:
      resources:
        reservations:
          devices:
            - driver: nvidia
              count: 1
              capabilities: [gpu]
```

## Best Practices

1. **Tool Design**
   - Clear, atomic functionality
   - Robust error handling
   - Consistent input/output formats
   - Comprehensive documentation

2. **Iteration Control**
   - Maximum iteration limits
   - Timeout mechanisms
   - Progress tracking
   - Cycle detection

3. **Error Handling**
   - Tool failure recovery
   - Graceful degradation
   - Alternative action paths
   - User feedback loops

4. **Performance Optimization**
   - Parallel tool execution
   - Result caching
   - Resource management
   - Load balancing

## Advanced Features

### 1. Multi-Step Planning

```json
{
    "chat_template": "{{ bos_token }}{% for message in messages %}{% if message['role'] == 'user' %}[INST] {{ message['content'] }} [/INST]{% elif message['role'] == 'assistant' %}
PLAN:
1. [Step 1 description]
2. [Step 2 description]
3. [Step 3 description]

EXECUTION:
{% for step in plan %}
Step {{ loop.index }}:
THOUGHT: {{ step.thought }}
ACTION: {{ step.action }}
OBSERVATION: {{ step.observation }}
REFLECTION: {{ step.reflection }}
{% endfor %}{{ eos_token }}{% endif %}{% endfor %}"
}
```

### 2. Tool Composition

```json
{
    "chat_template": "{{ bos_token }}{% for message in messages %}{% if message['role'] == 'user' %}[INST] {{ message['content'] }} [/INST]{% elif message['role'] == 'assistant' %}
WORKFLOW:
{
  "steps": [
    {
      "tool": "tool1",
      "params": {},
      "next": {
        "success": "tool2",
        "error": "fallback1"
      }
    },
    {
      "tool": "tool2",
      "params": {}
    }
  ]
}{{ eos_token }}{% endif %}{% endfor %}"
}
```

## Monitoring and Evaluation

1. **Performance Metrics**
   - Success rate
   - Iteration count
   - Tool usage statistics
   - Response time
   - Error rates

2. **Quality Assurance**
   - Action validation
   - Result verification
   - Logic checks
   - User feedback

## Future Enhancements

1. **Advanced Capabilities**
   - Dynamic tool discovery
   - Adaptive planning
   - Multi-agent collaboration
   - Learning from experience

2. **Integration Features**
   - External API integration
   - Custom tool development
   - Workflow automation
   - Event handling

3. **Development Tools**
   - Tool testing framework
   - Plan visualizer
   - Debug console
   - Performance profiler

## Research Directions

1. **Theoretical Framework**
   - Action selection strategies
   - Planning algorithms
   - Tool composition patterns
   - Error recovery models

2. **Practical Applications**
   - Automated research
   - Data analysis
   - Process automation
   - Decision support

3. **Safety and Ethics**
   - Action validation
   - Permission systems
   - Audit trails
   - Ethical guidelines

## Tool Development Guide

1. **Tool Structure**
   - Input/output schema
   - Error handling
   - Resource management
   - Documentation

2. **Integration Points**
   - API endpoints
   - Authentication
   - Rate limiting
   - Monitoring

3. **Testing Strategy**
   - Unit tests
   - Integration tests
   - Load testing
   - Error scenarios