default:
    @just -l

install:
    cp freight-cli/freight ~/.local/bin

build:
    cargo build -q

publish:
    cargo fmt
    cargo clippy -q -- -D warnings
    cargo test -q

loc:
    find src/ -name "*.rs" | xargs cat | wc -l
