#include <algorithm>
#include <fstream>
#include <iostream>
#include <sstream>
#include <string>
#include <vector>

#include "../runner.cpp"
#include "../utils.cpp"

using namespace std;

class Day02 : public Runner {
    string part1(ifstream &input) override {
        int safe = 0;
        for (string line; getline(input, line);) {
            if (line.empty()) {
                continue;
            }
            auto numbers = parse_numbers(line);
            safe += is_safe(numbers);
        }
        return to_string(safe);
    }

    string part2(ifstream &input) override {
        int safe = 0;
        for (string line; getline(input, line);) {
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
};