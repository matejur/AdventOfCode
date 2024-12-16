#include "day16.h"

#include <iomanip>
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
pair<int, int> solve(const vector<string> grid) {
    int startX = 1;
    int startY = grid.size() - 2;

    priority_queue<tuple<int, int, int, int>, vector<tuple<int, int, int, int>>,
                   greater<tuple<int, int, int, int>>>
        pq;

    pq.push(make_tuple(0, startX, startY, 0));

    map<tuple<int, int, int>, int> scores;

    while (!pq.empty()) {
        auto [score, x, y, dir] = pq.top();
        pq.pop();

        if (grid[y][x] == '#') {
            continue;
        }

        if (scores.count(make_tuple(x, y, dir))) {
            continue;
        }

        scores[{x, y, dir}] = score;

        if (grid[y][x] == 'E') {
            queue<tuple<int, int, int>> q;
            q.push({x, y, dir});

            set<pair<int, int>> path;

            while (!q.empty()) {
                auto curr = q.front();
                auto [x, y, dir] = curr;
                q.pop();

                int currScore = scores[curr];

                if (path.count({x, y})) {
                    continue;
                }
                path.insert({x, y});

                for (int d = 0; d < 4; d++) {
                    int newX = x - dirs[d][0];
                    int newY = y - dirs[d][1];

                    if (grid[newY][newX] == '#') {
                        continue;
                    }

                    if (scores.count({newX, newY, d})) {
                        int diff = currScore - scores[{newX, newY, d}];
                        if (diff == 1 || diff == 1001) {
                            q.push({newX, newY, d});
                        }
                    }
                }
            }

            return {score, path.size()};
        }

        auto forward = dirs[dir];

        pq.push(make_tuple(score + 1, x + dirs[dir][0], y + forward[1], dir));
        pq.push(make_tuple(score + 1000, x, y, (dir + 1) % 4));
        pq.push(make_tuple(score + 1000, x, y, (dir + 3) % 4));
    }

    return {-1, -1};
}

void run(const vector<string> &input) {
    cout << "----- Day 16 -----" << endl;
    auto [part1, part2] = solve(input);
    cout << "Part 1: " << part1 << endl;
    cout << "Part 2: " << part2 << endl;
}
};  // namespace day16
