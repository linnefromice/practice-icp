#!/bin/bash

script_dir=$(dirname "$(readlink -f "$0")")

. "$script_dir/utils.sh"

set -e -o pipefail
trap 'on_error $BASH_SOURCE $LINENO "$BASH_COMMAND" "$@"' ERR

. "$script_dir/units/success.sh"
. "$script_dir/units/failure.sh"
