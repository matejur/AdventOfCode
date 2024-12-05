#include "day05.h"

#include <algorithm>
#include <iostream>
#include <map>
#include <string>
#include <vector>

#include "../utils.h"

using namespace std;
namespace day05 {

bool checkUpdate(vector<int> update, const map<int, vector<int>> &rules) {
    for (int i = 0; i < update.size() - 1; i++) {
        for (int j = i + 1; j < update.size(); j++) {
            if (rules.find(update[i]) == rules.end()) {
                return false;
            }
            auto rule = rules.at(update[i]);
            if (find(rule.begin(), rule.end(), update[j]) == rule.end()) {
                return false;
            }
        }
    }
    return true;
}

int fixUpdateAndReturnMiddle(vector<int> &update,
                             const map<int, vector<int>> &rules) {
    for (int i = 0; i < update.size() - 1; i++) {
        for (int j = i + 1; j < update.size(); j++) {
            if (rules.find(update[i]) == rules.end()) {
                int temp = update[i];
                update[i] = update[update.size() - 1];
                update[update.size() - 1] = temp;
                continue;
            }
            auto rule = rules.at(update[i]);
            if (find(rule.begin(), rule.end(), update[j]) == rule.end()) {
                int temp = update[j];
                update[j] = update[i];
                update[i] = temp;
            }
        }
    }

    return update[update.size() / 2];
}

string part1(const vector<string> &input) {
    bool readingRules = true;
    int answer = 0;

    map<int, vector<int>> rules;
    for (string line : input) {
        if (line.empty()) {
            readingRules = false;
            continue;
        }
        if (readingRules) {
            auto rule = parseNumbersDelimiter(line, '|');
            rules[rule[0]].push_back(rule[1]);
        } else {
            auto updates = parseNumbersDelimiter(line, ',');
            if (checkUpdate(updates, rules)) {
                int middle = updates[updates.size() / 2];
                answer += middle;
            }
        }
    }

    return to_string(answer);
}

string part2(const vector<string> &input) {
    bool readingRules = true;
    int answer = 0;

    map<int, vector<int>> rules;
    for (string line : input) {
        if (line.empty()) {
            readingRules = false;
            continue;
        }
        if (readingRules) {
            auto rule = parseNumbersDelimiter(line, '|');
            rules[rule[0]].push_back(rule[1]);
        } else {
            auto updates = parseNumbersDelimiter(line, ',');
            if (!checkUpdate(updates, rules)) {
                answer += fixUpdateAndReturnMiddle(updates, rules);
            }
        }
    }
    return to_string(answer);
}

void run(const vector<string> &input) {
    cout << "----- Day 5 -----" << endl;
    cout << "Part 1: " << part1(input) << endl;
    cout << "Part 2: " << part2(input) << endl;
}
};  // namespace day05
