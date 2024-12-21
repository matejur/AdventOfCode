#include "day21.h"

#include <climits>
#include <iostream>
#include <map>
#include <queue>
#include <string>
#include <vector>

using namespace std;
namespace day21 {
vector<tuple<int, int, char>> directions = {
    {0, 1, 'v'},
    {1, 0, '>'},
    {0, -1, '^'},
    {-1, 0, '<'},
};

map<pair<char, char>, vector<string>> numericShortestPaths;
map<pair<char, char>, vector<string>> directionalShortestPaths;
vector<string> numeric_keypad = {"789", "456", "123", "#0A"};
vector<string> directional_keypad = {"#^A", "<v>"};

map<pair<char, char>, vector<string>> findShortestPaths(vector<string> keypad) {
    map<pair<char, char>, vector<string>> shortestPaths;
    for (int y1 = 0; y1 < keypad.size(); y1++) {
        for (int x1 = 0; x1 < keypad[y1].size(); x1++) {
            for (int y2 = 0; y2 < keypad.size(); y2++) {
                for (int x2 = 0; x2 < keypad[y2].size(); x2++) {
                    char c1 = keypad[y1][x1];
                    char c2 = keypad[y2][x2];
                    if (c1 == '#' || c2 == '#') {
                        continue;
                    }
                    if (c1 == c2) {
                        shortestPaths[{c1, c2}] = {"A"};
                        continue;
                    }

                    vector<string> allPaths;
                    queue<tuple<int, int, string>> q;
                    q.push({x1, y1, ""});
                    int shortest = INT_MAX;

                    while (!q.empty()) {
                        auto [x, y, path] = q.front();
                        q.pop();

                        if (path.size() > shortest) {
                            break;
                        }

                        if (x == x2 && y == y2) {
                            shortest = path.size();
                            allPaths.push_back(path + "A");
                            continue;
                        }

                        for (auto [dx, dy, dir] : directions) {
                            int nx = x + dx;
                            int ny = y + dy;
                            if (nx < 0 || nx >= keypad[y].size() || ny < 0 ||
                                ny >= keypad.size()) {
                                continue;
                            }
                            if (keypad[ny][nx] == '#') {
                                continue;
                            }
                            q.push({nx, ny, path + dir});
                        }
                    }

                    shortestPaths[{c1, c2}] = allPaths;
                }
            }
        }
    }
    return shortestPaths;
}

map<pair<pair<char, char>, int>, long> memo;
long getShortest(string path, int depth) {
    if (depth == 0) {
        return path.size();
    }

    long out = 0;
    path = "A" + path;
    for (int i = 0; i < path.size() - 1; i++) {
        char curr = path[i];
        char next = path[i + 1];

        if (memo.find({{curr, next}, depth}) != memo.end()) {
            out += memo[{{curr, next}, depth}];
            continue;
        }

        long shortest = LONG_MAX;
        for (string p : directionalShortestPaths[{curr, next}]) {
            long s = getShortest(p, depth - 1);
            if (s < shortest) {
                shortest = s;
            }
        }

        memo[{{curr, next}, depth}] = shortest;
        out += shortest;
    }

    return out;
}

long robotChain(const vector<string> &input, int numRobots) {
    long answer = 0;
    for (string code : input) {
        int numeric = stoi(code);
        code = "A" + code;

        long sequenceLength = 0;
        for (int i = 0; i < code.size() - 1; i++) {
            char curr = code[i];
            char next = code[i + 1];

            long shortest = LONG_MAX;
            for (string p : numericShortestPaths[{curr, next}]) {
                long s = getShortest(p, numRobots);
                if (s < shortest) {
                    shortest = s;
                }
            }
            sequenceLength += shortest;
        }
        answer += sequenceLength * numeric;
    }

    return answer;
}

string part1(const vector<string> &input) {
    return to_string(robotChain(input, 2));
}

string part2(const vector<string> &input) {
    return to_string(robotChain(input, 25));
}

void run(const vector<string> &input) {
    numericShortestPaths = findShortestPaths(numeric_keypad);
    directionalShortestPaths = findShortestPaths(directional_keypad);
    cout << "----- Day 21 -----" << endl;
    cout << "Part 1: " << part1(input) << endl;
    cout << "Part 2: " << part2(input) << endl;
}
};  // namespace day21
