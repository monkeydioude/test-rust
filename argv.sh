#!/bin/bash

if [ "$#" -lt 1 ]
then
    echo "oh no"
    exit 1
fi

i=0
for arg in "$@"
do
    if [ $i = 3 ]
    then
        exit 0
    fi

    echo $arg
    ((i = i+1))
done
