#!/usr/bin/env sh

# Initiate the virtual environment for application and install dependencies
# Initiate project
python3 -m venv venv

# Activate environment and install requirements
. ./venv/bin/activate
pip install -e .
