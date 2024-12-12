#include "day12.h"

#include <iostream>
#include <map>
#include <queue>
#include <set>
#include <string>
#include <vector>

using namespace std;
namespace day12 {
int dirs[4][2] = {{1, 0}, {0, 1}, {-1, 0}, {0, -1}};

int countSides(const set<tuple<int, int, int>> &fences) {
    int sides = 0;
    set<tuple<int, int, int>> visited;

    for (auto &fence : fences) {
        auto [x, y, dir] = fence;

        if (visited.count(fence)) {
            continue;
        }

        visited.insert(fence);

        int startX = x;
        int startY = y;

        // go perpendicular to the fence in both directions
        int newDir = (dir + 1) % 4;
        while (fences.count({x, y, dir})) {
            x += dirs[newDir][0];
            y += dirs[newDir][1];
            visited.insert({x, y, dir});
        }

        x = startX;
        y = startY;
        newDir = (dir + 3) % 4;
        while (fences.count({x, y, dir})) {
            x += dirs[newDir][0];
            y += dirs[newDir][1];
            visited.insert({x, y, dir});
        }

        sides++;
    }

    return sides;
}
int calculatePrice(const vector<string> &input, int x, int y, bool *visited,
                   bool part2) {
    set<tuple<int, int, int>> fences;
    queue<tuple<int, int, int>> q;
    q.push({x, y, 0});

    char region = input[y][x];
    int area = 0;
    int perimeter = 0;
    int corners = 0;

    while (!q.empty()) {
        auto [x, y, dir] = q.front();
        q.pop();

        if (x < 0 || x >= input[y].size() || y < 0 || y >= input.size() ||
            input[y][x] != region) {
            perimeter++;
            fences.insert({x, y, dir});
            continue;
        }

        if (visited[y * input[y].size() + x]) {
            continue;
        }

        visited[y * input[y].size() + x] = true;
        area++;

        for (int i = 0; i < 4; i++) {
            q.push({x + dirs[i][0], y + dirs[i][1], i});
        }
    }

    if (part2) {
        perimeter = countSides(fences);
    }

    return area * perimeter;
}

string part1(const vector<string> &input) {
    bool visited[input.size() * input[0].size()] = {false};

    int answer = 0;
    for (int y = 0; y < input.size(); y++) {
        for (int x = 0; x < input[0].size(); x++) {
            if (!visited[y * input[0].size() + x]) {
                answer += calculatePrice(input, x, y, visited, false);
            }
        }
    }

    return to_string(answer);
}

string part2(const vector<string> &input) {
    bool visited[input.size() * input[0].size()] = {false};

    int answer = 0;
    for (int y = 0; y < input.size(); y++) {
        for (int x = 0; x < input[0].size(); x++) {
            if (!visited[y * input[0].size() + x]) {
                answer += calculatePrice(input, x, y, visited, true);
            }
        }
    }

    return to_string(answer);
}

void run(const vector<string> &input) {
    cout << "----- Day 12 -----" << endl;
    cout << "Part 1: " << part1(input) << endl;
    cout << "Part 2: " << part2(input) << endl;
}
};  // namespace day12
