#include "day17.h"

#include <iostream>
#include <set>
#include <string>
#include <vector>

#include "../utils.h"

using namespace std;
namespace day17 {
long registerA = 0;
long registerB = 0;
long registerC = 0;

long getComboOperand(int opcode) {
    if (opcode < 4) {
        return opcode;
    } else if (opcode == 4) {
        return registerA;
    } else if (opcode == 5) {
        return registerB;
    } else if (opcode == 6) {
        return registerC;
    }

    return 0;
}

string join(vector<long> v) {
    string out = "";
    for (int i = 0; i < v.size(); i++) {
        out += to_string(v[i]);
        if (i != v.size() - 1) {
            out += ",";
        }
    }
    return out;
}

string part1(vector<long> program) {
    int ip = 0;
    vector<long> output;
    while (ip < program.size() - 1) {
        int opcode = program[ip];
        int operand = program[ip + 1];
        long numerator;
        long denominator;

        switch (opcode) {
            case 0:
                numerator = registerA;
                denominator = 1 << getComboOperand(operand);
                registerA = numerator / denominator;
                break;
            case 1:
                registerB ^= operand;
                break;
            case 2:
                registerB = getComboOperand(operand) % 8;
                break;
            case 3:
                if (registerA != 0) {
                    ip = operand;
                    continue;
                }
            case 4:
                registerB ^= registerC;
                break;
            case 5:
                output.push_back(getComboOperand(operand) % 8);
                break;
            case 6:
                numerator = registerA;
                denominator = 1 << getComboOperand(operand);
                registerB = numerator / denominator;
            case 7:
                numerator = registerA;
                denominator = 1 << getComboOperand(operand);
                registerC = numerator / denominator;
                break;
            default:
                break;
        }
        ip += 2;
    }

    return join(output);
}

int backward(long a, int xor1, int xor2) {
    return ((a % 8) ^ xor1 ^ xor2 ^ (a >> ((a % 8) ^ xor1))) % 8;
}

string part2(vector<long> program) {
    int xor1 = -1;
    int xor2 = -1;
    for (int op = 0; op < program.size(); op += 2) {
        if (program[op] == 1) {
            if (xor1 == -1) {
                xor1 = program[op + 1];
            } else {
                xor2 = program[op + 1];
                break;
            }
        }
    }

    set<long> possible;
    possible.insert(0);

    for (int i = program.size() - 1; i >= 0; i--) {
        set<long> nextPossible;
        for (long a : possible) {
            for (int j = 0; j < 8; j++) {
                long next = (a << 3) + j;
                if (backward(next, xor1, xor2) == program[i]) {
                    nextPossible.insert(next);
                }
            }
        }
        possible = nextPossible;
    }

    long answer = *possible.begin();
    registerA = answer;
    if (part1(program) == join(program)) {
        return to_string(answer);
    }

    return "No solution found";
}

void run(const vector<string> &input) {
    registerA = extractAllNumbers(input[0])[0];
    vector<long> program = extractAllNumbers(input[4]);

    cout << "----- Day 17 -----" << endl;
    cout << "Part 1: " << part1(program) << endl;
    cout << "Part 2: " << part2(program) << endl;
}
};  // namespace day17
