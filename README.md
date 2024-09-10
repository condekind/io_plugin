
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
