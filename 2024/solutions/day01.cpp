#include "day01.h"

#include <algorithm>
#include <iostream>
#include <string>
#include <vector>

using namespace std;
namespace day01 {
string part1(const vector<string> &input) {
    vector<int> left;
    vector<int> right;

    for (const string &line : input) {
        if (line.empty()) {
            continue;
        }
        int space = line.find(' ');
        left.push_back(stoi(line.substr(0, space)));
        right.push_back(stoi(line.substr(space + 1)));
    }

    sort(left.begin(), left.end());
    sort(right.begin(), right.end());

    int answer = 0;
    for (int i = 0; i < left.size(); i++) {
        answer += abs(left[i] - right[i]);
    }

    return to_string(answer);
}

string part2(const vector<string> &input) {
    vector<int> left;
    vector<int> right;

    for (const string &line : input) {
        if (line.empty()) {
            continue;
        }
        int space = line.find(' ');
        left.push_back(stoi(line.substr(0, space)));
        right.push_back(stoi(line.substr(space + 1)));
    }

    int answer = 0;
    for (int i = 0; i < left.size(); i++) {
        int left_value = left[i];
        for (int j = 0; j < right.size(); j++) {
            int right_value = right[j];
            if (left_value == right_value) {
                answer += left_value;
            }
        }
    }

    return to_string(answer);
}

void run(const vector<string> &input) {
    cout << "----- Day 1 -----" << endl;
    cout << "Part 1: " << part1(input) << endl;
    cout << "Part 2: " << part2(input) << endl;
}
};  // namespace day01