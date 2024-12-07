#include "day07.h"

#include <cmath>
#include <iostream>
#include <string>
#include <vector>

#include "../utils.h"

using namespace std;
namespace day07 {
bool canBeTrue(int i, long value, long testValue, const vector<int> &equation) {
    if (i == equation.size()) {
        return value == testValue;
    }
    return canBeTrue(i + 1, value * equation[i], testValue, equation) ||
           canBeTrue(i + 1, value + equation[i], testValue, equation);
}

bool canBeTrueConcat(int i, long value, long testValue,
                     const vector<int> &equation) {
    if (value > testValue) {
        return false;
    }

    if (i == equation.size()) {
        return value == testValue;
    }

    long concatValue =
        value * pow(10, (int)log10(equation[i]) + 1) + equation[i];
    return canBeTrueConcat(i + 1, concatValue, testValue, equation) ||
           canBeTrueConcat(i + 1, value * equation[i], testValue, equation) ||
           canBeTrueConcat(i + 1, value + equation[i], testValue, equation);
}

string part1(const vector<string> &input) {
    long answer = 0;
    for (string line : input) {
        int colon = line.find(':');
        long testValue = stol(line.substr(0, colon));
        vector<int> equation =
            parseNumbersDelimiter(line.substr(colon + 2), ' ');
        if (canBeTrue(1, equation[0], testValue, equation)) {
            answer += testValue;
        }
    }
    return to_string(answer);
}

string part2(const vector<string> &input) {
    long answer = 0;
    for (string line : input) {
        int colon = line.find(':');
        long testValue = stol(line.substr(0, colon));
        vector<int> equation =
            parseNumbersDelimiter(line.substr(colon + 2), ' ');
        if (canBeTrueConcat(0, 0, testValue, equation)) {
            answer += testValue;
        }
    }
    return to_string(answer);
}

void run(const vector<string> &input) {
    cout << "----- Day 7 -----" << endl;
    cout << "Part 1: " << part1(input) << endl;
    cout << "Part 2: " << part2(input) << endl;
}
};  // namespace day07
