#include "day07.h"

#include <cmath>
#include <iostream>
#include <string>
#include <vector>

#include "../utils.h"

using namespace std;
namespace day07 {
bool canBeTrue(int i, long testValue, const vector<int> &equation) {
    if (i == 0) {
        return equation[i] == testValue;
    }
    if (testValue % equation[i] == 0 &&
        canBeTrue(i - 1, testValue / equation[i], equation)) {
        return true;
    }
    if (testValue - equation[i] > 0 &&
        canBeTrue(i - 1, testValue - equation[i], equation)) {
        return true;
    }

    return false;
}

bool canBeTrueConcat(int i, long testValue, const vector<int> &equation) {
    int curr = equation[i];
    if (i == 0) {
        return curr == testValue;
    }
    if (testValue % curr == 0 &&
        canBeTrueConcat(i - 1, testValue / equation[i], equation)) {
        return true;
    }
    if (testValue > curr &&
        canBeTrueConcat(i - 1, testValue - equation[i], equation)) {
        return true;
    }

    int curDigits = log10(curr) + 1;
    int valueDigits = log10(testValue) + 1;

    if (valueDigits > curDigits &&
        testValue % (int)pow(10, curDigits) == curr &&
        canBeTrueConcat(i - 1, testValue / (int)pow(10, curDigits), equation)) {
        return true;
    }

    return false;
}

string part1(const vector<string> &input) {
    long answer = 0;
    for (string line : input) {
        int colon = line.find(':');
        long testValue = stol(line.substr(0, colon));
        vector<int> equation =
            parseNumbersDelimiter(line.substr(colon + 2), ' ');
        if (canBeTrue(equation.size() - 1, testValue, equation)) {
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
        if (canBeTrueConcat(equation.size() - 1, testValue, equation)) {
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
