#include <algorithm>
#include <fstream>
#include <iostream>
#include <regex>
#include <sstream>
#include <string>
#include <vector>

#include "../runner.cpp"

using namespace std;

class Day03 : public Runner {
    string part1(ifstream &input) override {
        int answer = 0;
        regex pattern(R"(mul\((\d{1,3}),(\d{1,3})\))");
        for (string line; getline(input, line);) {
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

    string part2(ifstream &input) override {
        int answer = 0;
        bool enabled = true;
        regex pattern(R"(mul\((\d{1,3}),(\d{1,3})\)|don't\(\)|do\(\))");
        for (string line; getline(input, line);) {
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
};