#!/bin/bash

cargo doc --manifest-path ./mingling/Cargo.toml --no-deps --features builds,general_renderer,repl,comp,parser --open
