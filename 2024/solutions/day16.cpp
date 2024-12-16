#include "day16.h"

#include <iostream>
#include <map>
#include <queue>
#include <set>
#include <string>
#include <tuple>
#include <vector>

using namespace std;
namespace day16 {
int dirs[4][2] = {{1, 0}, {0, 1}, {-1, 0}, {0, -1}};
int lowestScore(const vector<string> grid) {
    int startX = 1;
    int startY = grid.size() - 2;

    priority_queue<tuple<int, int, int, int>, vector<tuple<int, int, int, int>>,
                   greater<tuple<int, int, int, int>>>
        pq;

    pq.push(make_tuple(0, startX, startY, 0));

    set<tuple<int, int, int>> visited;

    while (!pq.empty()) {
        auto [score, x, y, dir] = pq.top();
        pq.pop();

        if (grid[y][x] == '#') {
            continue;
        }

        if (visited.count(make_tuple(x, y, dir))) {
            continue;
        }

        visited.insert(make_tuple(x, y, dir));

        if (grid[y][x] == 'E') {
            return score;
        }

        auto forward = dirs[dir];
        auto left = dirs[(dir + 1) % 4];
        auto right = dirs[(dir + 3) % 4];

        pq.push(make_tuple(score + 1, x + forward[0], y + forward[1], dir));
        pq.push(
            make_tuple(score + 1001, x + left[0], y + left[1], (dir + 1) % 4));
        pq.push(make_tuple(score + 1001, x + right[0], y + right[1],
                           (dir + 3) % 4));
    }
    return -1;
}
string part1(const vector<string> &input) {
    return to_string(lowestScore(input));
}

string part2(const vector<string> &input) { return "Not implemented"; }

void run(const vector<string> &input) {
    cout << "----- Day 16 -----" << endl;
    cout << "Part 1: " << part1(input) << endl;
    cout << "Part 2: " << part2(input) << endl;
}
};  // namespace day16
