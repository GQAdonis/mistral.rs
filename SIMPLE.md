# Quick Start Guide for Mistral.rs on Azure VM

This guide provides the simplest way to run Mistral.rs with CUDA support using a GGUF model.

## Prerequisites

- Azure VM with CUDA-capable GPU
- Docker installed
- NVIDIA Container Toolkit installed
- At least 8GB of GPU memory
- At least 16GB of system RAM

## One-Line Setup

```bash
docker run --gpus all -p 8000:8000 --name mistralrs \
  -v $HOME/.cache/huggingface:/root/.cache/huggingface \
  -e HUGGING_FACE_HUB_TOKEN=$HUGGING_FACE_HUB_TOKEN \
  -e TOK_MODEL_ID=meta-llama/Llama-3.2-3b-instruct \
  tribehealth/mistral-rs-cuda:latest \
  gguf --quantized-model-id QuantFactory/Llama-3.2-3B-Instruct-GGUF \
  --quantized-filename llama-3.2-3b-instruct.Q4_K_M.gguf
```

## What Each Flag Does

### Mandatory Environment Variables
- `-e TOK_MODEL_ID`: The original model ID for the tokenizer (e.g., mistralai/Mistral-7B-Instruct-v0.1)
- `-e QUANTIZED_MODEL_ID`: The HuggingFace repository containing the GGUF model (e.g., TheBloke/Mistral-7B-Instruct-v0.1-GGUF)
- `-e QUANTIZED_FILENAME`: The specific GGUF model file to use (e.g., mistral-7b-instruct-v0.1.Q4_K_M.gguf)

### Optional Environment Variables
- `-e HUGGING_FACE_HUB_TOKEN`: Your HuggingFace token for downloading gated models (required only if the model is not public)
- `-e CUDA_VISIBLE_DEVICES`: Specify which GPU to use (default: 0)

### Other Flags
- `--gpus all`: Enables CUDA GPU support
- `-p 8000:8000`: Exposes the OpenAI-compatible API on port 8000
- `-v $HOME/.cache/huggingface:/root/.cache/huggingface`: Persists downloaded models
- `gguf`: Subcommand to specify GGUF model type

## Finding GGUF Models

1. Visit [TheBloke's HuggingFace profile](https://huggingface.co/TheBloke) to find GGUF versions of popular models
2. Choose a model (e.g., Mistral-7B-Instruct-v0.1-GGUF)
3. Set the environment variables:
   - `TOK_MODEL_ID`: Use the original model ID (e.g., mistralai/Mistral-7B-Instruct-v0.1)
   - `QUANTIZED_MODEL_ID`: Use TheBloke's GGUF version (e.g., TheBloke/Mistral-7B-Instruct-v0.1-GGUF)
   - `QUANTIZED_FILENAME`: Use the specific GGUF file (e.g., mistral-7b-instruct-v0.1.Q4_K_M.gguf)

## List Available Models

To see which models are available on your server, use the OpenAI-compatible models endpoint:

```bash
curl http://localhost:8000/v1/models
```

The response will show all available models and their capabilities:

```json
{
  "object": "list",
  "data": [
    {
      "id": "llama-3.2-3b-instruct",
      "object": "model",
      "owned_by": "mistral.rs",
      "permission": []
    }
  ]
}
```

## Test the API

Once running, test with curl:

```bash
curl http://localhost:8000/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{
    "model": "llama-3.2-3b-instruct",
    "messages": [{"role": "user", "content": "Hello, how are you?"}],
    "temperature": 0.7,
    "max_tokens": 100
  }'
```

## Troubleshooting

1. If you see CUDA errors, ensure the NVIDIA Container Toolkit is properly installed:
   ```bash
   sudo apt-get install -y nvidia-container-toolkit
   sudo systemctl restart docker
   ```

2. Verify CUDA is available:
   ```bash
   docker run --gpus all nvidia/cuda:12.3.2-base-ubuntu22.04 nvidia-smi
   ```

3. If model download fails:
   - Check your HuggingFace token is correct
   - Set up your token using one of these methods:
     ```bash
     # Method 1: Using cache file (recommended)
     mkdir -p ~/.cache/huggingface
     echo "your_huggingface_token" > ~/.cache/huggingface/token

     # Method 2: Using environment variable
     export HUGGING_FACE_HUB_TOKEN=your_token_here

     # Method 3: Using --token-source parameter
     --token-source literal:your_token_here
     ```
   - Ensure you have enough disk space
   - Check internet connectivity

4. For "out of memory" errors:
   - Try a smaller model
   - Reduce batch size
   - Use a GPU with more memory

For more detailed setup and configuration options, refer to the main documentation.