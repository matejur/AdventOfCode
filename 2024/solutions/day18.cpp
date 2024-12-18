#include "day18.h"

#include <iostream>
#include <map>
#include <queue>
#include <set>
#include <string>
#include <vector>

#include "../utils.h"

using namespace std;
namespace day18 {

int width = 71;
int height = 71;

int pathLength(const vector<string> fallingBytes, int numBytes) {
    string grid[height + 2];
    for (int i = 0; i < height; i++) {
        grid[i + 1] = '#' + string(width, '.') + '#';
    }
    grid[0] = grid[height + 1] = string(width + 2, '#');

    for (int i = 0; i < numBytes; i++) {
        auto coords = parseNumbersDelimiter(fallingBytes[i], ',');
        grid[coords[1] + 1][coords[0] + 1] = '#';
    }

    int x = 1;
    int y = 1;
    queue<tuple<int, int, int>> q;
    q.push({x, y, 0});

    set<pair<int, int>> visited;
    visited.insert({x, y});
    while (!q.empty()) {
        auto [x, y, steps] = q.front();
        q.pop();

        if (x == width && y == height) {
            return steps;
        }

        for (int i = 0; i < 4; i++) {
            int nx = x;
            int ny = y;
            switch (i) {
                case 0:
                    nx++;
                    break;
                case 1:
                    nx--;
                    break;
                case 2:
                    ny++;
                    break;
                case 3:
                    ny--;
                    break;
            }

            if (grid[ny][nx] == '#') {
                continue;
            }

            if (visited.find({nx, ny}) != visited.end()) {
                continue;
            }

            visited.insert({nx, ny});
            q.push({nx, ny, steps + 1});
        }
    }

    return -1;
}

string part1(const vector<string> &input) {
    int numBytes = 1024;
    if (input.size() < 30) {
        width = 7;
        height = 7;
        numBytes = 12;
    }

    return to_string(pathLength(input, numBytes));
}

string part2(const vector<string> &input) {
    if (input.size() < 30) {
        width = 7;
        height = 7;
    }

    int numBytesLow = 0;
    int numBytesHigh = input.size();

    while (numBytesLow < numBytesHigh) {
        int numBytes = (numBytesLow + numBytesHigh) / 2;
        if (pathLength(input, numBytes) == -1) {
            numBytesHigh = numBytes;
        } else {
            numBytesLow = numBytes + 1;
        }
    }

    return input[numBytesHigh - 1];
}

void run(const vector<string> &input) {
    cout << "----- Day 18 -----" << endl;
    cout << "Part 1: " << part1(input) << endl;
    cout << "Part 2: " << part2(input) << endl;
}
};  // namespace day18
