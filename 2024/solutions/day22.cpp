#include "day22.h"

#include <string.h>

#include <iostream>
#include <map>
#include <queue>
#include <set>
#include <string>
#include <vector>

using namespace std;
namespace day22 {
long evolve(long secret) {
    long mult = secret * 64;
    secret = (mult ^ secret) % 16777216;
    long div = secret / 32;
    secret = (secret ^ div);  //% 16777216;
    mult = secret * 2048;
    secret = (mult ^ secret) % 16777216;
    return secret;
}

string part1(const vector<string> &input) {
    long answer = 0;
    for (string line : input) {
        long secret = stol(line);
        for (int i = 0; i < 2000; i++) {
            secret = evolve(secret);
        }
        answer += secret;
    }
    return to_string(answer);
}

string part2(const vector<string> &input) {
    int sequenceAccum[1 << 20] = {0};

    int max = 0;
    for (string line : input) {
        bool seenSequences[1 << 20] = {false};

        long secret = stol(line);
        int prev1 = 0;
        int prev2 = 0;
        int prev3 = 0;
        int prev4 = 0;
        for (int i = 0; i < 2000; i++) {
            long next = evolve(secret);
            int price = next % 10;
            int prevPrice = secret % 10;
            int change = price - prevPrice;

            prev4 = prev3;
            prev3 = prev2;
            prev2 = prev1;
            prev1 = change + 9;

            if (i > 3) {
                int key = prev4 | prev3 << 5 | prev2 << 10 | prev1 << 15;
                if (!seenSequences[key]) {
                    seenSequences[key] = true;
                    sequenceAccum[key] += price;
                    if (sequenceAccum[key] > max) {
                        max = sequenceAccum[key];
                    }
                }
            }

            secret = next;
        }
    }

    return to_string(max);
}

void run(const vector<string> &input) {
    cout << "----- Day 22 -----" << endl;
    cout << "Part 1: " << part1(input) << endl;
    cout << "Part 2: " << part2(input) << endl;
}
};  // namespace day22
