pub fn solve(input_text: &String) -> u32 {
	let steps = 1000;
	let mut moons = vec![];
	let initial_raw_positions = input_text
		.split(|char| char == '\n')
		.filter(|str| !str.is_empty())
		.collect::<Vec<_>>();
	for raw_position in initial_raw_positions {
		let position: Vector3D = raw_position
			.split(|char: char| {
				char == ',' || char == '<' || char == '>' || char == '=' || char.is_whitespace()
			})
			.filter_map(|str| str.parse::<i32>().ok())
			.collect::<Vec<_>>()
			.into();
		let moon = Moon::new(position);
		moons.push(moon);
	}
	for _ in 1..=steps {
		update_moon_velocities(&mut moons);
		update_moon_positions(&mut moons);
	}
	let mut total_energy_of_system: u32 = 0;
	for moon in &moons {
		total_energy_of_system += moon.get_total_energy();
	}
	total_energy_of_system
}

fn update_moon_velocities(moons: &mut Vec<Moon>) {
	for idx_a in 0..moons.len() - 1 {
		let mut moon_a = moons[idx_a];
		for idx_b in (idx_a + 1)..moons.len() {
			let mut moon_b = moons[idx_b];
			// Update X-axis
			if moon_a.position.x > moon_b.position.x {
				moon_a.velocity.x -= 1;
				moon_b.velocity.x += 1;
			} else if moon_a.position.x < moon_b.position.x {
				moon_a.velocity.x += 1;
				moon_b.velocity.x -= 1;
			}
			// Update Y-axis
			if moon_a.position.y > moon_b.position.y {
				moon_a.velocity.y -= 1;
				moon_b.velocity.y += 1;
			} else if moon_a.position.y < moon_b.position.y {
				moon_a.velocity.y += 1;
				moon_b.velocity.y -= 1;
			}
			// Update Z-axis
			if moon_a.position.z > moon_b.position.z {
				moon_a.velocity.z -= 1;
				moon_b.velocity.z += 1;
			} else if moon_a.position.z < moon_b.position.z {
				moon_a.velocity.z += 1;
				moon_b.velocity.z -= 1;
			}
			moons[idx_a] = moon_a;
			moons[idx_b] = moon_b;
		}
	}
}

fn update_moon_positions(moons: &mut Vec<Moon>) {
	for moon in moons {
		moon.position.x += moon.velocity.x;
		moon.position.y += moon.velocity.y;
		moon.position.z += moon.velocity.z;
	}
}

#[derive(Debug, Clone, Copy)]
struct Moon {
	position: Vector3D,
	velocity: Vector3D,
}

impl Moon {
	fn new(position: Vector3D) -> Self {
		Self {
			position,
			velocity: Vector3D::default(),
		}
	}
	fn get_potential_energy(&self) -> u32 {
		let Vector3D { x, y, z } = self.position;
		(x.abs() + y.abs() + z.abs()) as u32
	}
	fn get_kinetic_energy(&self) -> u32 {
		let Vector3D { x, y, z } = self.velocity;
		(x.abs() + y.abs() + z.abs()) as u32
	}
	fn get_total_energy(&self) -> u32 {
		self.get_potential_energy() * self.get_kinetic_energy()
	}
}

#[derive(Debug, Clone, Copy)]
struct Vector3D {
	x: i32,
	y: i32,
	z: i32,
}

impl Default for Vector3D {
	fn default() -> Self {
		Self { x: 0, y: 0, z: 0 }
	}
}

impl From<Vec<i32>> for Vector3D {
	fn from(value: Vec<i32>) -> Self {
		Self {
			x: value[0],
			y: value[1],
			z: value[2],
		}
	}
}
