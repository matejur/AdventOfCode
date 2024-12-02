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