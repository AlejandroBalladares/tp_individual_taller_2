# tp_individual_taller_2

Ejecutar sevidor: cargo run --bin server -- 127.0.0.1:8080
Ejecutar cliente: cargo run --bin client -- 127.0.0.1:8080 ./data/nombre_archivo.txt

Entregable: zip -r entrega.zip Cargo.toml Cargo.lock src/ tests/
Comando de clippy util: cargo clippy --all-targets --all-features -- -D warnings