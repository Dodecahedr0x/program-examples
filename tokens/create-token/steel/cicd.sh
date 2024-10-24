#!/bin/bash

# This script is for quick building & deploying of the program.
# It also serves as a reference for the commands used for building & deploying Solana programs.
# Run this bad boy with "bash cicd.sh" or "./cicd.sh"

cargo build-sbf --manifest-path=./program/Cargo.toml
solana program deploy ./tests/fixtures/token_metadata.so -u localhost
solana program deploy ./target/deploy/create_token_program.so -u localhost
