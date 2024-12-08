#!/bin/bash

arg1=$1

if [ -z "$arg1" ]; then
    echo "Provide day number as argument"
    exit 1
fi

day_string=$arg1
if [ $arg1 -lt 10 ]; then
    day_string="0$day_string"
fi

if [ -z "$AOC_SESSION" ]; then
    echo "AOC_SESSION not set"
elif [ ! -f "inputs/in$arg1.txt" ]; then
    echo "Downloading input"
    wget "https://adventofcode.com/2024/day/$arg1/input" --header "Cookie: session=$AOC_SESSION" -O "inputs/in$arg1.txt"
else
    echo "Input already exists"
fi

if [ -f "solutions/day$day_string.cpp" ]; then
    echo "Day $day_string already exists"
    exit 1
fi


if [ $? -ne 0 ]; then
    echo "Failed to download input"
    exit 1
fi

cat > "solutions/day$day_string.h" <<EOF
#ifndef DAY${day_string}_H
#define DAY${day_string}_H

#include <string>
#include <vector>

namespace day$day_string {
void run(const std::vector<std::string> &input);
}

#endif
EOF

cat > "solutions/day$day_string.cpp" <<EOF
#include "day$day_string.h"

#include <iostream>
#include <string>
#include <vector>

using namespace std;
namespace day$day_string {
string part1(const vector<string> &input) {
    return "Not implemented";
}

string part2(const vector<string> &input) {
    return "Not implemented";
}

void run(const vector<string> &input) {
    cout << "----- Day $arg1 -----" << endl;
    cout << "Part 1: " << part1(input) << endl;
    cout << "Part 2: " << part2(input) << endl;
}
};
EOF

sed -i "/\/\/ SED MARKER 1/i#include \"solutions/day$day_string.h\"" main.cpp
sed -i "/};  \/\/ SED MARKER 2/i\ \ \ \ {$arg1, day$day_string::run}," main.cpp

cmake . build