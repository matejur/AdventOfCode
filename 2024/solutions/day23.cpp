#include "day23.h"

#include <algorithm>
#include <iostream>
#include <map>
#include <queue>
#include <set>
#include <string>
#include <vector>

using namespace std;
namespace day23 {
map<string, set<string>> parseNetwork(const vector<string> &input) {
    map<string, set<string>> network;
    for (const string &line : input) {
        string pc1 = line.substr(0, 2);
        string pc2 = line.substr(3, 2);
        network[pc1].insert(pc2);
        network[pc2].insert(pc1);
    }
    return network;
}
string part1(const vector<string> &input) {
    auto network = parseNetwork(input);

    int answer = 0;
    for (const auto &[pc, connections] : network) {
        int triangles = 0;
        for (const string neighbor : connections) {
            for (const string neighbor2 : network[neighbor]) {
                if (pc[0] != 't' && neighbor[0] != 't' && neighbor2[0] != 't') {
                    continue;
                }
                if (find(connections.begin(), connections.end(), neighbor2) !=
                    connections.end()) {
                    triangles++;
                }
            }
        }
        answer += triangles;
    }

    return to_string(answer / 6);
}

// https://en.wikipedia.org/wiki/Bron%E2%80%93Kerbosch_algorithm
// WITH PIVOTING
void bronKerbosch(set<string> &R, set<string> &P, set<string> &X,
                  map<string, set<string>> &network,
                  set<string> &largestClique) {
    if (P.empty() && X.empty()) {
        if (R.size() > largestClique.size()) {
            largestClique = R;
        }
        return;
    }

    set<string> PunionX = P;
    PunionX.insert(X.begin(), X.end());

    string pivot = *PunionX.begin();

    set<string> PminusN;
    for (const string p : P) {
        if (network[pivot].find(p) == network[pivot].end()) {
            PminusN.insert(p);
        }
    }

    while (!PminusN.empty()) {
        string pc = *PminusN.begin();
        set<string> newR = R;
        newR.insert(pc);

        set<string> newP;
        for (const string p : P) {
            if (network[pc].find(p) != network[pc].end()) {
                newP.insert(p);
            }
        }

        set<string> newX;
        for (const string x : X) {
            if (network[pc].find(x) != network[pc].end()) {
                newX.insert(x);
            }
        }

        bronKerbosch(newR, newP, newX, network, largestClique);

        PminusN.erase(pc);
        X.insert(pc);
    }
}

string part2(const vector<string> &input) {
    auto network = parseNetwork(input);

    set<string> R;
    set<string> X;
    set<string> P;
    for (const auto &[pc, connections] : network) {
        P.insert(pc);
    }

    set<string> largestClique;
    bronKerbosch(R, P, X, network, largestClique);

    string answer = "";
    for (const string &pc : largestClique) {
        answer += pc + ",";
    }
    return answer.substr(0, answer.size() - 1);
}

void run(const vector<string> &input) {
    cout << "----- Day 23 -----" << endl;
    cout << "Part 1: " << part1(input) << endl;
    cout << "Part 2: " << part2(input) << endl;
}
};  // namespace day23
