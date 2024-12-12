#include "day12.h"

#include <iostream>
#include <map>
#include <queue>
#include <set>
#include <string>
#include <vector>

using namespace std;
namespace day12 {

int calculateSides(const set<pair<int, int>> &fences) {
    int corners = 0;

    return corners;
}
int calculatePrice(const vector<string> &input, int x, int y, bool *visited,
                   bool part2) {
    set<pair<int, int>> fences;
    queue<pair<int, int>> q;
    q.push({x, y});

    char region = input[y][x];
    int area = 0;
    int perimeter = 0;
    int corners = 0;

    while (!q.empty()) {
        auto [x, y] = q.front();
        q.pop();

        if (x < 0 || x >= input[y].size() || y < 0 || y >= input.size() ||
            input[y][x] != region) {
            perimeter++;
            fences.insert({x, y});
            continue;
        }

        if (visited[y * input[y].size() + x]) {
            continue;
        }

        visited[y * input[y].size() + x] = true;
        area++;

        q.push({x + 1, y});
        q.push({x - 1, y});
        q.push({x, y + 1});
        q.push({x, y - 1});
    }

    if (part2) {
        perimeter = calculateSides(fences);
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
