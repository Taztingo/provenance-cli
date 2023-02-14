#!/bin/bash

# DESCRIPTION
# Sends balances from one account to another.
#
# AUTHOR
# Matthew Witkowski
#
# VERSION
# 1.0.0
#
# ARGS
# amount;The amount of currency to send;1000000000nhash
# receiver;The account to receive the balance;validator
# sender;The account to send the balance from;validator
#
# BUGS
# None at this time

PROVENANCE_BINARY="$1"
PROVENANCE_BUILD_DIR="$2"
PROVENANCE_HOME="$3"
TEST="$4"
GAS_PRICES="$5"
GAS_ADJUSTMENT="$6"

AMOUNT="$7"
TO="$8"
FROM="$9"

"$PROVENANCE_BINARY" --home "$PROVENANCE_HOME" "$TEST" tx bank send "$FROM" "$TO" "$AMOUNT" --gas auto --gas-prices "$GAS_PRICES" --gas-adjustment "$GAS_ADJUSTMENT" -y