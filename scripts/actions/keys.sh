#!/bin/bash

PIO_DIR=/home/matt/workspace/provenance
BINARY=$PIO_DIR/build/provenanced
PIO_HOME=$PIO_DIR/build/run/provenanced/
TEST=-t

"$BINARY" keys list --home "$PIO_HOME" "$TEST"