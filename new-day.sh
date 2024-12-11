#!/bin/bash
set -eo pipefail

SCRIPT_DIR=$(dirname $0)
cd ${SCRIPT_DIR}/crates

if [[ -z "${1}" ]]; then
    cat << EOF
creates a new rust package in the workspace for the current Advent of Code day.

usage:
    new-day.sh {day}

{day} - puzzle date; package will be created as day{day} (e.g. day12)
EOF
elif [[ "${1}" =~ ^(([1-9])|(1[0-9])|(2[0-5]))$ ]]; then 
    # create new package in workspace if one doesn't already exist
    PACKAGE_NAME=day${1}

    if [[ -d "${PACKAGE_NAME}" ]]; then
        echo "${PACKAGE_NAME} already exists"
        exit 1
    fi
    cargo new ${PACKAGE_NAME}

    # Overwrite `main.rs` with template file
    cp ../sample_main.rs "${PACKAGE_NAME}/src/main.rs"

    # Update test module name
    sed -i "s/dayXYZ/${PACKAGE_NAME}/g" "${PACKAGE_NAME}/src/main.rs"

    # Initialize sample files
    touch ${PACKAGE_NAME}/{input,sample}.txt
else
    echo "invalid day: ${1}; please supply a number between 1 and 25"
fi