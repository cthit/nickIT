#!/usr/bin/env sh

# Activate virtual envriontment
. ./venv/bin/activate

# Flask entry point
export FLASK_APP=nickITAPI.app

# Run application allowing external requests
python3 -m flask run --host 0.0.0.0
