#include <chrono>
#include <fstream>
#include <functional>
#include <iostream>
#include <map>
#include <string>

#include "solutions/day01.h"
#include "solutions/day02.h"
#include "solutions/day03.h"
#include "solutions/day04.h"
#include "solutions/day05.h"
#include "solutions/day06.h"
#include "solutions/day07.h"
#include "solutions/day08.h"
#include "solutions/day09.h"
#include "solutions/day10.h"
#include "solutions/day11.h"
#include "solutions/day12.h"
#include "solutions/day13.h"
#include "solutions/day14.h"
#include "solutions/day15.h"
#include "solutions/day16.h"
#include "solutions/day17.h"
#include "solutions/day18.h"
#include "solutions/day19.h"
// SED MARKER 1

using namespace std;

map<int, function<void(const vector<string>)>> days = {
    {1, day01::run},  {2, day02::run},  {3, day03::run},  {4, day04::run},
    {5, day05::run},  {6, day06::run},  {7, day07::run},  {8, day08::run},
    {9, day09::run},  {10, day10::run}, {11, day11::run}, {12, day12::run},
    {13, day13::run}, {14, day14::run}, {15, day15::run}, {16, day16::run},
    {17, day17::run}, {18, day18::run},
    {19, day19::run},
};  // SED MARKER 2

void run_day(int day, bool example) {
    vector<string> input;
    if (example) {
        string example_path = "inputs/in" + to_string(day) + "_ex.txt";
        ifstream example_file(example_path);
        if (!example_file.is_open()) {
            cout << "Paste the example input for day " << day
                 << " and press Ctrl+D" << endl;
            for (string line; getline(cin, line);) {
                input.push_back(line);
            }
            cout << endl;

            // Write the input to the example file
            ofstream example_file(example_path);
            for (const string &line : input) {
                example_file << line << endl;
            }
        } else {
            for (string line; getline(example_file, line);) {
                input.push_back(line);
            }
        }
    } else {
        string input_path = "inputs/in" + to_string(day) + ".txt";
        ifstream input_file(input_path);
        if (!input_file.is_open()) {
            cerr << "Could not open file: " << input_path << endl;
            return;
        }

        for (string line; getline(input_file, line);) {
            input.push_back(line);
        }
    }

    auto it = days.find(day);
    if (it == days.end()) {
        cerr << "Day " << day << " not found" << endl;
        return;
    }

    auto start = chrono::high_resolution_clock::now();
    it->second(input);
    auto end = chrono::high_resolution_clock::now();
    auto time =
        chrono::duration_cast<chrono::milliseconds>(end - start).count();
    cout << "Time: " << time << "ms" << endl;
}

int main(int argc, char *argv[]) {
    if (argc == 1) {
        cerr << "Running all days" << endl;
        for (int i = 1; i <= days.size(); i++) {
            run_day(i, false);
        }
        return 0;
    }

    if (argc == 2) {
        try {
            int day = stoi(argv[1]);
            run_day(day, false);
        } catch (invalid_argument &e) {
            cerr << "Invalid day: " << argv[1] << endl;
        }
    } else {
        string flag = argv[2];
        try {
            int day = stoi(argv[1]);
            run_day(day, flag == "-e");
        } catch (invalid_argument &e) {
            cerr << "Invalid day: " << argv[1] << endl;
        }
    }
}