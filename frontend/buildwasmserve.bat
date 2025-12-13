:: Datoteka za avtomatično izvajanje sledečih komand pri projektu Knjižni sistem
cargo build
wasm-pack build --release --target=web
basic-http-server --addr 127.0.0.1:4001