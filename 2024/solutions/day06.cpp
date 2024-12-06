#include "day06.h"

#include <iostream>
#include <string>
#include <unordered_set>
#include <vector>

using namespace std;
namespace day06 {
int dir[4][2] = {{0, -1}, {1, 0}, {0, 1}, {-1, 0}};

void getStart(const vector<string> &input, int &startX, int &startY) {
    for (int i = 0; i < input.size(); i++) {
        for (int j = 0; j < input[i].size(); j++) {
            if (input[i][j] == '^') {
                startX = j;
                startY = i;
            }
        }
    }
}

unordered_set<int> getVisited(const vector<string> &input, int x, int y) {
    unordered_set<int> visited;
    visited.insert(x + y * input[0].size());
    int curDir = 0;
    while (true) {
        int newX = x + dir[curDir][0];
        int newY = y + dir[curDir][1];

        if (newX < 0 || newX >= input[0].size() || newY < 0 ||
            newY >= input.size()) {
            break;
        }

        if (input[newY][newX] == '#') {
            curDir = (curDir + 1) % 4;
        } else {
            x = newX;
            y = newY;
            visited.insert(x + y * input[0].size());
        }
    }
    return visited;
}

string part1(const vector<string> &input) {
    int x = 0;
    int y = 0;
    getStart(input, x, y);

    auto visited = getVisited(input, x, y);

    return to_string(visited.size());
}

string part2(const vector<string> &input) {
    int dir[4][2] = {{0, -1}, {1, 0}, {0, 1}, {-1, 0}};
    int startX = 0;
    int startY = 0;
    getStart(input, startX, startY);
    auto visited = getVisited(input, startX, startY);

    int width = input[0].size();
    int area = input.size() * width;

    int answer = 0;
    for (int visit : visited) {
        int ox = visit % width;
        int oy = visit / width;

        if (ox == startX && oy == startY) {
            continue;
        }

        int x = startX;
        int y = startY;
        int curDir = 0;

        unordered_set<int> visited;
        visited.insert(startX + startY * width + curDir * area);

        while (true) {
            int newX = x + dir[curDir][0];
            int newY = y + dir[curDir][1];

            if (newX < 0 || newX >= width || newY < 0 || newY >= input.size()) {
                break;
            }

            if (input[newY][newX] == '#' || (newX == ox && newY == oy)) {
                curDir = (curDir + 1) % 4;
                int key = x + y * width + curDir * area;
                if (visited.count(key)) {
                    answer++;
                    break;
                }
                visited.insert(key);
            } else {
                x = newX;
                y = newY;
            }
        }
    }
    return to_string(answer);
}

void run(const vector<string> &input) {
    cout << "----- Day 6 -----" << endl;
    cout << "Part 1: " << part1(input) << endl;
    cout << "Part 2: " << part2(input) << endl;
}
};  // namespace day06
