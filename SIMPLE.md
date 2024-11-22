# Quick Start: Running Mistral.rs with Docker

This guide provides the simplest way to run Mistral.rs on an Azure VM with CUDA support.

## Prerequisites

- Azure VM with CUDA-capable GPU
- Docker installed
- NVIDIA Container Toolkit installed
- At least 8GB of GPU memory
- At least 16GB of system RAM

## One-Line Setup

```bash
docker run --gpus all -p 8000:8000 \
  -v $HOME/.cache/huggingface:/root/.cache/huggingface \
  -e MODEL=TheBloke/Llama-2-3B-GGUF \
  -e CUDA_VISIBLE_DEVICES=0 \
  tribehealth/mistral-rs-cuda:latest
```

## What This Does

1. `--gpus all`: Enables GPU support
2. `-p 8000:8000`: Exposes the OpenAI-compatible API on port 8000
3. `-v $HOME/.cache/huggingface:/root/.cache/huggingface`: Persists downloaded models
4. `-e MODEL=TheBloke/Llama-2-3B-GGUF`: Specifies the model to use
5. `-e CUDA_VISIBLE_DEVICES=0`: Uses the first GPU

## Testing the API

Once running, test with curl:

```bash
curl http://localhost:8000/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{
    "model": "TheBloke/Llama-2-3B-GGUF",
    "messages": [{"role": "user", "content": "Hello, how are you?"}]
  }'
```

## Common Issues

1. If the container fails to start, ensure NVIDIA drivers and Container Toolkit are properly installed
2. If model download fails, check your internet connection and disk space
3. For "out of memory" errors, try a smaller model or increase your GPU memory

For more detailed setup and configuration options, refer to the main documentation.