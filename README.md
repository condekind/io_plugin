
# IO Plugin Exploration

## Build

(Optional): if you have pyenv, install Python 3.12.4:

```shell
pyenv install 3.12.4
```

Create and activate a virtual environment:

```python
# If you're not using pyenv, change `python` to `/usr/bin/python3`
# or the location of your desired Python 3.12+ version
python -m venv .venv

# Activate the venv
source .venv/bin/activate
```

Install maturin and polars:
```shell
pip install maturin polars
```

Build the plugin:

```shell
maturin develop
```

## Run

Inspect/tweak the contents of `run.py`, then:

```
python run.py [NAME-OF-FILE]
```

Where `NAME-OF-FILE` is the name of any file that will be read by the Line Reader IO Plugin (if omitted, `/dev/null` is the default)

## Example

```shell
python run.py /etc/os-release
```

```
--- Fibonacci sequence plugin ---
lf.head(10).collect():
shape: (10, 2)
┌─────────┬─────────┐
│ fib_pos ┆ fib_neg │
│ ---     ┆ ---     │
│ i64     ┆ i64     │
╞═════════╪═════════╡
│ 1       ┆ -1      │
│ 1       ┆ -1      │
│ 2       ┆ -2      │
│ 3       ┆ -3      │
│ 5       ┆ -5      │
│ 8       ┆ -8      │
│ 13      ┆ -13     │
│ 21      ┆ -21     │
│ 34      ┆ -34     │
│ 55      ┆ -55     │
└─────────┴─────────┘

lf.head(10).collect():
shape: (10, 2)
┌─────────┬─────────┐
│ fib_pos ┆ fib_neg │
│ ---     ┆ ---     │
│ i64     ┆ i64     │
╞═════════╪═════════╡
│ 89      ┆ -89     │
│ 144     ┆ -144    │
│ 233     ┆ -233    │
│ 377     ┆ -377    │
│ 610     ┆ -610    │
│ 987     ┆ -987    │
│ 1597    ┆ -1597   │
│ 2584    ┆ -2584   │
│ 4181    ┆ -4181   │
│ 6765    ┆ -6765   │
└─────────┴─────────┘

--- Line reader plugin ---
lf.head(3).collect():
shape: (3, 1)
┌──────────────────────────┐
│ lines                    │
│ ---                      │
│ str                      │
╞══════════════════════════╡
│ NAME="Arch Linux"        │
│ PRETTY_NAME="Arch Linux" │
│ ID=arch                  │
└──────────────────────────┘
```
