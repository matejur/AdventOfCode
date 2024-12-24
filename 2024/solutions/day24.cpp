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

string part2(const vector<string> &input) {
    auto gates = parseGates(input);

    // first, there is a half adder
    auto s0 = getOutput("x00", "y00", "XOR", gates);
    if (s0.empty() || s0 != "z00") {
        cout << "Problem with first half adder" << endl;
    }

    // check full adders?
    string prevCarryWire = getOutput("x00", "y00", "AND", gates);
    for (int i = 1; i < 45; i++) {
        string xStr = "x" + to_string(i);
        string yStr = "y" + to_string(i);
        if (i < 10) {
            xStr = "x0" + to_string(i);
            yStr = "y0" + to_string(i);
        }

        auto beforeSum = getOutput(xStr, yStr, "XOR", gates);
        auto beforeCarry1 = getOutput(xStr, yStr, "AND", gates);

        auto beforeCarry2 = getOutput(beforeSum, prevCarryWire, "AND", gates);
        auto sum = getOutput(beforeSum, prevCarryWire, "XOR", gates);

        if (sum[0] != 'z') {
            cout << endl << "Iteration " << i << endl;
            cout << xStr << " XOR " << yStr << "->" << beforeSum << endl;
            cout << xStr << " AND " << yStr << "->" << beforeCarry1 << endl;
            cout << beforeSum << " AND " << prevCarryWire << "->"
                 << beforeCarry2 << endl;
            cout << beforeSum << " XOR " << prevCarryWire << "->" << sum
                 << endl;
        }
        prevCarryWire = getOutput(beforeCarry1, beforeCarry2, "OR", gates);
        if (sum[0] != 'z') {
            cout << beforeCarry1 << " OR " << beforeCarry2 << "->"
                 << prevCarryWire << endl;

            cout << "Fix the input" << endl;
            exit(0);
        }
    }

    set<string> answer = {"z09", "gwh", "wgb", "wbw",
                          "z21", "rcb", "z39", "jct"};

    string ans = "";
    for (const string &wire : answer) {
        ans += wire + ",";
    }

    return ans.substr(0, ans.length() - 1);
}

void run(const vector<string> &input) {
    cout << "----- Day 24 -----" << endl;
    cout << "Part 1: " << part1(input) << endl;
    cout << "Part 2: " << part2(input) << " (SOLVED BY HAND!!!)" << endl;
}
};  // namespace day24
