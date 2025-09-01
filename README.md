# 測試 Rust 使用 musl 編譯如何影響網路連線

## Build and Run with musl

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
    Error: reqwest::Error { kind: Request, url: Url { scheme: "https", cannot_be_a_base: false, username: "", password: None, host: Some(Domain("your-resource-name.openai.azure.com")), port: None, path: "/openai/deployments/gpt-4.1/chat/completions", query: Some("api-version=2025-01-01-preview"), fragment: None }, source: hyper::Error(Connect, ConnectError("dns error", Custom { kind: Uncategorized, error: "failed to lookup address information: Name does not resolve" })) }
    ```

## Build and Run with gnu

1. 設定 `AZURE_OPENAI_API_KEY` 與 `AZURE_BASE_URL` 環境變數

    AZURE_BASE_URL = https://your-resource-name.openai.azure.com/openai

2. 安裝、建置、執行

    ```sh
    # Build
    cargo build --target x86_64-unknown-linux-gnu

    # Run
    ./target/x86_64-unknown-linux-gnu/debug/test-azure-connection
    ```

    ```log
    狀態碼: 200 OK
    回應: {"choices":[{"content_filter_results":{"hate":{"filtered":false,"severity":"safe"},"self_harm":{"filtered":false,"severity":"safe"},"sexual":{"filtered":false,"severity":"safe"},"violence":{"filtered":false,"severity":"safe"}},"finish_reason":"length","index":0,"logprobs":null,"message":{"annotations":[],"content":"Hello! It looks like","refusal":null,"role":"assistant"}}],"created":1756750165,"id":"chatcmpl-CB3G19mgoZi8x3popLBXCFneXwo9N","model":"gpt-4.1-2025-04-14","object":"chat.completion","prompt_filter_results":[{"prompt_index":0,"content_filter_results":{"hate":{"filtered":false,"severity":"safe"},"jailbreak":{"filtered":false,"detected":false},"self_harm":{"filtered":false,"severity":"safe"},"sexual":{"filtered":false,"severity":"safe"},"violence":{"filtered":false,"severity":"safe"}}}],"system_fingerprint":"fp_c79ab13e31","usage":{"completion_tokens":5,"completion_tokens_details":{"accepted_prediction_tokens":0,"audio_tokens":0,"reasoning_tokens":0,"rejected_prediction_tokens":0},"prompt_tokens":8,"prompt_tokens_details":{"audio_tokens":0,"cached_tokens":0},"total_tokens":13}}
    ```

## 相關連結

- [Considerations for Using the Azure OpenAI Services Provider #1552](https://github.com/openai/codex/issues/1552)
