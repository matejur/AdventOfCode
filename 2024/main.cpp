#include <string>
#include <iostream>
#include <fstream>
#include "day01.cpp"

int main(int argc, char *argv[]) {
    if (argc != 2) {
        std::cerr << "Usage: " << argv[0] << " <day>" << std::endl;
        return 1;
    }

    std::string day = argv[1];
    if (day.size() == 1) {
        day = "0" + day;
    }

    std::string input_path = "inputs/in" + day + ".txt";
    std::ifstream input(input_path);
    if (!input.is_open()) {
        std::cerr << "Could not open file: " << input_path << std::endl;
        return 1;
    }

    std::cout << "Day " << day << " Part 1: " << part1(input) << std::endl;

    input.clear();
    input.seekg(0);

    std::cout << "Day " << day << " Part 2: " << part2(input) << std::endl;
}