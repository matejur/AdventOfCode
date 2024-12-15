#include "day15.h"

#include <iostream>
#include <map>
#include <string>
#include <vector>

using namespace std;
namespace day15 {
map<char, pair<int, int>> directions = {
    {'^', {0, -1}},
    {'v', {0, 1}},
    {'<', {-1, 0}},
    {'>', {1, 0}},
};
void moveBoxes(vector<string> &grid, int x, int y, const string &moves) {
    for (char move : moves) {
        int dx = directions[move].first;
        int dy = directions[move].second;

        int nextX = x + dx;
        int nextY = y + dy;

        if (grid[nextY][nextX] == '#') {
            continue;
        }

        if (grid[nextY][nextX] == '.') {
            grid[y][x] = '.';
            x = nextX;
            y = nextY;
            grid[y][x] = '@';
        }

        if (grid[nextY][nextX] == 'O') {
            bool canSlide = true;
            int numBoxes = 0;
            for (int b = 0; b < grid.size(); b++) {
                int bx = x + dx * b;
                int by = y + dy * b;
                if (grid[by][bx] == '#') {
                    canSlide = false;
                    break;
                }
                if (grid[by][bx] == '.') {
                    numBoxes = b;
                    break;
                }
            }

            if (canSlide) {
                grid[y][x] = '.';
                x = nextX;
                y = nextY;
                for (int b = 1; b < numBoxes; b++) {
                    int bx = x + dx * b;
                    int by = y + dy * b;
                    grid[by][bx] = 'O';
                }
                grid[y][x] = '@';
            }
        }
    }
}

int calculateGPS(const vector<string> &grid) {
    int GPS = 0;
    for (int y = 0; y < grid.size(); y++) {
        for (int x = 0; x < grid[y].size(); x++) {
            if (grid[y][x] == 'O' || grid[y][x] == '[') {
                GPS += 100 * y + x;
            }
        }
    }
    return GPS;
}

bool tryPush(vector<string> &grid, int x, int y, int dy) {
    if (grid[y][x] == '.') {
        return true;
    }
    int left;
    int right;
    if (grid[y][x] == '[') {
        left = x;
        right = x + 1;
    } else {
        left = x - 1;
        right = x;
    }

    if (grid[y + dy][left] == '#' || grid[y + dy][right] == '#') {
        return false;
    }

    return tryPush(grid, right, y + dy, dy) && tryPush(grid, left, y + dy, dy);
}

void push(vector<string> &grid, int x, int y, int dy) {
    int left;
    int right;
    if (grid[y][x] == '[') {
        left = x;
        right = x + 1;
    } else if (grid[y][x] == ']') {
        left = x - 1;
        right = x;
    } else {
        return;
    }

    push(grid, right, y + dy, dy);
    push(grid, left, y + dy, dy);
    grid[y][left] = '.';
    grid[y][right] = '.';
    grid[y + dy][left] = '[';
    grid[y + dy][right] = ']';
}

void moveBigBoxes(vector<string> &grid, int x, int y, const string &moves) {
    for (int i = 0; i < moves.size(); i++) {
        char move = moves[i];
        int dx = directions[move].first;
        int dy = directions[move].second;

        int nextX = x + dx;
        int nextY = y + dy;
        if (grid[nextY][nextX] == '#') {
            continue;
        }

        if (grid[nextY][nextX] == '.') {
            grid[y][x] = '.';
            x = nextX;
            y = nextY;
            grid[y][x] = '@';
        }

        if (grid[nextY][nextX] == '[' || grid[nextY][nextX] == ']') {
            // simpler case
            if (dy == 0) {
                bool canSlide = true;
                int numBoxes = 0;
                for (int b = 0; b < grid.size(); b++) {
                    int bx = x + dx * b;
                    int by = y + dy * b;
                    if (grid[by][bx] == '#') {
                        canSlide = false;
                        break;
                    }
                    if (grid[by][bx] == '.') {
                        numBoxes = b;
                        break;
                    }
                }

                if (canSlide) {
                    grid[y][x] = '.';
                    x = nextX;
                    y = nextY;
                    for (int b = 1; b < numBoxes; b++) {
                        int bx = x + dx * b;
                        int by = y + dy * b;
                        if (dx == 1) {
                            grid[by][bx] = b % 2 == 1 ? '[' : ']';
                        } else {
                            grid[by][bx] = b % 2 == 0 ? '[' : ']';
                        }
                    }
                    grid[y][x] = '@';
                }
            } else {
                if (tryPush(grid, nextX, nextY, dy)) {
                    push(grid, nextX, nextY, dy);
                    grid[y][x] = '.';
                    x = nextX;
                    y = nextY;
                    grid[y][x] = '@';
                }
            }
        }
    }
}

string part1(const vector<string> &input) {
    vector<string> grid;
    bool readingGrid = true;
    string moves;
    int startX = 0;
    int startY = 0;
    for (int y = 0; y < input.size(); y++) {
        string line = input[y];
        if (line.empty()) {
            readingGrid = false;
        }
        if (readingGrid) {
            grid.push_back(line);

            int x = line.find('@');
            if (x != string::npos) {
                startX = x;
                startY = y;
            }
        } else {
            moves += line;
        }
    }

    moveBoxes(grid, startX, startY, moves);

    return to_string(calculateGPS(grid));
}

string part2(const vector<string> &input) {
    vector<string> grid;
    bool readingGrid = true;
    string moves;
    int startX = 0;
    int startY = 0;
    for (int y = 0; y < input.size(); y++) {
        if (input[y].empty()) {
            readingGrid = false;
        }

        if (readingGrid) {
            string line;
            for (int x = 0; x < input[y].size(); x++) {
                char c = input[y][x];
                if (c == '#') {
                    line += "##";
                } else if (c == 'O') {
                    line += "[]";
                } else if (c == '@') {
                    line += "@.";
                    startX = 2 * x;
                    startY = y;
                } else if (c == '.') {
                    line += "..";
                }
            }
            grid.push_back(line);
        } else {
            moves += input[y];
        }
    }

    moveBigBoxes(grid, startX, startY, moves);

    return to_string(calculateGPS(grid));
}

void run(const vector<string> &input) {
    cout << "----- Day 15 -----" << endl;
    cout << "Part 1: " << part1(input) << endl;
    cout << "Part 2: " << part2(input) << endl;
}
};  // namespace day15
