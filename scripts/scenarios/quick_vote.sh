#!/bin/bash

# DESCRIPTION
# Sets up a simple blockchain scenario with the voting period set to 20 seconds.
#
# AUTHOR
# Carlton Hannah
#
# VERSION
# 1.0.0
#
# ARGS
# seconds;The voting period length in seconds;20
#
# BUGS
# None at this time


# Provenance Configuration
PROVENANCE_BINARY="$1"
PROVENANCE_BUILD_DIR="$2"
PROVENANCE_HOME="$3"
TEST="$4"
GAS_PRICES="$5"
GAS_ADJUSTMENT="$6"

# Quickvote config
CONFIG=run-config
VOTING_PERIOD="$7s"
MINUTE_EPOCH="{\"identifier\": \"minute\",\"start_height\": \"0\",\"duration\": \"12\",\"current_epoch\": \"0\", \"current_epoch_start_height\": \"0\",\"epoch_counting_started\": false}"

# Clean and rebuild my provenance run environment
build_provenance() {
    cd ${PROVENANCE_BUILD_DIR}
    go mod vendor
    make clean 
    make build 
    make ${CONFIG}
}

# Adjust the voting period to have faster proposal voting for development
adjust_voting_period() {
    cat ${PROVENANCE_HOME}/config/genesis.json | jq ' .app_state.gov.voting_params.voting_period="'${VOTING_PERIOD}'" ' | tee ${PROVENANCE_HOME}/config/genesis.json
}

# Main entrypoint for the script
main() {
    DIR=$(pwd)
    build_provenance
    adjust_voting_period
    cd "$DIR"
}

DIR=$(pwd)

cd ${PROVENANCE_BUILD_DIR}
go mod vendor
make clean 
make build 
make ${CONFIG}

cat ${PROVENANCE_HOME}/config/genesis.json | jq ' .app_state.gov.voting_params.voting_period="'${VOTING_PERIOD}'" ' | tee ${PROVENANCE_HOME}/config/genesis.json

cd "$DIR"