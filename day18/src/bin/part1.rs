//! --- Day 18: Many-Worlds Interpretation ---
//!
//! As you approach Neptune, a planetary security system detects you and activates a giant tractor
//! beam on Triton! You have no choice but to land.
//!
//! A scan of the local area reveals only one interesting feature: a massive underground vault. You
//! generate a map of the tunnels (your puzzle input). The tunnels are too narrow to move
//! diagonally.
//!
//! Only one entrance (marked @) is present among the open passages (marked .) and stone walls (#),
//! but you also detect an assortment of keys (shown as lowercase letters) and doors (shown as
//! uppercase letters). Keys of a given letter open the door of the same letter: a opens A, b opens
//! B, and so on. You aren't sure which key you need to disable the tractor beam, so you'll need to
//! collect all of them.
//!
//! For example, suppose you have the following map:
//!
//! #########
//! #b.A.@.a#
//! #########
//!
//! Starting from the entrance (@), you can only access a large door (A) and a key (a). Moving
//! toward the door doesn't help you, but you can move 2 steps to collect the key, unlocking A in
//! the process:
//!
//! #########
//! #b.....@#
//! #########
//!
//! Then, you can move 6 steps to collect the only other key, b:
//!
//! #########
//! #@......#
//! #########
//!
//! So, collecting every key took a total of 8 steps.
//!
//! Here is a larger example:
//!
//! ########################
//! #f.D.E.e.C.b.A.@.a.B.c.#
//! ######################.#
//! #d.....................#
//! ########################
//!
//! The only reasonable move is to take key a and unlock door A:
//!
//! ########################
//! #f.D.E.e.C.b.....@.B.c.#
//! ######################.#
//! #d.....................#
//! ########################
//!
//! Then, do the same with key b:
//!
//! ########################
//! #f.D.E.e.C.@.........c.#
//! ######################.#
//! #d.....................#
//! ########################
//!
//! ...and the same with key c:
//!
//! ########################
//! #f.D.E.e.............@.#
//! ######################.#
//! #d.....................#
//! ########################
//!
//! Now, you have a choice between keys d and e. While key e is closer, collecting it now would be
//! slower in the long run than collecting key d first, so that's the best choice:
//!
//! ########################
//! #f...E.e...............#
//! ######################.#
//! #@.....................#
//! ########################
//!
//! Finally, collect key e to unlock door E, then collect key f, taking a grand total of 86 steps.
//!
//! Here are a few more examples:
//!
//!     ########################
//!     #...............b.C.D.f#
//!     #.######################
//!     #.....@.a.B.c.d.A.e.F.g#
//!     ########################
//!
//!     Shortest path is 132 steps: b, a, c, d, f, e, g
//!
//!     #################
//!     #i.G..c...e..H.p#
//!     ########.########
//!     #j.A..b...f..D.o#
//!     ########@########
//!     #k.E..a...g..B.n#
//!     ########.########
//!     #l.F..d...h..C.m#
//!     #################
//!
//!     Shortest paths are 136 steps;
//!     one is: a, f, b, j, g, n, h, d, l, o, e, p, c, i, k, m
//!
//!     ########################
//!     #@..............ac.GI.b#
//!     ###d#e#f################
//!     ###A#B#C################
//!     ###g#h#i################
//!     ########################
//!
//!     Shortest paths are 81 steps; one is: a, c, f, i, d, g, b, e, h
//!
//! How many steps is the shortest path that collects all of the keys?

use std::collections::{HashMap, HashSet, VecDeque};
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::{Range, RangeInclusive};
use std::rc::Rc;

const KEYS: RangeInclusive<u8> = b'a'..=b'z';
const DOORS: RangeInclusive<u8> = b'A'..=b'Z';
const DIRECTIONS: [(isize, isize); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

struct Node {
    keys: HashMap<u8, Edge>,
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut keys: Vec<u8> = self.keys.keys().copied().collect();
        keys.sort();
        for (i, key) in keys.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }

            let edge = self.keys.get(key).unwrap();
            write!(f, "{}{}", char::from(*key), edge)?;
        }
        Ok(())
    }
}

struct Edge {
    distance: usize,
    required_keys: HashSet<u8>,
}

impl fmt::Display for Edge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut required_keys: Vec<char> =
            self.required_keys.iter().map(|&c| char::from(c)).collect();
        required_keys.sort();
        write!(
            f,
            "({},{})",
            self.distance,
            required_keys.iter().collect::<String>()
        )
    }
}

