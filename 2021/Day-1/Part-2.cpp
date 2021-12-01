#include <vector>
#include <fstream>
#include <iostream>

int main() {
  int depth, sum_increased_count = 0;
  std::vector<int> depths;
  std::ifstream input_file("input.txt");
  if (input_file.is_open()) {
    while (input_file >> depth) {
      depths.push_back(depth);
    }
    input_file.close();
  } else {
    std::cout << "Unable to open the file";
  }
  int i = 0, j = i + 1, depth_count = depths.size();
  while (j + 2 < depth_count) {
    int current_window = depths[i] + depths[i+1] + depths[i+2];
    int next_window = depths[j] + depths[j+1] + depths[j+2];
    if (next_window > current_window) {
      sum_increased_count++;
    }
    i++;
    j++;
  }
  std::cout << sum_increased_count << std::endl;
  return 0;
}
