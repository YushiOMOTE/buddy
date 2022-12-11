# Buddy

OpenAI-based LINE bot with memory (experimental)

![](./assets/demo.jpg)

``` mermaid
graph LR
    Line(Line) -->|Webhook| Backend
    subgraph AWS
        Backend("Backend\n(Lambda)") --> Storage("Context Storage\n(DynamoDB)")
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
