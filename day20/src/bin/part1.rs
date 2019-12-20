//! --- Day 20: Donut Maze ---
//!
//! You notice a strange pattern on the surface of Pluto and land nearby to get a closer look. Upon
//! closer inspection, you realize you've come across one of the famous space-warping mazes of the
//! long-lost Pluto civilization!
//!
//! Because there isn't much space on Pluto, the civilization that used to live here thrived by
//! inventing a method for folding spacetime. Although the technology is no longer understood,
//! mazes like this one provide a small glimpse into the daily life of an ancient Pluto citizen.
//!
//! This maze is shaped like a donut. Portals along the inner and outer edge of the donut can
//! instantly teleport you from one side to the other. For example:
//!
//!          A
//!          A
//!   #######.#########
//!   #######.........#
//!   #######.#######.#
//!   #######.#######.#
//!   #######.#######.#
//!   #####  B    ###.#
//! BC...##  C    ###.#
//!   ##.##       ###.#
//!   ##...DE  F  ###.#
//!   #####    G  ###.#
//!   #########.#####.#
//! DE..#######...###.#
//!   #.#########.###.#
//! FG..#########.....#
//!   ###########.#####
//!              Z
//!              Z
//!
//! This map of the maze shows solid walls (#) and open passages (.). Every maze on Pluto has a
//! start (the open tile next to AA) and an end (the open tile next to ZZ). Mazes on Pluto also
//! have portals; this maze has three pairs of portals: BC, DE, and FG. When on an open tile next
//! to one of these labels, a single step can take you to the other tile with the same label. (You
//! can only walk on . tiles; labels and empty space are not traversable.)
//!
//! One path through the maze doesn't require any portals. Starting at AA, you could go down 1,
//! right 8, down 12, left 4, and down 1 to reach ZZ, a total of 26 steps.
//!
//! However, there is a shorter path: You could walk from AA to the inner BC portal (4 steps), warp
//! to the outer BC portal (1 step), walk to the inner DE (6 steps), warp to the outer DE (1 step),
//! walk to the outer FG (4 steps), warp to the inner FG (1 step), and finally walk to ZZ (6
//! steps). In total, this is only 23 steps.
//!
//! Here is a larger example:
//!
//!                    A
//!                    A
//!   #################.#############
//!   #.#...#...................#.#.#
//!   #.#.#.###.###.###.#########.#.#
//!   #.#.#.......#...#.....#.#.#...#
//!   #.#########.###.#####.#.#.###.#
//!   #.............#.#.....#.......#
//!   ###.###########.###.#####.#.#.#
//!   #.....#        A   C    #.#.#.#
//!   #######        S   P    #####.#
//!   #.#...#                 #......VT
//!   #.#.#.#                 #.#####
//!   #...#.#               YN....#.#
//!   #.###.#                 #####.#
//! DI....#.#                 #.....#
//!   #####.#                 #.###.#
//! ZZ......#               QG....#..AS
//!   ###.###                 #######
//! JO..#.#.#                 #.....#
//!   #.#.#.#                 ###.#.#
//!   #...#..DI             BU....#..LF
//!   #####.#                 #.#####
//! YN......#               VT..#....QG
//!   #.###.#                 #.###.#
//!   #.#...#                 #.....#
//!   ###.###    J L     J    #.#.###
//!   #.....#    O F     P    #.#...#
//!   #.###.#####.#.#####.#####.###.#
//!   #...#.#.#...#.....#.....#.#...#
//!   #.#####.###.###.#.#.#########.#
//!   #...#.#.....#...#.#.#.#.....#.#
//!   #.###.#####.###.###.#.#.#######
//!   #.#.........#...#.............#
//!   #########.###.###.#############
//!            B   J   C
//!            U   P   P
//!
//! Here, AA has no direct path to ZZ, but it does connect to AS and CP. By passing through AS, QG,
//! BU, and JO, you can reach ZZ in 58 steps.
//!
//! In your maze, how many steps does it take to get from the open tile marked AA to the open tile
//! marked ZZ?

use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::RangeInclusive;

const LETTERS: RangeInclusive<u8> = b'A'..=b'Z';

struct Node {
    edges: HashMap<String, usize>,
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (label, distance) in self.edges.iter() {
            write!(f, "{}({}) ", label, distance)?;
        }
        Ok(())
    }
}

fn find_labels(map: &Vec<Vec<u8>>) -> Result<HashMap<String, Vec<(usize, usize)>>, Box<dyn Error>> {
    let mut label_map: HashMap<String, Vec<(usize, usize)>> = HashMap::new();
    let (width, height) = (map[0].len(), map.len());
    println!("Finding labels...");
    for y in 0..height {
        for x in 0..width {
            if LETTERS.contains(&map[y][x]) {
                // we check for the second letter below or to the right... if neither of those
                // work, it's because we've already visited this label.
                if y + 1 < height && LETTERS.contains(&map[y + 1][x]) {
                    // vertical label
                    let label = String::from_utf8(vec![map[y][x], map[y + 1][x]])?;
                    println!("\x1b[F\x1b[KFound label: {}", label);

                    let positions = label_map.entry(label).or_default();
                    if y > 0 && map[y - 1][x] == b'.' {
                        (*positions).push((x, y - 1));
                    } else {
                        (*positions).push((x, y + 2));
                    }
                } else if x + 1 < width && LETTERS.contains(&map[y][x + 1]) {
                    // horizontal label
                    let label = String::from_utf8(vec![map[y][x], map[y][x + 1]])?;
                    let positions = label_map.entry(label).or_default();
                    if x > 0 && map[y][x - 1] == b'.' {
                        (*positions).push((x - 1, y));
                    } else {
                        (*positions).push((x + 2, y));
                    }
                }
            }
        }
    }
    println!("\x1b[F\x1b[KFound {} labels", label_map.len());

    Ok(label_map)
}

