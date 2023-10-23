#!/bin/bash

function on_error()
{
    status=$?
    script=$1
    line=$2
    command=$3

    echo "------------------------------------------------------------"
    echo "Error occured on $script [Line $line]: Status $status"
    echo "command: $command"
    echo "------------------------------------------------------------"
}
