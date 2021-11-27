#!/bin/sh
gcc part1.c -o part1

number=$(./part1)
echo $number
python3 part2.py $number

rm part1