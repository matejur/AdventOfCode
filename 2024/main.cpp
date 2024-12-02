#include <fstream>
#include <iostream>
#include <string>

#include "all_days.cpp"

int main(int argc, char *argv[]) {
    if (argc != 2) {
        std::cerr << "Usage: " << argv[0] << " <day>" << std::endl;
        return 1;
    }

    std::string day_string = argv[1];
    if (day_string.size() == 1) {
        day_string = "0" + day_string;
    }

    std::string input_path = "inputs/in" + day_string + ".txt";
    std::ifstream input(input_path);
    if (!input.is_open()) {
        std::cerr << "Could not open file: " << input_path << std::endl;
        return 1;
    }

    Runner *runner = get_runner(std::stoi(day_string));
    std::cout << "Part 1: " << runner->part1(input) << std::endl;

    input.clear();
    input.seekg(0);

    std::cout << "Part 2: " << runner->part2(input) << std::endl;
}