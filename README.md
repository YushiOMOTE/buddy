# Buddy

OpenAI-based LINE bot with memory (experimental)

<img src="./assets/demo.jpg" width="300">

``` mermaid
graph LR
    Line(Line) -->|Webhook| Backend
    subgraph AWS
        Backend("Backend (Lambda)") --> Storage("Context Storage (DynamoDB)")
    end
    Backend --> OpenAI("Open AI")
```

## Build and deploy

### Setup

``` sh
cargo install cargo-lambda
```

### Build

``` sh
cargo lambda build --release
```

### Deploy

``` sh
cargo lambda deploy --iam-role <lambda execution role>
```
