#include <string>
#include <fstream>
#include <iostream>

int main() {
  std::string direction;
  int horizontal_position = 0, depth = 0, aim = 0; 
  std::ifstream input_file("input.txt");
  if (input_file.is_open()) {
    while (std::getline(input_file, direction)) {
      char direction_starts_with = direction.front();
      int distance = direction.back() - 48;   // get the real integer value from the ASCII value
      if (direction_starts_with == 'f') {
        horizontal_position += distance;
        depth += (aim * distance);
      } else if (direction_starts_with == 'u') {
        aim -= distance;
      } else if (direction_starts_with == 'd') {
        aim += distance;
      }
    }
    input_file.close();
  } else {
    std::cout << "Unable to open the file";
  }
  std::cout << horizontal_position * depth << std::endl;
  return 0;
}
