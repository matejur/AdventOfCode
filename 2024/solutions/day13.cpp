#include "day13.h"

#include <iostream>
#include <numeric>
#include <string>
#include <vector>

#include "../utils.h"

using namespace std;
namespace day13 {
long solveEquation(vector<long> buttonA, vector<long> buttonB,
                   vector<long> prize) {
    long x1 = prize[0] * buttonB[1] - prize[1] * buttonB[0];
    long x2 = buttonA[0] * buttonB[1] - buttonA[1] * buttonB[0];

    long y1 = buttonA[0] * prize[1] - buttonA[1] * prize[0];
    long y2 = buttonA[0] * buttonB[1] - buttonA[1] * buttonB[0];

    if (x1 % x2 != 0 || y1 % y2 != 0) {
        return 0;
    }

    long x = x1 / x2;
    long y = y1 / y2;

    return 3 * x + y;
}

string part1(const vector<string> &input) {
    int answer = 0;
    for (int i = 0; i < input.size(); i += 4) {
        auto buttonA = extractAllNumbers(input[i]);
        auto buttonB = extractAllNumbers(input[i + 1]);
        auto prize = extractAllNumbers(input[i + 2]);

        answer += solveEquation(buttonA, buttonB, prize);
    }
    return to_string(answer);
}

string part2(const vector<string> &input) {
    long answer = 0;
    for (int i = 0; i < input.size(); i += 4) {
        auto buttonA = extractAllNumbers(input[i]);
        auto buttonB = extractAllNumbers(input[i + 1]);
        auto prize = extractAllNumbers(input[i + 2]);

        prize[0] += 10000000000000;
        prize[1] += 10000000000000;

        answer += solveEquation(buttonA, buttonB, prize);
    }
    return to_string(answer);
}

void run(const vector<string> &input) {
    cout << "----- Day 13 -----" << endl;
    cout << "Part 1: " << part1(input) << endl;
    cout << "Part 2: " << part2(input) << endl;
}
};  // namespace day13