fn find_connections(
    map: &Vec<Vec<u8>>,
    labels: &HashMap<String, Vec<(usize, usize)>>,
) -> HashMap<String, Node> {
    fn bfs(
        from_label: &String,
        edges: &mut HashMap<String, usize>,
        map: &Vec<Vec<u8>>,
        visited: &mut Vec<Vec<bool>>,
        label_positions: &HashMap<(usize, usize), String>,
        width: usize,
        height: usize,
        pos: (usize, usize),
        distance: usize,
    ) {
        let x = pos.0;
        let y = pos.1;
        visited[y][x] = true;

        if let Some(label) = label_positions.get(&pos) {
            // ignore cycles
            if label != from_label {
                // distance + 1 because it takes one more step to walk into the portal
                let distance = if label == "ZZ" {
                    distance
                } else {
                    distance + 1
                };
                let old = edges.insert(label.clone(), distance);
                if !old.is_none() {
                    panic!("Label {} can reach both portals for {}", from_label, label);
                }
            }
        }

        let moves = [
            (Some(x), y.checked_sub(1)),
            (Some(x + 1), Some(y)),
            (Some(x), Some(y + 1)),
            (x.checked_sub(1), Some(y)),
        ];
        for newpos in moves.iter() {
            if let (Some(x), Some(y)) = newpos {
                if *x < width && *y < height && map[*y][*x] == b'.' && !visited[*y][*x] {
                    bfs(
                        from_label,
                        edges,
                        map,
                        visited,
                        label_positions,
                        width,
                        height,
                        (*x, *y),
                        distance + 1,
                    );
                }
            }
        }
    }

    let mut connections = HashMap::new();
    let (width, height) = (map[0].len(), map.len());
    let label_positions: HashMap<(usize, usize), String> = labels
        .iter()
        .flat_map(|(l, v)| v.iter().map(move |p| (p.clone(), l.clone())))
        .collect();

    for (label, positions) in labels.iter() {
        let mut edges = HashMap::new();
        let mut visited = vec![vec![false; width]; height];
        for pos in positions.iter() {
            bfs(
                label,
                &mut edges,
                &map,
                &mut visited,
                &label_positions,
                width,
                height,
                *pos,
                0,
            );
        }

        let node = Node { edges };
        println!("{} can reach: {}", label, node);
        connections.insert(label.clone(), node);
    }

    connections
}

fn find_shortest_path(connections: &HashMap<String, Node>) -> usize {
    fn dijkstras(
        connections: &HashMap<String, Node>,
        distances: &mut HashMap<String, usize>,
        unvisited: &mut HashSet<String>,
        label: &String,
    ) -> usize {
        let current = connections.get(label).unwrap();
        let distance_from_start = distances.get(label).unwrap().clone();
        if label == "ZZ" {
            return distance_from_start;
        }

        for (to_label, distance) in current.edges.iter() {
            if unvisited.contains(to_label) {
                let current_tentative_distance = distances.get_mut(to_label).unwrap();
                let new_tentative_distance = distance_from_start + distance;
                if new_tentative_distance < *current_tentative_distance {
                    *current_tentative_distance = new_tentative_distance;
                }
            }
        }

        unvisited.remove(label);

        let mut remaining: Vec<(&String, usize)> = unvisited
            .iter()
            .map(|l| (l, *distances.get(l).unwrap()))
            .collect();
        remaining.sort_by_key(|(_, d)| *d);
        if remaining.is_empty() || remaining[0].1 == std::usize::MAX {
            return std::usize::MAX;
        }

        let next_label = remaining[0].0.clone();
        dijkstras(connections, distances, unvisited, &next_label)
    }

    let first_node = "AA".to_owned();
    let mut distances = connections
        .keys()
        .map(|k| {
            (
                k.clone(),
                if *k == first_node { 0 } else { std::usize::MAX },
            )
        })
        .collect();
    let mut unvisited = connections.keys().cloned().collect();
    dijkstras(&connections, &mut distances, &mut unvisited, &first_node)
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let map = reader
        .lines()
        .map(|line| line.map(|l| l.bytes().collect::<Vec<u8>>()))
        .collect::<Result<Vec<Vec<_>>, _>>()?;
    let labels = find_labels(&map)?;
    let connections = find_connections(&map, &labels);
    let result = find_shortest_path(&connections);
    println!("Shortest distance: {}", result);

    Ok(())
}
