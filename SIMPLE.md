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
docker run --gpus all -p 8000:8000 \
  -v $HOME/.cache/huggingface:/root/.cache/huggingface \
  -e HUGGING_FACE_HUB_TOKEN=your_token_here \
  -e TOK_MODEL_ID=mistralai/Mistral-7B-Instruct-v0.1 \
  -e QUANTIZED_MODEL_ID=TheBloke/Mistral-7B-Instruct-v0.1-GGUF \
  -e QUANTIZED_FILENAME=mistral-7b-instruct-v0.1.Q4_K_M.gguf \
  tribehealth/mistral-rs-cuda:latest \
  gguf \
  --port 8000
```

## What Each Flag Does

- `--gpus all`: Enables CUDA GPU support
- `-p 8000:8000`: Exposes the OpenAI-compatible API on port 8000
- `-v $HOME/.cache/huggingface:/root/.cache/huggingface`: Persists downloaded models
- `-e TOK_MODEL_ID`: The original model ID for the tokenizer
- `-e QUANTIZED_MODEL_ID`: The HuggingFace repository containing the GGUF model
- `-e QUANTIZED_FILENAME`: The specific GGUF model file to use
- `-e HUGGING_FACE_HUB_TOKEN`: Your HuggingFace token for downloading models
- `gguf`: Subcommand to specify GGUF model type
- `--port 8000`: The port to run the server on
- `-e CUDA_VISIBLE_DEVICES=0`: Uses the first GPU

## Finding GGUF Models

1. Visit [TheBloke's HuggingFace profile](https://huggingface.co/TheBloke) to find GGUF versions of popular models
2. Choose a model (e.g., Mistral-7B-Instruct-v0.1-GGUF)
3. Set the environment variables:
   - `TOK_MODEL_ID`: Use the original model ID (e.g., mistralai/Mistral-7B-Instruct-v0.1)
   - `QUANTIZED_MODEL_ID`: Use TheBloke's GGUF version (e.g., TheBloke/Mistral-7B-Instruct-v0.1-GGUF)
   - `QUANTIZED_FILENAME`: Use the specific GGUF file (e.g., mistral-7b-instruct-v0.1.Q4_K_M.gguf)

## Test the API

Once running, test with curl:

```bash
curl http://localhost:8000/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{
    "model": "llama2",
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
   - Ensure you have enough disk space
   - Check internet connectivity

4. For "out of memory" errors:
   - Try a smaller model
   - Reduce batch size
   - Use a GPU with more memory

For more detailed setup and configuration options, refer to the main documentation.