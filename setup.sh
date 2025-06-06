#!/bin/bash

lsof -ti :8501 | xargs kill -9

lsof -ti :8502 | xargs kill -9

lsof -ti :8060 | xargs kill -9
lsof -ti :8090 | xargs kill -9
VENV_PATH="/home/blazevfx/Documents/hackathon/scriptkiddies/bank-vc-issuer/env"

# Activate venv
source "$VENV_PATH/bin/activate"

# Run both Streamlit apps in the background
streamlit run "./bank-vc-issuer/app.py" --server.port 8501 &
streamlit run "./verifier_cli/verifier_portal.py" --server.port 8502 &

# Run Rust CLIs in the background
./verifier_cli/target/release/verifier_cli &
./wallet/target/release/wallet &
deactivate
