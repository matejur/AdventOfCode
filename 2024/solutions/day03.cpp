#include <iostream>
#include <regex>
#include <string>
#include <vector>

using namespace std;

namespace day03 {
string part1(const vector<string> &input) {
    int answer = 0;
    regex pattern(R"(mul\((\d{1,3}),(\d{1,3})\))");
    for (string line : input) {
        if (line.empty()) {
            continue;
        }

        string::const_iterator search_start(line.cbegin());
        smatch res;
        while (regex_search(search_start, line.cend(), res, pattern)) {
            int a = stoi(res[1]);
            int b = stoi(res[2]);

            answer += a * b;
            search_start = res.suffix().first;
        }
    }
    return to_string(answer);
}

string part2(const vector<string> &input) {
    int answer = 0;
    bool enabled = true;
    regex pattern(R"(mul\((\d{1,3}),(\d{1,3})\)|don't\(\)|do\(\))");
    for (string line : input) {
        if (line.empty()) {
            continue;
        }

        string::const_iterator search_start(line.cbegin());
        smatch res;
        while (regex_search(search_start, line.cend(), res, pattern)) {
            if (res[0] == "do()") {
                enabled = true;
            } else if (res[0] == "don't()") {
                enabled = false;
            } else if (enabled) {
                int a = stoi(res[1]);
                int b = stoi(res[2]);

                answer += a * b;
            }
            search_start = res.suffix().first;
        }
    }
    return to_string(answer);
}

void run(const vector<string> &input) {
    cout << "----- Day 3 -----" << endl;
    cout << "Part 1: " << part1(input) << endl;
    cout << "Part 2: " << part2(input) << endl;
}
};  // namespace day03