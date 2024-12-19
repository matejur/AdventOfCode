#include "day19.h"

#include <iostream>
#include <map>
#include <string>
#include <vector>

#include "../utils.h"

using namespace std;
namespace day19 {
map<string, long> memo;

bool isPossible(const vector<string> &patterns, const string &design) {
    if (design.size() == 0) {
        return true;
    }

    for (auto pattern : patterns) {
        if (design.rfind(pattern, 0) != string::npos) {
            if (isPossible(patterns, design.substr(pattern.size()))) {
                return true;
            }
        }
    }
    return false;
}

long possibleOptions(const vector<string> &patterns, const string &design) {
    if (memo.find(design) != memo.end()) {
        return memo[design];
    }

    if (design.size() == 0) {
        return 1;
    }

    long answer = 0;
    for (auto pattern : patterns) {
        if (design.rfind(pattern, 0) != string::npos) {
            answer += possibleOptions(patterns, design.substr(pattern.size()));
        }
    }

    memo[design] = answer;
    return answer;
}

string part1(const vector<string> &input) {
    auto patterns = splitString(input[0], ", ");

    int answer = 0;
    for (int i = 2; i < input.size(); i++) {
        answer += isPossible(patterns, input[i]);
    }

    return to_string(answer);
}

string part2(const vector<string> &input) {
    auto patterns = splitString(input[0], ", ");

    long answer = 0;
    for (int i = 2; i < input.size(); i++) {
        answer += possibleOptions(patterns, input[i]);
    }

    return to_string(answer);
}

void run(const vector<string> &input) {
    cout << "----- Day 19 -----" << endl;
    cout << "Part 1: " << part1(input) << endl;
    cout << "Part 2: " << part2(input) << endl;
}
};  // namespace day19
