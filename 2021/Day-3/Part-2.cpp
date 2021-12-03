#include <string>
#include <vector>
#include <fstream>
#include <iostream>

std::vector<std::string> create_report();
int find_most_common_bit_at_a_position(std::vector<std::string> &binary_strings_arr, int position);
int find_least_common_bit_at_a_position(std::vector<std::string> &binary_strings_arr, int position);
std::vector<std::string> find_binary_strings_with_given_bit_at_given_position(std::vector<std::string> &binary_strings_arr, char bit_value, int position);

int main() {
  std::vector<std::string> report = create_report();
  int size = report[0].size();
  std::vector<std::string> temp_arr_one = report, oxygen_arr, co2_arr;
  for (int i = 0; i < size; i++) {
    char most_common_bit_at_current_position = find_most_common_bit_at_a_position(temp_arr_one, i) == 1 ? '1' : '0';
    oxygen_arr = find_binary_strings_with_given_bit_at_given_position(temp_arr_one, most_common_bit_at_current_position, i);
    if (oxygen_arr.size() == 1) break;
    temp_arr_one = oxygen_arr;
  }
  temp_arr_one = report;
  for (int i = 0; i < size; i++) {
    char least_common_bit_at_current_position = find_least_common_bit_at_a_position(temp_arr_one, i) == 1 ? '1' : '0';
    co2_arr = find_binary_strings_with_given_bit_at_given_position(temp_arr_one, least_common_bit_at_current_position, i);
    if (co2_arr.size() == 1) break;
    temp_arr_one = co2_arr;
  }
  std::string oxygen_generator_rating = oxygen_arr.front();
  std::string co2_scrubber_rating = co2_arr.front();
  std::cout << std::stoi(oxygen_generator_rating, 0, 2) * std::stoi(co2_scrubber_rating, 0, 2) << std::endl;
  return 0;
}

std::vector<std::string> create_report() {
  std::string binary_string;
  std::vector<std::string> report;
  std::ifstream input_file("input.txt");
  if (input_file.is_open()) {
    while (std::getline(input_file, binary_string)) {
      report.push_back(binary_string);
    }
    input_file.close();
  } else {
    std::cout << "Unable to open the file";
  }
  return report;
}

int find_most_common_bit_at_a_position(std::vector<std::string> &binary_strings_arr, int position) {
  int bit_sum = 0;
  for (auto binary_string : binary_strings_arr) {
    int bit_value = ((int) binary_string[position]) - 48;
    bit_sum += bit_value;
  }
  if (bit_sum >= ((float)binary_strings_arr.size() / 2)) {
    return 1;
  } else {
    return 0;
  }
}

int find_least_common_bit_at_a_position(std::vector<std::string> &binary_strings_arr, int position) {
  int bit_sum = 0;
  for (auto binary_string : binary_strings_arr) {
    int bit_value = ((int) binary_string[position]) - 48;
    bit_sum += bit_value;
  }
  if (bit_sum >= ((float)binary_strings_arr.size() / 2)) {
    return 0;
  }
  return 1;
}

std::vector<std::string> find_binary_strings_with_given_bit_at_given_position(std::vector<std::string> &binary_strings_arr, char bit_value, int position) {
  std::vector<std::string> new_binary_strings_arr;
  for (auto binary_string : binary_strings_arr) {
    if (binary_string[position] == bit_value) {
      new_binary_strings_arr.push_back(binary_string);
    }
  }
  return new_binary_strings_arr;
}
