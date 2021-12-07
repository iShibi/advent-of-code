
#include <vector>
#include <fstream>
#include <stdlib.h>
#include <iostream>
#include <algorithm>

std::vector<int> read_input();

int main() {
  std::vector<int> crab_positions = read_input();
  int crab_count = crab_positions.size();
  std::sort(crab_positions.begin(), crab_positions.end());
  int median = crab_positions[crab_count/2];
  int fuel_cost = 0;
  for (auto position : crab_positions) {
    fuel_cost += std::abs(position - median);
  }
  std::cout << fuel_cost << std::endl;
  return 0;
}

std::vector<int> read_input() {
  std::vector<int> crab_positions;
  std::ifstream input_file("input.txt");
  int position;
  if (input_file.is_open()) {
    while (input_file >> position) {
      crab_positions.push_back(position);
      if (input_file.peek() == ',') {
        input_file.ignore();
      }
    }
  } else {
    std::cout << "Unable to open the file" << std::endl;
  }
  return crab_positions;
}