fn build_graph(map: Vec<Vec<u8>>) -> HashMap<u8, Node> {
    // This function does a bfs to find the connections between a given key to all other keys. It
    // assumes there is only one way of getting between each pair so it doesn't bother making sure
    // each path is the "shortest".
    fn find_keys(
        map: &Vec<Vec<u8>>,
        visited: &mut Vec<Vec<u8>>,
        all_keys: &HashSet<u8>,
        from_key: u8,
        keys: &mut HashMap<u8, Edge>,
        validx: &Range<usize>,
        validy: &Range<usize>,
        x: usize,
        y: usize,
    ) {
        let mut queue: VecDeque<(usize, usize, usize, Rc<HashSet<u8>>)> = VecDeque::new();
        queue.push_back((x, y, 0, Rc::new(HashSet::new())));

        while let Some((x, y, distance, mut required_keys)) = queue.pop_front() {
            visited[y][x] = from_key;

            match map[y][x] {
                key if key != from_key && KEYS.contains(&key) => {
                    // if we require *this* key to reach *this* key, that clearly can't happen
                    if !required_keys.contains(&key) {
                        keys.insert(
                            key,
                            Edge {
                                distance,
                                required_keys: required_keys.as_ref().clone(),
                            },
                        );
                    }
                }
                door if DOORS.contains(&door) => {
                    let mut required_keys_set = required_keys.as_ref().clone();
                    required_keys_set.insert(door - b'A' + b'a');
                    required_keys = Rc::new(required_keys_set);
                }
                _ => (),
            }

            if keys.len() == all_keys.len() {
                // we've found all the keys; no need to continue our search
                return;
            }

            for (deltax, deltay) in DIRECTIONS.iter() {
                let x = ((x as isize) + deltax) as usize;
                let y = ((y as isize) + deltay) as usize;
                if validx.contains(&x)
                    && validy.contains(&y)
                    && map[y][x] != b'#'
                    && visited[y][x] != from_key
                {
                    queue.push_back((x, y, distance + 1, Rc::clone(&required_keys)));
                }
            }
        }
    }

    // find the position of all of the keys and the starting point
    let (width, height) = (map[0].len(), map.len());
    let validx = 1..(width - 1);
    let validy = 1..(height - 1);
    let mut all_keys = HashSet::new();
    let mut positions = Vec::new();
    for y in validy.clone() {
        for x in validx.clone() {
            if KEYS.contains(&map[y][x]) || map[y][x] == b'@' {
                if map[y][x] != b'@' {
                    all_keys.insert(map[y][x]);
                }
                positions.push((x, y));
            }
        }
    }

    // build a graph from each key and the starting point to every other key, tracking the distance
    // and the keys necessary to make that journey. `visited` is created once to minimize allocs.
    let mut visited = vec![vec![0u8; width]; height];
    let mut graph = HashMap::new();
    println!("Built graph for  0/{} keys + start", positions.len());
    for (i, (x, y)) in positions.iter().enumerate() {
        let mut all_keys = all_keys.clone();
        all_keys.remove(&map[*y][*x]);

        let mut keys = HashMap::new();
        find_keys(
            &map,
            &mut visited,
            &all_keys,
            map[*y][*x],
            &mut keys,
            &validx,
            &validy,
            *x,
            *y,
        );

        let node = Node { keys };
        graph.insert(map[*y][*x], node);

        println!(
            "\x1b[FBuilt graph for {: >2}/{} keys + start",
            i + 1,
            positions.len()
        );
    }

    // print our graph
    let mut keys: Vec<u8> = graph.keys().copied().collect();
    keys.sort();
    for key in keys {
        let node = graph.get(&key).unwrap();
        println!("{}: {}", char::from(key), node);
    }

    graph
}

fn find_shortest_path(graph: HashMap<u8, Node>) -> usize {
    fn build_cache_key(current: u8, collected_keys: &HashSet<u8>) -> String {
        let mut s = String::with_capacity(collected_keys.len() + 1);
        s.push(char::from(current));

        let mut collected_keys: Vec<u8> = collected_keys.iter().copied().collect();
        collected_keys.sort();
        for collected in collected_keys.iter() {
            s.push(char::from(*collected));
        }

        s
    }

    fn dfs(
        cache: &mut HashMap<String, usize>,
        graph: &HashMap<u8, Node>,
        current: u8,
        all_keys: &HashSet<u8>,
        collected_keys: &HashSet<u8>,
    ) -> usize {
        if all_keys.len() == collected_keys.len() + 1 {
            // if I'm at the last key, I no longer need to move to finish
            return 0;
        }

        let cache_key = build_cache_key(current, collected_keys);
        if let Some(distance) = cache.get(&cache_key) {
            return *distance;
        }

        let mut collected_keys = collected_keys.clone();
        collected_keys.insert(current.clone());

        let node = graph.get(&current).unwrap();
        let reachable_neighbors: Vec<(&u8, &Edge)> = node
            .keys
            .iter()
            .filter(|(k, edge)| {
                !collected_keys.contains(k) && edge.required_keys.is_subset(&collected_keys)
            })
            .collect();

        let mut shortest_distance = std::usize::MAX;
        for (key, edge) in reachable_neighbors {
            // dfs may return MAX if we reach a node from which we cannot complete the maze
            let distance = dfs(cache, graph, *key, all_keys, &mut collected_keys)
                .saturating_add(edge.distance);
            if distance < shortest_distance {
                shortest_distance = distance;
            }
        }

        cache.insert(cache_key, shortest_distance);
        shortest_distance
    }

    let all_keys: HashSet<u8> = graph.keys().copied().collect();
    dfs(
        &mut HashMap::new(),
        &graph,
        b'@',
        &all_keys,
        &HashSet::new(),
    )
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let map = reader
        .lines()
        .map(|line| line.map(|l| l.bytes().collect::<Vec<u8>>()))
        .collect::<Result<Vec<Vec<u8>>, _>>()?;
    let graph = build_graph(map);
    let shortest_path = find_shortest_path(graph);
    println!("Shortest: {}", shortest_path);

    Ok(())
}
