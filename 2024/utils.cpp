#include "utils.h"

#include <regex>
#include <sstream>
#include <string>

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

vector<long> extractAllNumbers(string line) {
    vector<long> numbers;

    for (int i = 0; i < line.size(); i++) {
        if (isdigit(line[i]) ||
            (line[i] == '-' && i != line.size() - 1 && isdigit(line[i + 1]))) {
            int j = i;
            while (j < line.size() && (isdigit(line[j]) || line[j] == '-')) {
                j++;
            }
            numbers.push_back(stol(line.substr(i, j - i)));
            i = j;
        }
    }

    return numbers;
}
