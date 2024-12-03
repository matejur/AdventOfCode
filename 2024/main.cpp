#include <fstream>
#include <iostream>
#include <string>

#include "all_days.cpp"

using namespace std;

int main(int argc, char *argv[]) {
    if (argc != 2) {
        cerr << "Usage: " << argv[0] << " <day>" << endl;
        return 1;
    }

    string day_string = argv[1];
    string input_path = "inputs/in" + day_string + ".txt";
    ifstream input(input_path);
    if (!input.is_open()) {
        cerr << "Could not open file: " << input_path << endl;
        return 1;
    }

    Runner *runner = get_runner(stoi(day_string));
    cout << "Part 1: " << runner->part1(input) << endl;

    input.clear();
    input.seekg(0);

    cout << "Part 2: " << runner->part2(input) << endl;
}