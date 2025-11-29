# Advent of Code – Rust Solutions

Personal solutions for the Advent of Code challenges, organized by year and executed through a single runner binary.

---

## Usage

The binary accepts two parameters:

| Flag           | Meaning                                | Example   |
| -------------- | -------------------------------------- | --------- |
| `--year`, `-y` | Target Advent of Code year             | `-y 2024` |
| `--day`, `-d`  | Target day (1–25). Accepts `1` or `01` | `-d 1`    |
| `--show-time`, `-t`| Show runtime of the solution           | `-t`        |

Internally, days are formatted as two digits, so both `1` and `01` work.

---

## Running with Cargo

Arguments to your binary must come after `--`:

```bash
cargo run -- -y 2024 -d 1
```

### Example

Run **Day 01 – Year 2024**:

```bash
cargo run -- -y 2024 -d 1
```

---

## Running the compiled binary

Build in release mode:

```bash
cargo build --release
```

Execute the binary:

```bash
./target/release/advent-of-code -y 2024 -d 1
```

---

## Input files

Puzzle inputs must follow this layout:

```
inputs/y{year}/{day}.txt
```

Where:

| Component | Description       | Example     |
|-----------| ----------------- |-------------|
| `{year}`  | Four-digit year   | `y2024`     |
| `{day}`   | Day in one or two digits | `01` or `1` |

Example:

```
inputs/y2024/01.txt
```

or

```
inputs/y2024/1.txt
```

---