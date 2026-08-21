#!/usr/bin/env bash

# Setting virtual environments for JAX
python3 -m venv .venv
source .venv/bin/activate
pip install -r engine/requirements.txt

# Running the scalogram web engine without it hogging memory 
XLA_PYTHON_CLIENT_ALLOCATOR=platform cargo run --release
