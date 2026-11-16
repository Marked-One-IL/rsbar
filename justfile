open:
    waypaper --restore
    cargo run

update:
    cargo build
    cargo install --path . --root ~/.local --force

