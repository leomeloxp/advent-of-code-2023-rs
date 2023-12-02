# Use `just work day-01 part1` to work on the specific binary for a specific day's problems
work day part:
    cd {{day}} && cargo watch -c -q -x "check -p {{day}}" -s "cargo run --bin {{part}}" -s "cargo test -p {{day}} --bin {{part}}"
create day:
    cargo generate --path ./daily-template --name {{day}}