#include <algorithm>
#include <iostream>
#include <string>
#include <vector>

#include "../utils.h"

using namespace std;

namespace day02 {
bool is_safe(vector<int> &numbers) {
    bool increasing = numbers[0] < numbers[1];

    for (int i = 0; i < numbers.size() - 1; i++) {
        int number = numbers[i + 1];
        int prev = numbers[i];

        if (abs(number - prev) > 3 || abs(number - prev) < 1) {
            return false;
        }
        if (increasing && number < prev) {
            return false;
        }
        if (!increasing && number > prev) {
            return false;
        }
    }
    return true;
}

string part1(const vector<string> &input) {
    int safe = 0;
    for (string line : input) {
        if (line.empty()) {
            continue;
        }
        auto numbers = parse_numbers(line);
        safe += is_safe(numbers);
    }
    return to_string(safe);
}

string part2(const vector<string> &input) {
    int safe = 0;
    for (string line : input) {
        if (line.empty()) {
            continue;
        }

        auto numbers = parse_numbers(line);
        if (is_safe(numbers)) {
            safe++;
        } else {
            for (int i = 0; i < numbers.size(); i++) {
                vector<int> copy = numbers;
                copy.erase(copy.begin() + i);
                if (is_safe(copy)) {
                    safe++;
                    break;
                }
            }
        }
    }
    return to_string(safe);
}

void run(const vector<string> &input) {
    cout << "Part 1: " << part1(input) << endl;
    cout << "Part 2: " << part2(input) << endl;
}
};  // namespace day02