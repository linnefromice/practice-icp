#!/bin/bash

set -e -o pipefail

script_dir=$(dirname "$(readlink -f "$0")")

. "$script_dir/units/success.sh"
. "$script_dir/units/failure.sh"
