#include <fstream>
#include <functional>
#include <iostream>
#include <map>
#include <string>

#include "solutions/day01.h"
#include "solutions/day02.h"
#include "solutions/day03.h"
// SED MARKER 1

using namespace std;

map<int, function<void(const vector<string>)>> days = {
    {1, day01::run},
    {2, day02::run},
    {3, day03::run},
};  // SED MARKER 2

void run_day(int day) {
    string input_path = "inputs/in" + to_string(day) + ".txt";
    ifstream input_file(input_path);
    if (!input_file.is_open()) {
        cerr << "Could not open file: " << input_path << endl;
        return;
    }

    vector<string> input;
    for (string line; getline(input_file, line);) {
        input.push_back(line);
    }

    auto it = days.find(day);
    if (it == days.end()) {
        cerr << "Day " << day << " not found" << endl;
        return;
    }

    it->second(input);
}

int main(int argc, char *argv[]) {
    if (argc != 2) {
        cerr << "Usage: " << argv[0] << " <day>" << endl;
        return 1;
    }

    string arg = argv[1];
    if (arg == "all") {
        for (int i = 1; i <= days.size(); i++) {
            run_day(i);
        }
    } else {
        run_day(stoi(arg));
    }
}