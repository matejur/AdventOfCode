#include "day25.h"

#include <array>
#include <iostream>
#include <string>
#include <vector>

using namespace std;
namespace day25 {
string part1(const vector<string> &input) {
    vector<array<int, 5>> keys;
    vector<array<int, 5>> locks;

    for (int i = 0; i < input.size(); i += 8) {
        array<int, 5> nums = {0, 0, 0, 0, 0};
        for (int y = 0; y < 5; y++) {
            for (int x = 0; x < 5; x++) {
                if (input[i + y + 1][x] == '#') {
                    nums[x]++;
                }
            }
        }

        if (input[i][0] == '#') {
            locks.push_back(nums);
        } else {
            keys.push_back(nums);
        }
    }

    int answer = 0;
    for (const auto &key : keys) {
        for (const auto &lock : locks) {
            bool valid = true;
            for (int pin = 0; pin < 5; pin++) {
                if (key[pin] + lock[pin] > 5) {
                    valid = false;
                    break;
                }
            }
            if (valid) {
                answer++;
            }
        }
    }

    return to_string(answer);
}

string part2(const vector<string> &input) { return "Merry Xmas!"; }

void run(const vector<string> &input) {
    cout << "----- Day 25 -----" << endl;
    cout << "Part 1: " << part1(input) << endl;
    cout << "Part 2: " << part2(input) << endl;
}
};  // namespace day25
