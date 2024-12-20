## PAN

**Bread Recipe Generator**

A command-line tool to generate bread recipes based on your desired parameters.

### Installation

**Using Cargo:**
1. Ensure you have Rust installed.
2. Clone this repository.
3. Run `cargo install --path .`

**Manual Installation:**
1. Compile the project using your preferred Rust toolchain.
2. Place the compiled binary in your system's PATH.

### Usage

```bash
pn [OPTIONS]
```

**Options:**

| Option | Description | Default |
|---|---|---|
| `-m, --mass` | Total flour mass in grams | 600 |
| `-d, --hydration` | Hydration percentage | 70 |
| `-f, --flour` | Flour type and percentage, e.g., `White:100` | `White:100` |
| `-s, --salt-percentage` | Salt percentage of flour | 2 |
| `-p, --preferment` | Preferment name, ratio of flour, and hydration, e.g., `starter:10:100` | None |
| `-h, --help` | Print help information |  |
| `-V, --version` | Print version information |  |

**Example Usage:**

```bash
pn -m 1000 -f 'white:80' -f 'rye:20' -p 'starter:10:100' -s 2 -d 60
```

This command will generate a bread recipe with the following parameters:

* Total flour mass: 1000g
* Hydration: 60%
* Flours: 80% white flour, 20% rye flour
* Salt: 2% of flour weight
* Preferment: 10% sourdough starter with 100% hydration

The output will be a detailed breakdown of the ingredients and their percentages, along with the total weight and hydration level.
