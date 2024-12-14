#include "day14.h"

#include <iostream>
#include <limits>
#include <numeric>
#include <string>
#include <vector>

#include "../utils.h"

using namespace std;
namespace day14 {

int width = 101;
int height = 103;

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

    int lowestXVariance = numeric_limits<int>::max();
    int lowestYVariance = numeric_limits<int>::max();
    int xSeconds = 0;
    int ySeconds = 0;

    for (int s = 1; s < max(width, height); s++) {
        int xs[robots.size()];
        int ys[robots.size()];
        for (int i = 0; i < robots.size(); i++) {
            xs[i] = ((robots[i][0] + s * robots[i][2]) % width + width) % width;
            ys[i] =
                ((robots[i][1] + s * robots[i][3]) % height + height) % height;
        }

        int meanX = accumulate(xs, xs + robots.size(), 0) / robots.size();
        int meanY = accumulate(ys, ys + robots.size(), 0) / robots.size();

        int varX = 0;
        int varY = 0;
        for (int i = 0; i < robots.size(); i++) {
            varX += (xs[i] - meanX) * (xs[i] - meanX);
            varY += (ys[i] - meanY) * (ys[i] - meanY);
        }

        if (varX < lowestXVariance) {
            lowestXVariance = varX;
            xSeconds = s;
        }
        if (varY < lowestYVariance) {
            lowestYVariance = varY;
            ySeconds = s;
        }
    }

    // Naive chinese remainder theorem
    int answer = 0;
    while (true) {
        if (answer % width == xSeconds && answer % height == ySeconds) {
            break;
        }
        answer++;
    }

    return to_string(answer);
}

void run(const vector<string> &input) {
    cout << "----- Day 14 -----" << endl;
    cout << "Part 1: " << part1(input) << endl;
    cout << "Part 2: " << part2(input) << endl;
}
};  // namespace day14
