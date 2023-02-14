#!/bin/bash

# DESCRIPTION
# Queries the balances for the account.
#
# AUTHOR
# Matthew Witkowski
#
# VERSION
# 1.0.0
#
# ARGS
# account;The account to query;validator
#
# BUGS
# None at this time

PROVENANCE_BINARY="$1"
PROVENANCE_BUILD_DIR="$2"
PROVENANCE_HOME="$3"
TEST="$4"
GAS_PRICES="$5"
GAS_ADJUSTMENT="$6"

ACCOUNT=$7

"$PROVENANCE_BINARY" --home "$PROVENANCE_HOME" "$TEST" q bank balances "$ACCOUNT"