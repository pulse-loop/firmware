#!/bin/bash

# read data from a txt file and take milliseconds R value.
# Example input: I (<milliseconds>) firmware: R: [<value>]
# Example output: <milliseconds>,<value>

# Usage: ./filter.sh <input file> <output file>

# Check if input file exists
if [ ! -f $1 ]; then
    echo "Input file does not exist"
    exit 1
fi

# Check if output file exists
if [ -f $2 ]; then
    echo "Output file already exists"
    exit 1
fi

# Read input file line by line
while read line; do
    # Check if line contains "firmware: R:"
    if [[ $line == *"firmware: R:"* ]]; then
        # Extract milliseconds and R value
        ms=$(echo $line | grep -oP '(?<=I \().*(?=\))')
        r=$(echo $line | grep -oP '(?<=firmware: R: \[).*(?=\])')
        # Write to output file
        echo "$ms,$r" >> $2
    fi
done < $1