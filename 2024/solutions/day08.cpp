#include "day08.h"

#include <iostream>
#include <map>
#include <set>
#include <string>
#include <vector>

using namespace std;
namespace day08 {
bool isInBounds(int x, int y, const vector<string> &input) {
    return x >= 0 && x < input[0].size() && y >= 0 && y < input.size();
}

int countAntinodes(const vector<string> &input, bool harmonics) {
    map<char, vector<tuple<int, int>>> antennas;
    for (int y = 0; y < input.size(); y++) {
        for (int x = 0; x < input[y].size(); x++) {
            char c = input[y][x];
            if (c == '.') continue;
            antennas[c].push_back({x, y});
        }
    }

    set<tuple<int, int>> antinodes;
    for (auto const &[_, positions] : antennas) {
        for (int i = 0; i < positions.size(); i++) {
            for (int j = i + 1; j < positions.size(); j++) {
                auto pos1 = positions[i];
                auto pos2 = positions[j];

                int x1 = get<0>(pos1);
                int y1 = get<1>(pos1);
                int x2 = get<0>(pos2);
                int y2 = get<1>(pos2);

                int dx = x2 - x1;
                int dy = y2 - y1;

                int factor = harmonics ? 0 : 1;
                while (isInBounds(x1 - dx * factor, y1 - dy * factor, input)) {
                    antinodes.insert({x1 - dx * factor, y1 - dy * factor});
                    factor++;

                    if (!harmonics) break;
                }

                factor = harmonics ? 0 : 1;
                while (isInBounds(x2 + dx * factor, y2 + dy * factor, input)) {
                    antinodes.insert({x2 + dx * factor, y2 + dy * factor});
                    factor++;

                    if (!harmonics) break;
                }
            }
        }
    }

    return antinodes.size();
}

string part1(const vector<string> &input) {
    return to_string(countAntinodes(input, false));
}

string part2(const vector<string> &input) {
    return to_string(countAntinodes(input, true));
}

void run(const vector<string> &input) {
    cout << "----- Day 8 -----" << endl;
    cout << "Part 1: " << part1(input) << endl;
    cout << "Part 2: " << part2(input) << endl;
}
};  // namespace day08
