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

vector<string> splitString(const string line, string delimiter) {
    vector<string> parts;
    size_t pos_start = 0;
    size_t pos_end, delim_len = delimiter.length();

    while ((pos_end = line.find(delimiter, pos_start)) != string::npos) {
        string token = line.substr(pos_start, pos_end - pos_start);
        pos_start = pos_end + delim_len;
        parts.push_back(token);
    }

    parts.push_back(line.substr(pos_start));
    return parts;
}
