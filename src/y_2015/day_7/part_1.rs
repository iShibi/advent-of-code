use std::collections::HashMap;

pub fn solve(input_text: &String) -> u16 {
	let instructions = input_text.split(|c| c == '\n').filter(|s| !s.is_empty()).collect::<Vec<_>>();
	let mut wires = HashMap::<String, u16>::new();
	let mut unknowns = HashMap::<_, bool>::new();
	loop {
		for instruction in instructions.clone() {
			let tokens = instruction.split_whitespace().map(|s| s.to_string()).collect::<Vec<String>>();
			if instruction.contains("AND") {
				if let [first_wire, _, second_wire, _, output_wire] = &(tokens)[..] {
					if let Some(first_wire_value) = wires.get(first_wire).copied().or(first_wire.parse::<u16>().ok()) {
						if let Some(second_wire_value) =
							wires.get(second_wire).copied().or(second_wire.parse::<u16>().ok())
						{
							let output = first_wire_value & second_wire_value;
							wires.insert(output_wire.to_string(), output);
							unknowns.remove(output_wire);
						} else {
							unknowns.insert(second_wire.to_string(), true);
						}
					} else {
						unknowns.insert(first_wire.to_string(), true);
					}
				}
			} else if instruction.contains("OR") {
				if let [first_wire, _, second_wire, _, output_wire] = &(tokens)[..] {
					if let Some(first_wire_value) = wires.get(first_wire).copied().or(first_wire.parse::<u16>().ok()) {
						if let Some(second_wire_value) =
							wires.get(second_wire).copied().or(second_wire.parse::<u16>().ok())
						{
							let output = first_wire_value | second_wire_value;
							wires.insert(output_wire.to_string(), output);
							unknowns.remove(output_wire);
						} else {
							unknowns.insert(second_wire.to_string(), true);
						}
					} else {
						unknowns.insert(first_wire.to_string(), true);
					}
				}
			} else if instruction.contains("LSHIFT") {
				if let [first_wire, _, shift_by, _, output_wire] = &(tokens)[..] {
					if let Some(first_wire_value) = wires.get(first_wire).copied().or(first_wire.parse::<u16>().ok()) {
						let output = first_wire_value << shift_by.parse::<u16>().unwrap();
						wires.insert(output_wire.to_string(), output);
						unknowns.remove(output_wire);
					} else {
						unknowns.insert(first_wire.to_string(), true);
					}
				}
			} else if instruction.contains("RSHIFT") {
				if let [first_wire, _, shift_by, _, output_wire] = &(tokens)[..] {
					if let Some(first_wire_value) = wires.get(first_wire).copied().or(first_wire.parse::<u16>().ok()) {
						let output = first_wire_value >> shift_by.parse::<u16>().unwrap();
						wires.insert(output_wire.to_string(), output);
						unknowns.remove(output_wire);
					} else {
						unknowns.insert(first_wire.to_string(), true);
					}
				}
			} else if instruction.contains("NOT") {
				if let [_, first_wire, _, output_wire] = &(tokens)[..] {
					if let Some(first_wire_value) = wires.get(first_wire).copied().or(first_wire.parse::<u16>().ok()) {
						let output = !first_wire_value;
						wires.insert(output_wire.to_string(), output);
						unknowns.remove(output_wire);
					} else {
						unknowns.insert(first_wire.to_string(), true);
					}
				}
			} else {
				if let [first_wire, _, output_wire] = &(tokens)[..] {
					if let Some(first_wire_value) = wires.get(first_wire).copied().or(first_wire.parse::<u16>().ok()) {
						let output = first_wire_value;
						wires.insert(output_wire.to_string(), output);
						unknowns.remove(output_wire);
					} else {
						unknowns.insert(first_wire.to_string(), true);
					}
				}
			}
		}
		if unknowns.is_empty() {
			break;
		}
	}
	wires.get("a").unwrap().clone()
}
