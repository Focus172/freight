default:
    @just -l

install:
    cp freight-cli/freight ~/.local/bin

build:
    cargo build

publish:
    cargo fmt
    cargo clippy -q -- -D warnings
    cargo test -q

loc:
    find yuma/src/ -name "*.rs" | xargs cat | wc -l
