#include "day20.h"

#include <iomanip>
#include <iostream>
#include <map>
#include <queue>
#include <set>
#include <string>
#include <vector>

using namespace std;
namespace day20 {

map<pair<int, int>, int> calculateDistances(const vector<string> &grid,
                                            int startX, int startY) {
    map<pair<int, int>, int> distances;

    queue<tuple<int, int, int>> q;
    q.push({0, startX, startY});

    while (!q.empty()) {
        auto [dist, x, y] = q.front();
        q.pop();

        if (grid[y][x] == '#') {
            continue;
        }
        if (distances.find({x, y}) != distances.end()) {
            continue;
        }

        distances[{x, y}] = dist;

        q.push({dist + 1, x + 1, y});
        q.push({dist + 1, x - 1, y});
        q.push({dist + 1, x, y + 1});
        q.push({dist + 1, x, y - 1});
    }

    return distances;
}

void cheatForPicoseconds(map<int, int> &outCheats, int picoseconds,
                         map<pair<int, int>, int> distancesToEnd, int x, int y,
                         int distFromStart, int shortestPath) {
    for (int dy = -picoseconds; dy <= picoseconds; dy++) {
        int maxDx = picoseconds - abs(dy);
        for (int dx = -maxDx; dx <= maxDx; dx++) {
            int newX = x + dx;
            int newY = y + dy;

            auto distToEnd = distancesToEnd.find({newX, newY});
            if (distToEnd != distancesToEnd.end()) {
                int length =
                    distFromStart + distToEnd->second + abs(dx) + abs(dy);
                if (length < shortestPath) {
                    outCheats[shortestPath - length]++;
                }
            }
        }
    }
}

map<int, int> getCheats(map<pair<int, int>, int> distancesFromStart,
                        map<pair<int, int>, int> distancesFromEnd,
                        int shortestPath, int cheatDuration) {
    map<int, int> cheats;
    for (auto [pos, dist] : distancesFromStart) {
        auto [x, y] = pos;

        cheatForPicoseconds(cheats, cheatDuration, distancesFromEnd, x, y, dist,
                            shortestPath);
    }

    return cheats;
}

string part1(const vector<string> &input) {
    int startX, startY, endX, endY;
    for (int y = 0; y < input.size(); y++) {
        for (int x = 0; x < input[y].size(); x++) {
            if (input[y][x] == 'S') {
                startX = x;
                startY = y;
            } else if (input[y][x] == 'E') {
                endX = x;
                endY = y;
            }
        }
    }

    auto distancesFromStart = calculateDistances(input, startX, startY);
    auto distancesFromEnd = calculateDistances(input, endX, endY);

    auto cheats = getCheats(distancesFromStart, distancesFromEnd,
                            distancesFromStart[{endX, endY}], 2);

    int answer = 0;
    for (auto [length, count] : cheats) {
        if (length >= 100) {
            answer += count;
        }
    }

    return to_string(answer);
}

string part2(const vector<string> &input) {
    int startX, startY, endX, endY;
    for (int y = 0; y < input.size(); y++) {
        for (int x = 0; x < input[y].size(); x++) {
            if (input[y][x] == 'S') {
                startX = x;
                startY = y;
            } else if (input[y][x] == 'E') {
                endX = x;
                endY = y;
            }
        }
    }

    auto distancesFromStart = calculateDistances(input, startX, startY);
    auto distancesFromEnd = calculateDistances(input, endX, endY);

    auto cheats = getCheats(distancesFromStart, distancesFromEnd,
                            distancesFromStart[{endX, endY}], 20);

    int answer = 0;
    for (auto [length, count] : cheats) {
        if (length >= 100) {
            answer += count;
        }
    }

    return to_string(answer);
}

void run(const vector<string> &input) {
    cout << "----- Day 20 -----" << endl;
    cout << "Part 1: " << part1(input) << endl;
    cout << "Part 2: " << part2(input) << endl;
}
};  // namespace day20
