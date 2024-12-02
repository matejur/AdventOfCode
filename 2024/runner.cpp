#include <string>

class Runner {
   private:
    int day;

   public:
    Runner(int day) : day(day) {}
    virtual std::string part1(std::ifstream &input) {
        return "Day " + std::to_string(day) + " part 1 not implemented";
    }

    virtual std::string part2(std::ifstream &input) {
        return "Day " + std::to_string(day) + " part 2 not implemented";
    }
};