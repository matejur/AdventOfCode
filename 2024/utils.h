#ifndef UTILS_H
#define UTILS_H

#include <string>
#include <vector>

std::vector<int> parse_numbers(std::string line);
std::vector<int> parseNumbersDelimiter(std::string line, char delimiter);

#endif