#include <map>

#include "runner.cpp"
#include "solutions/day01.cpp"
#include "solutions/day02.cpp"

std::map<int, Runner *> runners = {
    {1, new Day01()},
    {2, new Day02()},
};

Runner *get_runner(int day) { return runners[day]; }