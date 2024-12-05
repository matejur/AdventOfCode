#include "utils.h"

#include <sstream>
#include <string>
#include <vector>

using namespace std;

vector<int> parse_numbers(string line) {
    vector<int> numbers;
    stringstream ss(line);
    int number;
    while (ss >> number) {
        numbers.push_back(number);
    }
    return numbers;
}

vector<int> parseNumbersDelimiter(string line, char delimiter) {
    vector<int> parts;
    stringstream ss(line);
    string part;
    while (getline(ss, part, delimiter)) {
        parts.push_back(stoi(part));
    }
    return parts;
}
