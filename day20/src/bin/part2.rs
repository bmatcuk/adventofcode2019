//! --- Part Two ---
//!
//! Strangely, the exit isn't open when you reach it. Then, you remember: the ancient Plutonians
//! were famous for building recursive spaces.
//!
//! The marked connections in the maze aren't portals: they physically connect to a larger or
//! smaller copy of the maze. Specifically, the labeled tiles around the inside edge actually
//! connect to a smaller copy of the same maze, and the smaller copy's inner labeled tiles connect
//! to yet a smaller copy, and so on.
//!
//! When you enter the maze, you are at the outermost level; when at the outermost level, only the
//! outer labels AA and ZZ function (as the start and end, respectively); all other outer labeled
//! tiles are effectively walls. At any other level, AA and ZZ count as walls, but the other outer
//! labeled tiles bring you one level outward.
//!
//! Your goal is to find a path through the maze that brings you back to ZZ at the outermost level
//! of the maze.
//!
//! In the first example above, the shortest path is now the loop around the right side. If the
//! starting level is 0, then taking the previously-shortest path would pass through BC (to level
//! 1), DE (to level 2), and FG (back to level 1). Because this is not the outermost level, ZZ is a
//! wall, and the only option is to go back around to BC, which would only send you even deeper
//! into the recursive maze.
//!
//! In the second example above, there is no path that brings you to ZZ at the outermost level.
//!
//! Here is a more interesting example:
//!
//!              Z L X W       C
//!              Z P Q B       K
//!   ###########.#.#.#.#######.###############
//!   #...#.......#.#.......#.#.......#.#.#...#
//!   ###.#.#.#.#.#.#.#.###.#.#.#######.#.#.###
//!   #.#...#.#.#...#.#.#...#...#...#.#.......#
//!   #.###.#######.###.###.#.###.###.#.#######
//!   #...#.......#.#...#...#.............#...#
//!   #.#########.#######.#.#######.#######.###
//!   #...#.#    F       R I       Z    #.#.#.#
//!   #.###.#    D       E C       H    #.#.#.#
//!   #.#...#                           #...#.#
//!   #.###.#                           #.###.#
//!   #.#....OA                       WB..#.#..ZH
//!   #.###.#                           #.#.#.#
//! CJ......#                           #.....#
//!   #######                           #######
//!   #.#....CK                         #......IC
//!   #.###.#                           #.###.#
//!   #.....#                           #...#.#
//!   ###.###                           #.#.#.#
//! XF....#.#                         RF..#.#.#
//!   #####.#                           #######
//!   #......CJ                       NM..#...#
//!   ###.#.#                           #.###.#
//! RE....#.#                           #......RF
//!   ###.###        X   X       L      #.#.#.#
//!   #.....#        F   Q       P      #.#.#.#
//!   ###.###########.###.#######.#########.###
//!   #.....#...#.....#.......#...#.....#.#...#
//!   #####.#.###.#######.#######.###.###.#.#.#
//!   #.......#.......#.#.#.#.#...#...#...#.#.#
//!   #####.###.#####.#.#.#.#.###.###.#.###.###
//!   #.......#.....#.#...#...............#...#
//!   #############.#.#.###.###################
//!                A O F   N
//!                A A D   M
//!
//! One shortest path through the maze is the following:
//!
//!     Walk from AA to XF (16 steps)
//!     Recurse into level 1 through XF (1 step)
//!     Walk from XF to CK (10 steps)
//!     Recurse into level 2 through CK (1 step)
//!     Walk from CK to ZH (14 steps)
//!     Recurse into level 3 through ZH (1 step)
//!     Walk from ZH to WB (10 steps)
//!     Recurse into level 4 through WB (1 step)
//!     Walk from WB to IC (10 steps)
//!     Recurse into level 5 through IC (1 step)
//!     Walk from IC to RF (10 steps)
//!     Recurse into level 6 through RF (1 step)
//!     Walk from RF to NM (8 steps)
//!     Recurse into level 7 through NM (1 step)
//!     Walk from NM to LP (12 steps)
//!     Recurse into level 8 through LP (1 step)
//!     Walk from LP to FD (24 steps)
//!     Recurse into level 9 through FD (1 step)
//!     Walk from FD to XQ (8 steps)
//!     Recurse into level 10 through XQ (1 step)
//!     Walk from XQ to WB (4 steps)
//!     Return to level 9 through WB (1 step)
//!     Walk from WB to ZH (10 steps)
//!     Return to level 8 through ZH (1 step)
//!     Walk from ZH to CK (14 steps)
//!     Return to level 7 through CK (1 step)
//!     Walk from CK to XF (10 steps)
//!     Return to level 6 through XF (1 step)
//!     Walk from XF to OA (14 steps)
//!     Return to level 5 through OA (1 step)
//!     Walk from OA to CJ (8 steps)
//!     Return to level 4 through CJ (1 step)
//!     Walk from CJ to RE (8 steps)
//!     Return to level 3 through RE (1 step)
//!     Walk from RE to IC (4 steps)
//!     Recurse into level 4 through IC (1 step)
//!     Walk from IC to RF (10 steps)
//!     Recurse into level 5 through RF (1 step)
//!     Walk from RF to NM (8 steps)
//!     Recurse into level 6 through NM (1 step)
//!     Walk from NM to LP (12 steps)
//!     Recurse into level 7 through LP (1 step)
//!     Walk from LP to FD (24 steps)
//!     Recurse into level 8 through FD (1 step)
//!     Walk from FD to XQ (8 steps)
//!     Recurse into level 9 through XQ (1 step)
//!     Walk from XQ to WB (4 steps)
//!     Return to level 8 through WB (1 step)
//!     Walk from WB to ZH (10 steps)
//!     Return to level 7 through ZH (1 step)
//!     Walk from ZH to CK (14 steps)
//!     Return to level 6 through CK (1 step)
//!     Walk from CK to XF (10 steps)
//!     Return to level 5 through XF (1 step)
//!     Walk from XF to OA (14 steps)
//!     Return to level 4 through OA (1 step)
//!     Walk from OA to CJ (8 steps)
//!     Return to level 3 through CJ (1 step)
//!     Walk from CJ to RE (8 steps)
//!     Return to level 2 through RE (1 step)
//!     Walk from RE to XQ (14 steps)
//!     Return to level 1 through XQ (1 step)
//!     Walk from XQ to FD (8 steps)
//!     Return to level 0 through FD (1 step)
//!     Walk from FD to ZZ (18 steps)
//!
//! This path takes a total of 396 steps to move from AA at the outermost layer to ZZ at the
//! outermost layer.
//!
//! In your maze, when accounting for recursion, how many steps does it take to get from the open
//! tile marked AA to the open tile marked ZZ, both at the outermost layer?

