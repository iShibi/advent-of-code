#include <string>
#include <vector>
#include <fstream>
#include <iostream>

int main() {
  int report_size = 0;
  std::string binary_string;
  std::vector<int> bit_sum(12);
  std::ifstream input_file("input.txt");
  if (input_file.is_open()) {
    while (std::getline(input_file, binary_string)) {
      report_size++;
      for (int i = 0; i < binary_string.size(); i++) {
        int bit_value = ((int) binary_string[i]) - 48;
        bit_sum[i] += bit_value;
      }
    }
    input_file.close();
  } else {
    std::cout << "Unable to open the file";
  }
  std::string gamma_rate, epsilon_rate;
  int half_size_of_report = report_size / 2;
  for (int i = 0; i < bit_sum.size(); i++) {
    if (bit_sum[i] > half_size_of_report) {
      gamma_rate.push_back('1');
      epsilon_rate.push_back('0');
    } else {
      gamma_rate.push_back('0');
      epsilon_rate.push_back('1');
    }
  }
  std::cout << std::stoi(gamma_rate, 0, 2) * std::stoi(epsilon_rate, 0, 2) << std::endl;
  return 0;
}



// OLD SOLUTION:

// #include <string>
// #include <vector>
// #include <fstream>
// #include <iostream>

// struct Counter {
//   int zero, one;
// };s

// int main() {
//   std::string binary_string;
//   std::ifstream input_file("input.txt");
//   std::vector<Counter> bit_value_counter(12);
//   if (input_file.is_open()) {
//     while (std::getline(input_file, binary_string)) {
//       for (int i = 0; i < binary_string.size(); i++) {
//         if (binary_string[i] == '0') {
//           bit_value_counter[i].zero += 1;
//         } else if (binary_string[i] == '1') {
//           bit_value_counter[i].one += 1;
//         }
//       }
//     }
//     input_file.close();
//   } else {
//     std::cout << "Unable to open the file";
//   }
//   std::string gamma_rate, epsilon_rate;
//   for (int i = 0; i < bit_value_counter.size(); i++) {
//     struct Counter bit_count = bit_value_counter[i];
//     if (bit_count.one > bit_count.zero) {
//       gamma_rate.push_back('1');
//       epsilon_rate.push_back('0');
//     } else {
//       gamma_rate.push_back('0');
//       epsilon_rate.push_back('1');
//     }
//   }
//   std::cout << std::stoi(gamma_rate, 0, 2) * std::stoi(epsilon_rate, 0, 2) << std::endl;
//   return 0;
// }
