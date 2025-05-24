# MathNet
## Setup development environment
### Pre-commit hooks:
```bash
cp pre-commit.sh .git/hooks/pre-commit
chmod +x .git/hooks/pre-commit
```
## Start application
### Instal dependencies
```bash
cd frontend
cargo install wasm-pack
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli
cargo install trunk
```
### Build frontend
```bash
cd frontend
trunk build --release --dist ../mathnet-server/web-folder
```

### Start database
```bash
docker run --rm --name pg -p 5432:5432 -e POSTGRES_PASSWORD=welcome postgres:17
```

### Start server
```bash
cd mathnet-server
cargo run
```

Go to [`127.0.0.1:8080`](http://127.0.0.1:8080)