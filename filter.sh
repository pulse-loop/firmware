#!/bin/bash

# read data from a txt file and take milliseconds R value.
# Example input: I (<milliseconds>) firmware: R: <R value>%
# Example output: <milliseconds>,<R value>

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
    # Check if line contains R value
    if [[ $line == *"R:"* ]]; then
        # Get milliseconds value
        milliseconds=$(echo $line | grep -oP '(?<=I \().*(?=ms firmware: R:)')
        # Get R value
        r_value=$(echo $line | grep -oP '(?<=R: ).*(?=%)')
        # Write to output file
        echo "$milliseconds,$r_value" >> $2
    fi
done < $1