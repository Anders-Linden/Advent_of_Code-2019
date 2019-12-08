use std::io::prelude::*;
extern crate utilities as utils;
use std::collections::BTreeMap;

use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::iter::FromIterator;

#[derive(Debug, Copy, Clone)]
struct Point {
	x: i32,
	y: i32,
}

impl Point {
	fn manhattan_distance(&self, other: &Point) -> u16 {
		//The distance between two points in a grid like street network
		((self.x - other.x).abs() + (self.y - other.y).abs()) as u16
	}
}

impl PartialEq for Point {
	fn eq(&self, other: &Self) -> bool {
		self.x == other.x && self.y == other.y
	}
}

impl Hash for Point {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.x.hash(state);
		self.y.hash(state);
	}
}

impl Eq for Point {}

enum Direction {
	Upp,
	Down,
	Right,
	Left,
}

fn walk(line: &mut Vec<Point>, distance: i32, direction: Direction) {
	let latest = *line.last().unwrap();
	for step in 1..distance + 1 {
		match direction {
			Direction::Right => line.push(Point {
				x: latest.x + step,
				y: latest.y,
			}),
			Direction::Left => line.push(Point {
				x: latest.x - step,
				y: latest.y,
			}),
			Direction::Upp => line.push(Point {
				x: latest.x,
				y: latest.y + step,
			}),
			Direction::Down => line.push(Point {
				x: latest.x,
				y: latest.y - step,
			}),
		}
	}
}

fn get_line(origin: Point, data: &String) -> Vec<Point> {
	let wire_circuit_desc: Vec<&str> = data.split(",").map(|way| way).collect();

	//let mut line = HashSet::new();
	let mut line = Vec::new();
	line.push(origin);
	for way in wire_circuit_desc {
		let distance: i32 = way[1..].parse().unwrap();
		match &way[0..1] {
			"R" => walk(&mut line, distance, Direction::Right),
			"L" => walk(&mut line, distance, Direction::Left),
			"U" => walk(&mut line, distance, Direction::Upp),
			"D" => walk(&mut line, distance, Direction::Down),
			_ => {}
		};
	}
	line
}

fn part1(
	origin: Point,
	data: (&Vec<Point>, &Vec<Point>),
) -> (u16, BTreeMap<u16, Point>) {
	let (wire_one, wire_two) = data;
	let hash_wire: HashSet<Point> =
		HashSet::from_iter(wire_one.iter().cloned());
	let hash_wire2: HashSet<Point> =
		HashSet::from_iter(wire_two.iter().cloned());
	let mut manhattan_distances: BTreeMap<u16, Point> = BTreeMap::new();

	for x in hash_wire.intersection(&hash_wire2) {
		if *x != origin {
			manhattan_distances.insert(origin.manhattan_distance(x), *x);
		}
	}

	let (distance, _point) = manhattan_distances.iter().next().unwrap();

	return (*distance, manhattan_distances);
}

fn part2(
	manhattan_distances: &BTreeMap<u16, Point>,
	data: (&Vec<Point>, &Vec<Point>),
) -> usize {
	let (wire_one, wire_two) = data;
	let mut steps = BTreeMap::new();

	for (_distance, point) in manhattan_distances.iter() {
		let steps_wire_one =
			wire_one.iter().position(|&r| r == *point).unwrap();
		let steps_wire_two =
			wire_two.iter().position(|&r| r == *point).unwrap();
		steps.insert(steps_wire_one + steps_wire_two, point);
	}

	let (step, _point) = steps.iter().next().unwrap();

	return *step;
}

fn main() {
	let mut data: Vec<String> = Vec::new();
	for line in utils::open_input("./assets/input_day03").lines() {
		data.push(line.unwrap().parse().unwrap())
	}
	let origin = Point { x: 0, y: 0 };

	let wire_one = get_line(origin, &data[0]);
	let wire_two = get_line(origin, &data[1]);

	//println!("{} {} {}", index + index_two, index, index_two);
	println!("Results for Day 3");
	println!("-------------------");
	let (distance, manhattan_distances) = part1(origin, (&wire_one, &wire_two));
	println!("Part 1: {}", distance);
	println!(
		"Part 2: {}",
		part2(&manhattan_distances, (&wire_one, &wire_two))
	);
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn code_coverage_test_main() {
		assert_eq!(main(), ());
	}
}
