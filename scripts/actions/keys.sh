#!/bin/bash

BINARY=$1
PIO_HOME=$2
TEST=$3

"$BINARY" keys list --home "$PIO_HOME" "$TEST"