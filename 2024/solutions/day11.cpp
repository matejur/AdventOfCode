#include "day11.h"

#include <cmath>
#include <iostream>
#include <map>
#include <string>
#include <vector>

#include "../utils.h"

using namespace std;
namespace day11 {

long blink(const vector<string> &input, int times) {
    map<long, long> stoneNums;
    vector<int> stones = parse_numbers(input[0]);

    for (int i = 0; i < stones.size(); i++) {
        stoneNums[stones[i]]++;
    }

    for (int i = 0; i < times; i++) {
        map<long, long> newStoneNums;
        for (auto &stone : stoneNums) {
            long currStone = stone.first;
            long currStoneCount = stone.second;

            if (currStone == 0) {
                newStoneNums[1] += currStoneCount;
                continue;
            }

            int numOfDigits = log10(currStone) + 1;
            if (numOfDigits % 2 == 0) {
                long exp = pow(10, numOfDigits / 2);
                long left = currStone / exp;
                long right = currStone % exp;

                newStoneNums[left] += currStoneCount;
                newStoneNums[right] += currStoneCount;
            } else {
                newStoneNums[currStone * 2024] += currStoneCount;
            }
        }

        stoneNums = newStoneNums;
    }

    long allStones = 0;
    for (auto &stone : stoneNums) {
        allStones += stone.second;
    }

    return allStones;
}

string part1(const vector<string> &input) {
    return to_string(blink(input, 25));
}

string part2(const vector<string> &input) {
    return to_string(blink(input, 75));
}

void run(const vector<string> &input) {
    cout << "----- Day 11 -----" << endl;
    cout << "Part 1: " << part1(input) << endl;
    cout << "Part 2: " << part2(input) << endl;
}
};  // namespace day11
