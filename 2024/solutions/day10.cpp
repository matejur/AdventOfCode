#include "day10.h"

#include <iostream>
#include <queue>
#include <string>
#include <tuple>
#include <vector>

using namespace std;
namespace day10 {
int trailheadScore(const vector<string> &input, int x, int y, int prev,
                   bool visited[], bool part2) {
    if (x < 0 || x >= input[0].size() || y < 0 || y >= input.size()) {
        return 0;
    }

    if (visited[y * input.size() + x]) {
        return 0;
    }

    char current = input[y][x];
    if (prev + 1 != current) {
        return 0;
    }

    int score = 0;
    visited[y * input.size() + x] = true;
    if (current == '9') {
        score++;
    } else {
        score += trailheadScore(input, x + 1, y, current, visited, part2);
        score += trailheadScore(input, x - 1, y, current, visited, part2);
        score += trailheadScore(input, x, y + 1, current, visited, part2);
        score += trailheadScore(input, x, y - 1, current, visited, part2);
    }
    if (part2) {
        visited[y * input.size() + x] = false;
    }

    return score;
}
string part1(const vector<string> &input) {
    int answer = 0;
    for (int y = 0; y < input.size(); y++) {
        for (int x = 0; x < input[y].size(); x++) {
            if (input[y][x] == '0') {
                bool visited[input.size() * input[0].size()] = {false};
                answer += trailheadScore(input, x, y, '0' - 1, visited, false);
            }
        }
    }

    return to_string(answer);
}

string part2(const vector<string> &input) {
    int answer = 0;
    for (int y = 0; y < input.size(); y++) {
        for (int x = 0; x < input[y].size(); x++) {
            if (input[y][x] == '0') {
                bool visited[input.size() * input[0].size()] = {false};
                answer += trailheadScore(input, x, y, '0' - 1, visited, true);
            }
        }
    }

    return to_string(answer);
}

void run(const vector<string> &input) {
    cout << "----- Day 10 -----" << endl;
    cout << "Part 1: " << part1(input) << endl;
    cout << "Part 2: " << part2(input) << endl;
}
};  // namespace day10
