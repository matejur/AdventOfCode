#include "day09.h"

#include <iostream>
#include <list>
#include <map>
#include <string>
#include <vector>

using namespace std;
namespace day09 {
string part1(const vector<string> &input) {
    const string diskMap = input[0];

    int memorySize = 0;
    for (const char &c : diskMap) {
        memorySize += c - '0';
    }

    int memory[memorySize];

    int fileID = 0;
    int memoryIndex = 0;
    vector<int> freeSpaceIndex;
    for (int i = 0; i < diskMap.size(); i += 2) {
        int fileSize = diskMap[i] - '0';
        int emptySpace = diskMap[i + 1] - '0';

        for (int j = 0; j < fileSize; j++) {
            memory[memoryIndex] = fileID;
            memoryIndex += 1;
        }

        for (int j = 0; j < emptySpace; j++) {
            freeSpaceIndex.push_back(memoryIndex);
            memory[memoryIndex] = -1;
            memoryIndex += 1;
        }

        fileID += 1;
    }

    for (int blankIndex : freeSpaceIndex) {
        while (memory[memorySize - 1] == -1) {
            memorySize -= 1;
        }
        if (memorySize < blankIndex) {
            break;
            return "Not implemented";
        }

        memory[blankIndex] = memory[memorySize - 1];
        memory[memorySize - 1] = -1;
    }

    long answer = 0;
    for (int i = 0; i < memorySize; i++) {
        answer += memory[i] * i;
    }

    return to_string(answer);
}

string part2(const vector<string> &input) {
    const string diskMap = input[0];

    map<int, tuple<int, int>> fileMap;
    vector<tuple<int, int>> freeSpaceList;

    int fileID = 0;
    int memoryIndex = 0;
    for (int i = 0; i < diskMap.size(); i += 2) {
        int fileSize = diskMap[i] - '0';
        int emptySpace = diskMap[i + 1] - '0';

        fileMap[fileID] = make_tuple(memoryIndex, fileSize);
        memoryIndex += fileSize;

        if (emptySpace > 0) {
            freeSpaceList.push_back(make_tuple(memoryIndex, emptySpace));
            memoryIndex += emptySpace;
        }

        fileID += 1;
    }

    fileID--;
    while (fileID > 0) {
        tuple<int, int> file = fileMap[fileID];
        int filePosition = get<0>(file);
        int fileSize = get<1>(file);

        for (int i = 0; i < freeSpaceList.size(); i++) {
            tuple<int, int> freeSpace = freeSpaceList[i];
            int freePosition = get<0>(freeSpace);
            int freeSize = get<1>(freeSpace);

            if (freePosition > filePosition) {
                freeSpaceList.resize(i);
                continue;
            }

            if (freeSize >= fileSize) {
                freeSpaceList[i] =
                    make_tuple(freePosition + fileSize, freeSize - fileSize);
                fileMap[fileID] = make_tuple(freePosition, fileSize);
                break;
            }
        }

        fileID--;
    }

    long answer = 0;
    for (int i = 0; i < fileMap.size(); i++) {
        tuple<int, int> file = fileMap[i];
        int memoryIndex = get<0>(file);
        int fileSize = get<1>(file);

        for (int j = 0; j < fileSize; j++) {
            answer += (memoryIndex + j) * i;
        }
    }

    return to_string(answer);
}

void run(const vector<string> &input) {
    cout << "----- Day 9 -----" << endl;
    cout << "Part 1: " << part1(input) << endl;
    cout << "Part 2: " << part2(input) << endl;
}
};  // namespace day09