use std::collections::{HashMap, HashSet, VecDeque};
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::RangeInclusive;

const LETTERS: RangeInclusive<u8> = b'A'..=b'Z';

struct Node {
    edges: HashMap<(String, bool), (usize, bool)>,
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for ((label, _), (distance, inner)) in self.edges.iter() {
            write!(
                f,
                "{}({},{}) ",
                label,
                distance,
                if *inner { "I" } else { "O" }
            )?;
        }
        Ok(())
    }
}

fn find_labels(
    map: &Vec<Vec<u8>>,
) -> Result<HashMap<String, Vec<(usize, usize, bool)>>, Box<dyn Error>> {
    let mut label_map: HashMap<String, Vec<(usize, usize, bool)>> = HashMap::new();
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
                        // entrance on top of label
                        let inner = y < height - 3;
                        (*positions).push((x, y - 1, inner));
                    } else {
                        // entrace on bottom of label
                        let inner = y > 2;
                        (*positions).push((x, y + 2, inner));
                    }
                } else if x + 1 < width && LETTERS.contains(&map[y][x + 1]) {
                    // horizontal label
                    let label = String::from_utf8(vec![map[y][x], map[y][x + 1]])?;
                    let positions = label_map.entry(label).or_default();
                    if x > 0 && map[y][x - 1] == b'.' {
                        // entrance to the left of label
                        let inner = x < width - 3;
                        (*positions).push((x - 1, y, inner));
                    } else {
                        // entrance to the right of label
                        let inner = x > 2;
                        (*positions).push((x + 2, y, inner));
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
    labels: &HashMap<String, Vec<(usize, usize, bool)>>,
) -> HashMap<String, Node> {
    fn bfs(
        from_label: &String,
        from_inner: bool,
        edges: &mut HashMap<(String, bool), (usize, bool)>,
        map: &Vec<Vec<u8>>,
        visited: &mut Vec<Vec<bool>>,
        label_positions: &HashMap<(usize, usize), (String, bool)>,
        width: usize,
        height: usize,
        pos: (usize, usize),
        distance: usize,
    ) {
        let x = pos.0;
        let y = pos.1;
        visited[y][x] = true;

        if let Some((label, inner)) = label_positions.get(&pos) {
            // ignore cycles - AA and ZZ can only connect to inner labels, and don't bother
            // recording any connections *back* to AA; we don't care.
            let mut valid_connection = label != from_label;
            if (from_label == "AA" && !inner) || label == "AA" {
                valid_connection = false;
            }
            if from_label == "ZZ" && !inner {
                valid_connection = false;
            }

            if valid_connection {
                // distance + 1 because it takes one more step to walk into the portal, except ZZ
                let distance = if label == "ZZ" {
                    distance
                } else {
                    distance + 1
                };
                let old = edges.insert((label.clone(), from_inner), (distance, *inner));
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
                        from_inner,
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
    let label_positions: HashMap<(usize, usize), (String, bool)> = labels
        .iter()
        .flat_map(|(l, v)| {
            v.iter()
                .map(move |(x, y, inner)| ((*x, *y), (l.clone(), *inner)))
        })
        .collect();

    for (label, positions) in labels.iter() {
        if label == "ZZ" {
            // no need to compute the connections here
            continue;
        }

        let mut edges = HashMap::new();
        let mut visited = vec![vec![false; width]; height];
        for (x, y, inner) in positions.iter() {
            bfs(
                label,
                *inner,
                &mut edges,
                &map,
                &mut visited,
                &label_positions,
                width,
                height,
                (*x, *y),
                0,
            );
        }

        let node = Node { edges };
        connections.insert(label.clone(), node);
    }

    let mut label_names: Vec<String> = connections.keys().cloned().collect();
    label_names.sort();
    for label in label_names {
        let node = connections.get(&label).unwrap();
        println!("{} can reach: {}", label, node);
    }

    connections
}

fn find_shortest_path(connections: &HashMap<String, Node>) -> usize {
    // We can't use Dijkstra's anymore because the quickest way to the portal before ZZ may put us
    // in a level > 0, which means we'll never finish the maze. By extension, the quickest way to
    // the portal 2 steps before ZZ may not set us up for success, etc. Additionally, we may need
    // to revisit nodes to walk our way back up to level 0. Dijkstra just won't work. Because we
    // need to allow cycles, we also can't use dfs because we might end up in an infinite loop.
    // We'll need bfs instead.
    let mut shortest = std::usize::MAX;
    let mut queue = VecDeque::new();
    queue.push_back(("AA".to_owned(), false, 0, 0, Vec::<String>::new()));
    while let Some((current, from_inner, total_distance, level, path)) = queue.pop_front() {
        if level == 50 || total_distance > shortest {
            // we've probably gone too far; bail
            continue;
        }

        let node = connections.get(&current).unwrap();
        for ((to_label, to_from_inner), (distance, inner)) in node.edges.iter() {
            if from_inner != *to_from_inner {
                continue;
            }

            let mut path = path.clone();
            path.push(format!("{}({}, {})", current, level, total_distance));

            if to_label == "ZZ" {
                if level == 0 {
                    // we've reached the end!
                    let total_distance = total_distance + *distance;
                    if total_distance < shortest {
                        shortest = total_distance;
                    }
                }

                // either way, we need to stop here
                continue;
            }

            let level = if *inner {
                level + 1
            } else if level > 0 {
                level - 1
            } else {
                continue;
            };
            queue.push_back((
                to_label.clone(),
                !*inner,
                total_distance + *distance,
                level,
                path,
            ));
        }
    }

    shortest
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
