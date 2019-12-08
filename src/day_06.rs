
use std::io::prelude::*;
extern crate utilities as utils;
extern crate petgraph;
use petgraph::Direction;
use petgraph::algo::dijkstra;
use petgraph::graphmap::DiGraphMap;
use petgraph::graphmap::UnGraphMap;


pub fn part1(orbits: &Vec<Vec<&str>>) -> i32 {
	let mut g = DiGraphMap::new();
	for planets in orbits {
		g.add_edge(planets[0], planets[1], 1);
	}

    let mut total = 0;
	for node in g.nodes() {
		let mut curr_node = node;
		while let Some(node) =
			g.neighbors_directed(curr_node, Direction::Incoming).next()
		{
			curr_node = node;
			total += 1;
		}
	}
	total
}

fn part2 (orbits: &Vec<Vec<&str>>) -> i32 {
    let mut g_undericted = UnGraphMap::new();
	for planets in orbits {
		g_undericted.add_edge(planets[1], planets[0], 1);
	}

    dijkstra(&g_undericted, "YOU", Some("SAN"), |_| 1)["SAN"] - 2
}

fn main() -> std::io::Result<()> {
    let mut orbits_string: String = String::new();
	utils::open_input("./assets/input_day06")
        .read_to_string(&mut orbits_string)?;

    let orbits: Vec<Vec<&str>> = orbits_string
		.lines()
		.map(|line| line.split(')').collect::<Vec<&str>>())
		.collect();

    println!("Results for Day 6");
	println!("-------------------");
    println!("Part 1: {}", part1(&orbits));
    println!("Part 2: {}", part2(&orbits));


    Ok(())
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn code_coverage_test_main() {
		assert_eq!(main().unwrap(), ());
	}

    #[test]
    fn test_indirect_direct_orbits() {
        let test_orbits ="COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L".to_string();

        let orbits: Vec<Vec<&str>> = test_orbits
		.lines()
		.map(|line| line.split(')').collect::<Vec<&str>>())
		.collect();
        assert_eq!(part1(&orbits), 42);
    }

    #[test]
    fn test_orbital_transfers() {
        let test_orbits ="COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN".to_string();

        let orbits: Vec<Vec<&str>> = test_orbits
		.lines()
		.map(|line| line.split(')').collect::<Vec<&str>>())
		.collect();
        assert_eq!(part2(&orbits), 4);
    }
}

