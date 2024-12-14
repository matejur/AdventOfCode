#include "day14.h"

#include <iostream>
#include <numeric>
#include <string>
#include <vector>

#include "../utils.h"

using namespace std;
namespace day14 {

int width = 101;
int height = 103;

class Robot {
    int vx, vy;

   public:
    int x, y;
    Robot(string start) {
        auto t = extractAllNumbers(start);
        x = t[0];
        y = t[1];
        vx = t[2];
        vy = t[3];
    }

    void move(int steps) {
        x = ((x + vx * steps) % width + width) % width;
        y = ((y + vy * steps) % height + height) % height;
    }

    int getQuadrant() {
        if (x < width / 2 && y < height / 2) {
            return 0;
        } else if (x > width / 2 && y < height / 2) {
            return 1;
        } else if (x < width / 2 && y > height / 2) {
            return 2;
        } else if (x > width / 2 && y > height / 2) {
            return 3;
        }
        return 4;
    };
};

int safetyFactor(vector<vector<long>> robots, int moves) {
    int q1 = 0;
    int q2 = 0;
    int q3 = 0;
    int q4 = 0;
    int vl = width / 2;
    int hl = height / 2;

    for (auto robot : robots) {
        int x = ((robot[0] + moves * robot[2]) % width + width) % width;
        int y = ((robot[1] + moves * robot[3]) % height + height) % height;

        if (x == vl || y == hl) continue;
        if (x < vl) {
            if (y < hl) {
                q1++;
            } else {
                q2++;
            }
        } else {
            if (y < hl) {
                q3++;
            } else {
                q4++;
            }
        }
    }

    return q1 * q2 * q3 * q4;
}

string part1(const vector<string> &input) {
    if (input.size() < 20) {
        width = 11;
        height = 7;
    }
    vector<vector<long>> robots;
    for (auto &line : input) {
        auto robot = extractAllNumbers(line);
        robots.push_back(robot);
    }

    return to_string(safetyFactor(robots, 100));
}

string part2(const vector<string> &input) {
    if (input.size() < 20) {
        width = 11;
        height = 7;
    }
    vector<vector<long>> robots;
    for (auto &line : input) {
        auto robot = extractAllNumbers(line);
        robots.push_back(robot);
    }

    int minSafety = INT32_MAX;
    int minSec = 0;

    for (int i = 1; i < width * height; i++) {
        int safety = safetyFactor(robots, i);

        if (safety < minSafety) {
            minSafety = safety;
            minSec = i;
        }
    }

    return to_string(minSec);
}

void run(const vector<string> &input) {
    cout << "----- Day 14 -----" << endl;
    cout << "Part 1: " << part1(input) << endl;
    cout << "Part 2: " << part2(input) << endl;
}
};  // namespace day14
