# 測試 Rust 使用 musl 編譯如何影響網路連線

## Build and Run

1. 設定 `AZURE_OPENAI_API_KEY` 與 `AZURE_BASE_URL` 環境變數

    AZURE_BASE_URL = https://your-resource-name.openai.azure.com/openai

2. 安裝、建置、執行

    ```sh
    # Install musl-tools
    sudo apt-get update
    sudo apt-get -y install musl-tools

    # Build
    cargo build --target x86_64-unknown-linux-musl

    # Run
    ./target/x86_64-unknown-linux-musl/debug/test-azure-connection
    ```

    ```log
    狀態碼: 200 OK
    回應: {"choices":[{"content_filter_results":{"hate":{"filtered":false,"severity":"safe"},"self_harm":{"filtered":false,"severity":"safe"},"sexual":{"filtered":false,"severity":"safe"},"violence":{"filtered":false,"severity":"safe"}},"finish_reason":"length","index":0,"logprobs":null,"message":{"annotations":[],"content":"Hello! It looks like","refusal":null,"role":"assistant"}}],"created":1756749680,"id":"chatcmpl-CB38CGNFrrplmi6y7Sw64xcY6QqIm","model":"gpt-4.1-2025-04-14","object":"chat.completion","prompt_filter_results":[{"prompt_index":0,"content_filter_results":{"hate":{"filtered":false,"severity":"safe"},"jailbreak":{"filtered":false,"detected":false},"self_harm":{"filtered":false,"severity":"safe"},"sexual":{"filtered":false,"severity":"safe"},"violence":{"filtered":false,"severity":"safe"}}}],"system_fingerprint":"fp_c79ab13e31","usage":{"completion_tokens":5,"completion_tokens_details":{"accepted_prediction_tokens":0,"audio_tokens":0,"reasoning_tokens":0,"rejected_prediction_tokens":0},"prompt_tokens":8,"prompt_tokens_details":{"audio_tokens":0,"cached_tokens":0},"total_tokens":13}}
    ```