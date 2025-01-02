#!/bin/bash

export PATH="$HOME/.cargo/bin:$PATH"

set -e

rustc src/interpreter.rs

./interpreter