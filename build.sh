#!/bin/bash

export PATH="$HOME/.cargo/bin:$PATH"

set -e

rustc src/interpreter.rs

# -C opt-level=3

./interpreter

# strip interpreter
# ls -l interpreter
