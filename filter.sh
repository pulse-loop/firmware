#!/bin/bash

# read data from a txt file and take milliseconds and O2 values
# Example input: I (<milliseconds>) firmware: O2: <O2 value>%
# Output a csv file with the milliseconds and O2 values.

# Usage: ./filter.sh <input file> <output file>

# Check if the input file exists
if [ ! -f $1 ]; then
    echo "Input file does not exist"
    exit 1
fi

# Check if the output file exists
if [ -f $2 ]; then
    echo "Output file already exists"
    exit 1
fi

# Read the input file line by line
while read line; do
    # Check if the line contains the O2 value
    if [[ $line == *"O2:"* ]]; then
        # Get the milliseconds
        milliseconds=$(echo $line | grep -oP '(?<=I \().*(?=\) firmware: O2: )')
        # Get the O2 value
        O2=$(echo $line | grep -oP '(?<=O2: ).*(?=%)')
        # Write the milliseconds and O2 value to the output file
        echo "$milliseconds,$O2" >> $2
    fi
done < $1
