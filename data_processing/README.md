# Welcome to the data processing folder

This folder contains scripts for __data collection__ and __analysis__.
It is primarily written in __Python__ and uses the __UV__ package manager.

## Structure
- `data/` — Folder containing data files stored as __.csv__
- `src/` — Source code folder
- `config.toml.example` — Example configuration file used by the scripts

## Usage
Copy and edit the configuration file:
```bash
cp config.toml.example config.toml
nano config.toml
```
Run a script:
```bash
uv run src/my_script.py
```

