#!/bin/bash

# Exit on any error
set -e

trunk build --release

cargo run --release