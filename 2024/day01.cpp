#include <string>
#include <fstream>
#include <vector>
#include <algorithm>
#include <iostream>

std::string part1(std::ifstream &input) {
    std::vector<int> left;
    std::vector<int> right;

    for (std::string line; std::getline(input, line);) {
        int space = line.find(' ');
        left.push_back(std::stoi(line.substr(0, space)));
        right.push_back(std::stoi(line.substr(space + 1)));
    }

    std::sort(left.begin(), left.end());
    std::sort(right.begin(), right.end());

    int answer = 0;
    for (int i = 0; i < left.size(); i++) {
        answer += std::abs(left[i] - right[i]);
    }

    return std::to_string(answer);
}

std::string part2(std::ifstream &input) {
    std::vector<int> left;
    std::vector<int> right;

    for (std::string line; std::getline(input, line);) {
        int space = line.find(' ');
        left.push_back(std::stoi(line.substr(0, space)));
        right.push_back(std::stoi(line.substr(space + 1)));
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

    return std::to_string(answer);
}