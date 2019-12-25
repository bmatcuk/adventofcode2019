//! --- Part Two ---
//!
//! You arrive at the vault only to discover that there is not one vault, but four - each with its
//! own entrance.
//!
//! On your map, find the area in the middle that looks like this:
//!
//! ...
//! .@.
//! ...
//!
//! Update your map to instead use the correct data:
//!
//! @#@
//! ###
//! @#@
//!
//! This change will split your map into four separate sections, each with its own entrance:
//!
//! #######       #######
//! #a.#Cd#       #a.#Cd#
//! ##...##       ##@#@##
//! ##.@.##  -->  #######
//! ##...##       ##@#@##
//! #cB#Ab#       #cB#Ab#
//! #######       #######
//!
//! Because some of the keys are for doors in other vaults, it would take much too long to collect
//! all of the keys by yourself. Instead, you deploy four remote-controlled robots. Each starts at
//! one of the entrances (@).
//!
//! Your goal is still to collect all of the keys in the fewest steps, but now, each robot has its
//! own position and can move independently. You can only remotely control a single robot at a
//! time. Collecting a key instantly unlocks any corresponding doors, regardless of the vault in
//! which the key or door is found.
//!
//! For example, in the map above, the top-left robot first collects key a, unlocking door A in the
//! bottom-right vault:
//!
//! #######
//! #@.#Cd#
//! ##.#@##
//! #######
//! ##@#@##
//! #cB#.b#
//! #######
//!
//! Then, the bottom-right robot collects key b, unlocking door B in the bottom-left vault:
//!
//! #######
//! #@.#Cd#
//! ##.#@##
//! #######
//! ##@#.##
//! #c.#.@#
//! #######
//!
//! Then, the bottom-left robot collects key c:
//!
//! #######
//! #@.#.d#
//! ##.#@##
//! #######
//! ##.#.##
//! #@.#.@#
//! #######
//!
//! Finally, the top-right robot collects key d:
//!
//! #######
//! #@.#.@#
//! ##.#.##
//! #######
//! ##.#.##
//! #@.#.@#
//! #######
//!
//! In this example, it only took 8 steps to collect all of the keys.
//!
//! Sometimes, multiple robots might have keys available, or a robot might have to wait for
//! multiple keys to be collected:
//!
//! ###############
//! #d.ABC.#.....a#
//! ######@#@######
//! ###############
//! ######@#@######
//! #b.....#.....c#
//! ###############
//!
//! First, the top-right, bottom-left, and bottom-right robots take turns collecting keys a, b, and
//! c, a total of 6 + 6 + 6 = 18 steps. Then, the top-left robot can access key d, spending another
//! 6 steps; collecting all of the keys here takes a minimum of 24 steps.
//!
//! Here's a more complex example:
//!
//! #############
//! #DcBa.#.GhKl#
//! #.###@#@#I###
//! #e#d#####j#k#
//! ###C#@#@###J#
//! #fEbA.#.FgHi#
//! #############
//!
//!     Top-left robot collects key a.
//!     Bottom-left robot collects key b.
//!     Top-left robot collects key c.
//!     Bottom-left robot collects key d.
//!     Top-left robot collects key e.
//!     Bottom-left robot collects key f.
//!     Bottom-right robot collects key g.
//!     Top-right robot collects key h.
//!     Bottom-right robot collects key i.
//!     Top-right robot collects key j.
//!     Bottom-right robot collects key k.
//!     Top-right robot collects key l.
//!
//! In the above example, the fewest steps to collect all of the keys is 32.
//!
//! Here's an example with more choices:
//!
//! #############
//! #g#f.D#..h#l#
//! #F###e#E###.#
//! #dCba@#@BcIJ#
//! #############
//! #nK.L@#@G...#
//! #M###N#H###.#
//! #o#m..#i#jk.#
//! #############
//!
//! One solution with the fewest steps is:
//!
//!     Top-left robot collects key e.
//!     Top-right robot collects key h.
//!     Bottom-right robot collects key i.
//!     Top-left robot collects key a.
//!     Top-left robot collects key b.
//!     Top-right robot collects key c.
//!     Top-left robot collects key d.
//!     Top-left robot collects key f.
//!     Top-left robot collects key g.
//!     Bottom-right robot collects key k.
//!     Bottom-right robot collects key j.
//!     Top-right robot collects key l.
//!     Bottom-left robot collects key n.
//!     Bottom-left robot collects key m.
//!     Bottom-left robot collects key o.
//!
//! This example requires at least 72 steps to collect all keys.
//!
//! After updating your map and using the remote-controlled robots, what is the fewest steps
//! necessary to collect all of the keys?

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

struct Graph {
    nodes: HashMap<u8, Node>,
    collected_keys: HashSet<u8>,
}

