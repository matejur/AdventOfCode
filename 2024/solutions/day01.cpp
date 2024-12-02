#include <algorithm>
#include <fstream>
#include <iostream>
#include <string>
#include <vector>

using namespace std;

class Day01 : public Runner {
   public:
    Day01() : Runner(1) {}

    string part1(ifstream &input) override {
        vector<int> left;
        vector<int> right;

        for (string line; getline(input, line);) {
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

    string part2(ifstream &input) override {
        vector<int> left;
        vector<int> right;

        for (string line; getline(input, line);) {
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
};