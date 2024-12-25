#include "day24.hpp"
#include "utils/input_reader.hpp"
#include <__format/format_functions.h>
#include <__iterator/unreachable_sentinel.h>
#include <cstdio>
#include <deque>
#include <functional>
#include <iostream>
#include <random>
#include <regex>
#include <string>
#include <tuple>
#include <unordered_map>
#include <unordered_set>

// wire can be either on, off, or unknown
// which is 1, 0, or -1, respectively
typedef int wire_state;

enum class GateType { AND, OR, XOR };
typedef std::tuple<std::string, GateType, std::string, std::string> GATE;
typedef std::tuple<std::string, GateType, std::string> GATELOGIC;
template <> struct std::hash<GATELOGIC> {
  size_t
  operator()(const std::tuple<std::string, GateType, std::string> &p) const {
    return hash<std::string>()(std::get<0>(p)) ^
           (hash<GateType>()(std::get<1>(p)) << 1) ^
           (hash<std::string>()(std::get<2>(p)) << 2);
  }
};
namespace day24 {
std::regex init_wire_re("([a-z0-9]+): (1|0)");
std::regex gate_re("([a-z0-9]+) (AND|OR|XOR) ([a-z0-9]+) -> ([a-z0-9]+)");

std::unordered_map<std::string, wire_state>
get_initial_wires(const std::string &content) {
  std::unordered_map<std::string, wire_state> wires;
  // iterate over all matches for the content
  auto begin =
      std::sregex_iterator(content.begin(), content.end(), init_wire_re);
  auto end = std::sregex_iterator();
  for (std::sregex_iterator it = begin; it != end; ++it) {
    std::smatch match = *it;
    wires[match[1]] = std::stoi(match[2]);
  }
  return wires;
}
std::deque<std::tuple<std::string, GateType, std::string, std::string>>
parse_gates(const std::string &content) {
  std::deque<std::tuple<std::string, GateType, std::string, std::string>> gates;
  auto begin = std::sregex_iterator(content.begin(), content.end(), gate_re);
  auto end = std::sregex_iterator();
  for (std::sregex_iterator it = begin; it != end; ++it) {
    std::smatch match = *it;
    GateType gate_type;
    if (match[2] == "AND") {
      gate_type = GateType::AND;
    } else if (match[2] == "OR") {
      gate_type = GateType::OR;
    } else if (match[2] == "XOR") {
      gate_type = GateType::XOR;
    }
    gates.push_back(std::make_tuple(match[1], gate_type, match[3], match[4]));
  }
  return gates;
}
int evaluate_gate(wire_state left_val, wire_state right_val,
                  GateType gate_type) {
  if (gate_type == GateType::AND) {
    return left_val & right_val;
  } else if (gate_type == GateType::OR) {
    return left_val | right_val;
  } else if (gate_type == GateType::XOR) {
    return left_val ^ right_val;
  }
  __builtin_unreachable();
}
void charge_wire_network(std::unordered_map<std::string, wire_state> &wires,
                         std::deque<GATE> &gates) {
  while (!gates.empty()) {
    GATE gate = gates.front();
    gates.pop_front();
    std::string left = std::get<0>(gate);
    std::string right = std::get<2>(gate);
    wire_state left_val, right_val;
    if (!wires.contains(left)) {
      wires[left] = -1;
    }
    if (!wires.contains(right)) {
      wires[right] = -1;
    }
    if ((left_val = wires.at(left)) == -1 ||
        (right_val = wires.at(right)) == -1) {
      // both left and right have not been evaluated
      gates.push_back(gate);
      continue;
    }
    std::string output = std::get<3>(gate);
    wires[output] = evaluate_gate(left_val, right_val, std::get<1>(gate));
  }
}
long evaluate_z_wire(std::string wire_name) {
  // return the number representing the wire name
  // for example, if wire_name is "z00", then return 1
  // if wire_name is "z01", then return 2
  // if wire_name is "z02", then return 4
  // if wire_name is "z03", then return 8
  // if wire_name is "z04", then return 16
  return std::pow(2, std::stol(wire_name.substr(1, 2)));
}
std::unordered_map<GATELOGIC, std::string>
parse_gates2(const std::string &content) {
  std::unordered_map<GATELOGIC, std::string> gates;
  auto begin = std::sregex_iterator(content.begin(), content.end(), gate_re);
  auto end = std::sregex_iterator();
  for (std::sregex_iterator it = begin; it != end; ++it) {
    std::smatch match = *it;
    GateType gate_type;
    if (match[2] == "AND") {
      gate_type = GateType::AND;
    } else if (match[2] == "OR") {
      gate_type = GateType::OR;
    } else if (match[2] == "XOR") {
      gate_type = GateType::XOR;
    }
    if (match[1] > match[3]) {
      gates[std::make_tuple(match[1], gate_type, match[3])] = match[4];
    } else {
      gates[std::make_tuple(match[3], gate_type, match[1])] = match[4];
    }
  }
  return gates;
}
GATELOGIC build_gate(std::string a, std::string b, GateType gate_type) {
  if (a > b) {
    return std::make_tuple(a, gate_type, b);
  } else {
    return std::make_tuple(b, gate_type, a);
  }
}
std::string gate_name(char prefix, int idx) {
  // 0 padded, 2 width number
  return std::format("{}{:0>2}", prefix, idx);
}
std::unordered_map<std::string, GATELOGIC>
reverse_gate_lookup(const std::unordered_map<GATELOGIC, std::string> &gates) {
  std::unordered_map<std::string, GATELOGIC> reverse_lookup;
  for (auto [key, value] : gates) {
    reverse_lookup[value] = key;
  }
  return reverse_lookup;
}
std::string
find_gate(const std::unordered_map<std::string, GATELOGIC> &reverse_lookup,
          const GateType &gate_type, const std::string &key) {
  for (auto [out_name, value] : reverse_lookup) {
    if (std::get<1>(value) == gate_type &&
        (std::get<2>(value) == key || std::get<0>(value) == key)) {
      return out_name;
    }
  }
  return "";
}
void validate_gate_bit(
    std::unordered_map<GATELOGIC, std::string> &gates,
    std::unordered_map<std::string, GATELOGIC> &reverse_lookup,
    std::unordered_set<std::string> &mismatched_outputs,
    std::vector<std::string> &carries, int bit) {
  std::string xname = gate_name('x', bit);
  std::string yname = gate_name('y', bit);
  std::string zname = gate_name('z', bit);
  std::string carry_in = carries[carries.size() - 1];
  // get the main XOR of the A and B
  std::string xor_out = gates.at(build_gate(xname, yname, GateType::XOR));
  // then find the wire for the carry out
  // it is possible that this gate does not exist due to the mistmatch
  // so check for existence and if it doesn't, we know of the mismatch and can
  // solve again
  GATELOGIC sum_out_key = build_gate(xor_out, carry_in, GateType::XOR);
  std::string sum_wire;
  if (!gates.contains(sum_out_key)) {
    auto carry_out = reverse_lookup.at(zname);
    // the output bit is attached to the combination of the XOR out and
    // something else figure out which wire (left,right) matches XOR out and the
    // other wire is the mismatch with the carry in wire
    if (xor_out == std::get<0>(carry_out)) {
      mismatched_outputs.insert(carry_in);
      carry_in = std::get<2>(carry_out);
      mismatched_outputs.insert(carry_in);
    } else if (xor_out == std::get<2>(carry_out)) {
      mismatched_outputs.insert(carry_in);
      carry_in = std::get<0>(carry_out);
      mismatched_outputs.insert(carry_in);
    } else if (carry_in == std::get<0>(carry_out)) {
      // if neither the inputs for the reverse lookup match with the XOR out,
      // there is a mismatch with the XOR out wire
      mismatched_outputs.insert(xor_out);
      xor_out = std::get<2>(carry_out);
      mismatched_outputs.insert(xor_out);
    } else {
      mismatched_outputs.insert(xor_out);
      xor_out = std::get<0>(carry_out);
      mismatched_outputs.insert(xor_out);
    }
    sum_wire = gates.at(build_gate(xor_out, carry_in, GateType::XOR));
  } else {
    sum_wire = gates.at(sum_out_key);
  }
  // if the sum wire is not the expected z wire, then this is a mismatch
  if (sum_wire != zname) {
    mismatched_outputs.insert(sum_wire);
    mismatched_outputs.insert(zname);
  }
  // continue exploring the full-addder implementation, make sure there are no
  // more mismatches XOR out AND the carry in
  std::string xy_and = gates.at(build_gate(xname, yname, GateType::AND));
  std::string xor_carry_and =
      gates.at(build_gate(xor_out, carry_in, GateType::AND));
  // possible that the carry_out_xor does not exist, so check for it. In the
  // case it doesn't exist this is a mismatch, so we need to solve again
  GATELOGIC carry_out_xor_key =
      build_gate(xy_and, xor_carry_and, GateType::XOR);
  std::string carry_out_xor;
  if (!gates.contains(carry_out_xor_key)) {
    // in this case, there are likely gates in between these input wires and the
    // desired operation for output carry. so we need to search for the actual
    // gates in between
    std::string xy_and_output = find_gate(reverse_lookup, GateType::OR, xy_and);
    std::string xor_carry_and_output =
        find_gate(reverse_lookup, GateType::OR, xor_carry_and);

    // becomes the pair with the other if there is no such gate
    if (xor_carry_and_output == "") {
      mismatched_outputs.insert(xor_carry_and);
      GATELOGIC xy_and_output_inputs = reverse_lookup.at(xy_and_output);
      xor_carry_and = std::get<2>(xy_and_output_inputs) == xy_and
                          ? std::get<0>(xy_and_output_inputs)
                          : std::get<2>(xy_and_output_inputs);
      mismatched_outputs.insert(xor_carry_and);
    } else if (xy_and_output == "") {
      mismatched_outputs.insert(xy_and);
      GATELOGIC xor_carry_and_output_inputs =
          reverse_lookup.at(xor_carry_and_output);
      xy_and = std::get<2>(xor_carry_and_output_inputs) == xor_carry_and
                   ? std::get<0>(xor_carry_and_output_inputs)
                   : std::get<2>(xor_carry_and_output_inputs);
      mismatched_outputs.insert(xy_and);
    }
    carry_out_xor = gates.at(build_gate(xy_and, xor_carry_and, GateType::OR));
  } else {
    carry_out_xor = gates.at(carry_out_xor_key);
  }
  // finally, add this carry wire to the list of carries
  carries.push_back(carry_out_xor);
}
}; // namespace day24
namespace aoc {
using namespace day24;

void solve_day24_part1(const std::string &input_path) {
  std::string content = FileReader::readFile(input_path);
  size_t split_idx = content.find("\n\n");
  std::unordered_map<std::string, wire_state> wires =
      get_initial_wires(content.substr(0, split_idx));
  auto gate_queue = parse_gates(content.substr(split_idx + 2));
  charge_wire_network(wires, gate_queue);
  long output = 0;
  for (auto [name, value] : wires) {
    if (name[0] == 'z' && value == 1) {
      long val = evaluate_z_wire(name);
      output += val;
    }
  }

  std::cout << "\nPart 1: " << output << std::endl;
}
void solve_day24_part2(const std::string &input_path) {
  std::string content = FileReader::readFile(input_path);
  size_t split_idx = content.find("\n\n");
  int output_size = 0;
  std::unordered_map<GATELOGIC, std::string> gates =
      parse_gates2(content.substr(split_idx + 2));
  std::unordered_map<std::string, GATELOGIC> reverse_lookup =
      reverse_gate_lookup(gates);
  std::unordered_set<std::string> mismatched_outputs;
  std::vector<std::string> carry_wires;
  for (auto [_, out_name] : gates) {
    // only need to check output names as 'z' wires are purely outputs
    if (out_name[0] == 'z') {
      output_size++;
    }
  }
  // in a standard full-adder the carry is initialized with the value of A AND B
  carry_wires.push_back(gates.at(build_gate("x00", "y00", GateType::AND)));
  // start at 1, because the first carry wire is the carry from the previous
  for (int i = 1; i < output_size - 1; i++) {
    validate_gate_bit(gates, reverse_lookup, mismatched_outputs, carry_wires,
                      i);
    // hard code the number of mismatches expected
    if (mismatched_outputs.size() == 8) {
      break;
    }
  }
  std::vector<std::string> output(mismatched_outputs.begin(),
                                  mismatched_outputs.end());
  std::sort(output.begin(), output.end());
  std::string output_str = "";
  for (std::string out : output) {
    output_str += out;
    output_str += ",";
  }
  std::cout << "\nPart 2: " << output_str << std::endl;
}
}; // namespace aoc
