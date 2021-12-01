#include <fstream>
#include <iostream>

int main() {
  int current_depth, previous_depth = -1, depth_increase_count = 0;
  std::ifstream input_file("input.txt");
  if (input_file.is_open()) {
    while (input_file >> current_depth) {
      if (previous_depth != -1 && current_depth > previous_depth) {
        depth_increase_count++;
      }
      previous_depth = current_depth;
    }
    input_file.close();
  } else {
    std::cout << "Unable to open the file";
  }
  std::cout << depth_increase_count << std::endl;
  return 0;
}
