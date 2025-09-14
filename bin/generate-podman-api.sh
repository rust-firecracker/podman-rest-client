set -euo pipefail
cargo run -p openapi-client-gen -- --skip-default-client --module --common-dir ./src/api_common ./swagger/swagger-v5.1.0.modified.yaml ./src/v5
cargo fmt
