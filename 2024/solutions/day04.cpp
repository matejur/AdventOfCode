#include "day04.h"

#include <iostream>
#include <string>
#include <vector>

using namespace std;
namespace day04 {

bool search_XMAS_dir(const vector<string> &grid, int x, int y, int dx, int dy) {
    int height = grid.size();
    int width = grid[0].size();

    string MAS = "MAS";

    for (int i = 0; i < 3; i++) {
        x += dx;
        y += dy;

        if (x < 0 || x >= width || y < 0 || y >= height) {
            return false;
        }

        if (grid[y][x] != MAS[i]) {
            return false;
        }
    }
    return true;
}

int search_XMAS(const vector<string> &grid, int x, int y) {
    int total = 0;

    for (int dy = -1; dy <= 1; dy++) {
        for (int dx = -1; dx <= 1; dx++) {
            if (dx == 0 && dy == 0) {
                continue;
            }

            if (search_XMAS_dir(grid, x, y, dx, dy)) {
                total++;
            }
        }
    }

    return total;
}

string part1(const vector<string> &input) {
    int total = 0;
    for (int y = 0; y < input.size(); y++) {
        for (int x = 0; x < input[y].size(); x++) {
            if (input[y][x] == 'X') {
                total += search_XMAS(input, x, y);
            }
        }
    }
    return to_string(total);
}

bool search_X_MAS(const vector<string> &grid, int x, int y) {
    string corners = string() + grid[y - 1][x - 1] + grid[y - 1][x + 1] +
                     grid[y + 1][x + 1] + grid[y + 1][x - 1];

    return corners == "MMSS" || corners == "MSSM" || corners == "SSMM" ||
           corners == "SMMS";
}

string part2(const vector<string> &input) {
    int total = 0;
    for (int y = 1; y < input.size() - 1; y++) {
        for (int x = 1; x < input[y].size() - 1; x++) {
            if (input[y][x] == 'A') {
                total += search_X_MAS(input, x, y);
            }
        }
    }
    return to_string(total);
}

void run(const vector<string> &input) {
    cout << "----- Day 4 -----" << endl;
    cout << "Part 1: " << part1(input) << endl;
    cout << "Part 2: " << part2(input) << endl;
}
};  // namespace day04
