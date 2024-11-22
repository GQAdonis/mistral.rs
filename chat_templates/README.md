# Chat Templates in Mistral.rs

This document describes the chat template system used in Mistral.rs, including the available templates, their purposes, and architectural considerations.

## Overview

The chat template system in Mistral.rs uses a JSON-based configuration approach to define how conversations should be formatted for different language models. Each template is stored as a JSON file containing a `chat_template` key with Jinja2-style templating syntax.

## Available Templates

### 1. Default Template (`default.json`)
The base template that provides a robust foundation for general conversation formatting. It includes:
- System message handling
- User/Assistant role alternation validation
- Special token management (BOS/EOS)
- Support for system prompts via `<<SYS>>` tags

### 2. Mistral Template (`mistral.json`)
A streamlined template optimized for Mistral models:
- Strict user/assistant alternation
- Simplified instruction format with `[INST]` tags
- No explicit system message support
- Automatic EOS token handling

### 3. Llama2 Template (`llama2.json`)
Specifically designed for Llama 2 models:
- Compatible with Meta's Llama 2 chat format
- System message integration
- B-chat style instruction markers

### 4. ChatML Template (`chatml.json`)
A minimal implementation of the ChatML format:
- Clean, standardized message structure
- Role-based message separation
- Simplified token handling

### 5. Vicuna Template (`vicuna.json`)
Optimized for Vicuna-based models:
- Custom instruction format
- Specialized system message handling

### 6. Phi Templates (`phi3.json`, `phi3.5.json`)
Templates designed for Microsoft's Phi series:
- Optimized for instruction-following
- Simplified conversation structure

## Template System Design

### Architecture

The template system follows a modular design with several key components:

1. **Template Storage**
   - JSON-based configuration files
   - Easy to add new templates
   - Version control friendly

2. **Template Processing**
   - Jinja2-style syntax for flexibility
   - Support for conditional logic
   - Variable interpolation

3. **Token Management**
   - Automatic handling of special tokens (BOS/EOS)
   - Model-specific token insertion

### Design Strengths

1. **Flexibility**
   - Easy to add new templates
   - Support for complex formatting logic
   - Model-agnostic design

2. **Maintainability**
   - Clear separation of concerns
   - JSON format for easy editing
   - Self-contained template files

3. **Validation**
   - Built-in conversation structure validation
   - Role alternation checking
   - Error handling for invalid states

### Areas for Improvement

1. **Documentation**
   - More detailed documentation for each template
   - Examples of usage for each format
   - Clear migration guides between templates

2. **Template Validation**
   - Schema validation for template files
   - Runtime validation of template syntax
   - Automated testing of template rendering

3. **Extensibility**
   - Plugin system for custom template processors
   - Template inheritance/composition
   - Dynamic template selection based on model

4. **Performance**
   - Template precompilation
   - Caching of frequently used templates
   - Optimized token handling

5. **Developer Experience**
   - Template development tools
   - Debug mode for template rendering
   - Interactive template testing

## Best Practices

1. **Template Selection**
   - Use model-specific templates when available
   - Fall back to default template for unknown models
   - Test template compatibility before deployment

2. **System Messages**
   - Keep system messages concise
   - Use consistent formatting
   - Validate system message handling

3. **Token Management**
   - Be mindful of token limits
   - Handle special tokens consistently
   - Test with different token configurations

## Contributing

When adding new templates:
1. Follow the existing JSON structure
2. Include comprehensive documentation
3. Add validation for conversation structure
4. Test with target models
5. Update this README with template details

## Future Considerations

1. **Template Registry**
   - Central repository for community templates
   - Version management for templates
   - Compatibility metadata

2. **Advanced Features**
   - Multi-modal support
   - Function calling templates
   - Stream processing optimization

3. **Tools and Utilities**
   - Template validation tools
   - Development environment
   - Performance benchmarking
