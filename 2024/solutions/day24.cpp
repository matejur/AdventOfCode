#include "day24.h"

#include <iostream>
#include <map>
#include <set>
#include <string>
#include <vector>

using namespace std;
namespace day24 {
map<string, bool> parseWires(const vector<string> &input) {
    map<string, bool> wires;

    for (const string &line : input) {
        if (line.empty()) {
            break;
        }

        string name = line.substr(0, 3);
        bool value = line.substr(5) == "1";
        wires[name] = value;
    }

    return wires;
}

map<tuple<string, string, string>, string> parseGates(
    const vector<string> &input) {
    map<tuple<string, string, string>, string> gates;
    bool readingWires = true;

    for (const string &line : input) {
        if (line.empty()) {
            readingWires = false;
            continue;
        }

        if (readingWires) {
            continue;
        }

        int firstSpace = line.find(' ');
        string wire1 = line.substr(0, firstSpace);
        int secondSpace = line.find(' ', firstSpace + 1);
        string gate = line.substr(firstSpace + 1, secondSpace - firstSpace - 1);
        string wire2 = line.substr(secondSpace + 1, 3);
        string output = line.substr(line.length() - 3);

        gates[make_tuple(wire1, gate, wire2)] = output;
    }

    return gates;
}

string part1(const vector<string> &input) {
    auto wires = parseWires(input);
    auto gates = parseGates(input);

    bool changed = true;
    while (changed) {
        changed = false;
        for (const auto &[key, value] : gates) {
            string wire1 = get<0>(key);
            string gate = get<1>(key);
            string wire2 = get<2>(key);
            string output = value;

            if (!wires.count(wire1) || !wires.count(wire2)) {
                continue;
            }

            if (wires.count(output)) {
                continue;
            }

            bool val1 = wires[wire1];
            bool val2 = wires[wire2];
            changed = true;

            if (gate == "AND") {
                wires[output] = val1 && val2;
            } else if (gate == "OR") {
                wires[output] = val1 || val2;
            } else if (gate == "XOR") {
                wires[output] = val1 ^ val2;
            } else {
                cout << "Unknown gate: " << gate << endl;
            }
        }
    }

    long answer = 0;
    for (int i = 64; i >= 0; i--) {
        string wire = "z" + to_string(i);
        if (i < 10) {
            wire = "z0" + to_string(i);
        }

        bool bit = wires.count(wire) && wires[wire];
        answer = (answer << 1) | bit;
    }

    return to_string(answer);
}

string getOutput(string wire1, string wire2, string op,
                 map<tuple<string, string, string>, string> gates) {
    auto it1 = gates.find({wire1, op, wire2});
    if (it1 != gates.end()) {
        return it1->second;
    }
    auto it2 = gates.find({wire2, op, wire1});
    if (it2 != gates.end()) {
        return it2->second;
    }
    return "";
}

vector<string> forward(string x, string y, string c,
                       map<tuple<string, string, string>, string> gates) {
    auto beforeSum = getOutput(x, y, "XOR", gates);
    auto beforeCarry1 = getOutput(x, y, "AND", gates);

    auto beforeCarry2 = getOutput(beforeSum, c, "AND", gates);
    auto sum = getOutput(beforeSum, c, "XOR", gates);
    string carry = getOutput(beforeCarry1, beforeCarry2, "OR", gates);
    return {sum, carry, beforeSum, beforeCarry1, beforeCarry2};
}

string part2(const vector<string> &input) {
    auto gates = parseGates(input);

    map<string, tuple<string, string, string>> reverse;
    for (const auto &[key, value] : gates) {
        string wire1 = get<0>(key);
        string gate = get<1>(key);
        string wire2 = get<2>(key);
        string output = value;

        reverse[output] = key;
    }

    set<string> swaps;
    string prevCarryWire = getOutput("x00", "y00", "AND", gates);
    for (int i = 1; i < 45; i++) {
        string xStr = "x" + to_string(i);
        string yStr = "y" + to_string(i);
        string zStr = "z" + to_string(i);
        if (i < 10) {
            xStr = "x0" + to_string(i);
            yStr = "y0" + to_string(i);
            zStr = "z0" + to_string(i);
        }

        auto out = forward(xStr, yStr, prevCarryWire, gates);
        string sum = out[0];
        string carry = out[1];

        if (sum != zStr) {
            bool found = false;
            for (const string &w1 : out) {
                if (found) {
                    break;
                }
                for (const string &w2 : out) {
                    if (w1 == w2) {
                        continue;
                    }

                    auto it1 = reverse.find(w1);
                    auto it2 = reverse.find(w2);

                    if (it1 != reverse.end() && it2 != reverse.end()) {
                        auto key1 = it1->second;
                        auto key2 = it2->second;

                        gates[key1] = w2;
                        gates[key2] = w1;

                        auto out = forward(xStr, yStr, prevCarryWire, gates);
                        string sum = out[0];
                        string carry = out[1];
                        if (sum == zStr) {
                            swaps.insert(w1);
                            swaps.insert(w2);
                            prevCarryWire = carry;
                            found = true;
                            break;
                        }
                        gates[key1] = w1;
                        gates[key2] = w2;
                    }
                }
            }
        } else {
            prevCarryWire = carry;
        }
    }

    if (swaps.size() != 8) {
        return "Failed to find solution with this implementation";
    }

    string ans = "";
    for (const string &wire : swaps) {
        ans += wire + ",";
    }

    return ans.substr(0, ans.length() - 1);
}

void run(const vector<string> &input) {
    cout << "----- Day 24 -----" << endl;
    cout << "Part 1: " << part1(input) << endl;
    cout << "Part 2: " << part2(input) << endl;
}
};  // namespace day24