impl fmt::Display for Graph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut collected_keys: Vec<char> =
            self.collected_keys.iter().map(|&c| char::from(c)).collect();
        collected_keys.sort();
        write!(
            f,
            "Keys not in this graph: {}\n",
            collected_keys.iter().collect::<String>()
        )?;

        let mut keys: Vec<u8> = self.nodes.keys().copied().collect();
        keys.sort();
        for key in keys {
            let node = self.nodes.get(&key).unwrap();
            write!(f, "{}: {}\n", char::from(key), node)?;
        }

        Ok(())
    }
}

fn build_graphs(mut map: Vec<Vec<u8>>) -> Vec<Graph> {
    // This function does a bfs to find the connections between a given key to all other keys. It
    // assumes there is only one way of getting between each pair so it doesn't bother making sure
    // each path is the "shortest".
    fn find_keys(
        map: &Vec<Vec<u8>>,
        visited: &mut Vec<Vec<u8>>,
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
    let (mut middlex, mut middley) = (0, 0);
    let mut all_keys = HashSet::new();
    let mut positions = Vec::new();
    for y in 1..(height - 1) {
        for x in 1..(width - 1) {
            if map[y][x] == b'@' {
                middlex = x;
                middley = y;
            } else if KEYS.contains(&map[y][x]) {
                all_keys.insert(map[y][x]);
                positions.push((x, y));
            }
        }
    }

    // modify the input to create four separate maps
    let starting_points = [
        ((middlex - 1, middley - 1), 1..middlex, 1..middley),
        (
            (middlex + 1, middley - 1),
            (middlex + 1)..(width - 1),
            1..middley,
        ),
        (
            (middlex + 1, middley + 1),
            (middlex + 1)..(width - 1),
            (middley + 1)..(height - 1),
        ),
        (
            (middlex - 1, middley + 1),
            1..middlex,
            (middley + 1)..(height - 1),
        ),
    ];
    map[middley][middlex] = b'#';
    map[middley - 1][middlex - 1] = b'@';
    map[middley - 1][middlex] = b'#';
    map[middley - 1][middlex + 1] = b'@';
    map[middley][middlex + 1] = b'#';
    map[middley + 1][middlex + 1] = b'@';
    map[middley + 1][middlex] = b'#';
    map[middley + 1][middlex - 1] = b'@';
    map[middley][middlex - 1] = b'#';

    // build a graph from each key and the starting point to every other key, tracking the distance
    // and the keys necessary to make that journey. `visited` is created once to minimize allocs.
    let mut visited = vec![vec![0u8; width]; height];
    let num_graphs = starting_points.len();
    println!("Built 0/{} graphs", num_graphs);

    let graphs: Vec<Graph> = starting_points
        .iter()
        .enumerate()
        .map(|(graph_num, (starting_point, validx, validy))| {
            let mut positions = positions.clone();
            positions.push(*starting_point);

            let mut nodes = HashMap::new();
            for (x, y) in positions.iter() {
                if !validx.contains(x) || !validy.contains(y) {
                    continue;
                }

                let mut keys = HashMap::new();
                find_keys(
                    &map,
                    &mut visited,
                    map[*y][*x],
                    &mut keys,
                    &validx,
                    &validy,
                    *x,
                    *y,
                );

                let node = Node { keys };
                nodes.insert(map[*y][*x], node);
            }

            let contains_keys: HashSet<u8> = nodes.keys().copied().collect();
            let collected_keys = all_keys.difference(&contains_keys).copied().collect();

            println!("\x1b[FBuilt {}/{} graphs", graph_num + 1, num_graphs);
            Graph {
                nodes,
                collected_keys,
            }
        })
        .collect();

    // print the graphs
    for graph in graphs.iter() {
        println!("{}", graph);
    }

    graphs
}

fn find_shortest_path(graph: &HashMap<u8, Node>, collected_keys: &HashSet<u8>) -> usize {
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
        let cache_key = build_cache_key(current, collected_keys);
        if let Some(distance) = cache.get(&cache_key) {
            return *distance;
        }

        let mut collected_keys = collected_keys.clone();
        collected_keys.insert(current.clone());

        if all_keys.is_subset(&collected_keys) {
            // we're at the last key so we no longer need to move to finish
            cache.insert(cache_key, 0);
            return 0;
        }

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
    dfs(&mut HashMap::new(), &graph, b'@', &all_keys, collected_keys)
}

fn main() -> Result<(), Box<dyn Error>> {
    // The intuition here is that each map is really independent. If a robot on one map reaches an
    // impasse such that the remaining keys are behind locked doors that other robots must unlock,
    // then it will not move while it waits for the other robots to unlock those doors. So,
    // essentially, we can treat each map as if every door that is unlocked by other robots has
    // already been unlocked, sovle them separately, and then sum their shortest paths together.
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let map = reader
        .lines()
        .map(|line| line.map(|l| l.bytes().collect::<Vec<u8>>()))
        .collect::<Result<Vec<Vec<u8>>, _>>()?;
    let graphs = build_graphs(map);
    let shortest_path: usize = graphs
        .iter()
        .map(
            |Graph {
                 nodes,
                 collected_keys,
             }| find_shortest_path(nodes, collected_keys),
        )
        .sum();
    println!("Shortest: {}", shortest_path);

    Ok(())
}
