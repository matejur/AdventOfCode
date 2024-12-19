#ifndef UTILS_H
#define UTILS_H

#include <string>
#include <vector>

std::vector<int> parse_numbers(std::string line);
std::vector<int> parseNumbersDelimiter(std::string line, char delimiter);

std::vector<long> extractAllNumbers(std::string line);
std::vector<std::string> splitString(const std::string line,
                                     std::string delimiter);
#endif