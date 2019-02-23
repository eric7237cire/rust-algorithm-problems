use bimap::BiMap;
use codejam::util::codejam::run_cases;
use std::collections::{HashMap, HashSet, VecDeque};
use std::io::Write;

/*
Cross product / dot product
normal vectors in a plane
Dividing plane
Sphere
integer math
big ints
*/

pub fn solve_all_cases()
{
    /*
    N, the number of cities visited by K. The next N lines contain three integers Xi, Yi and Zi e
    */

    run_cases(
        &["E-small-practice", "E-large-practice"],
        "y2017round4",
        |reader, buffer| {
            let p = reader.read_int();
            //suit/value
            let premade_stacks: Vec<Vec<(u16, u16)>> = (0..p)
                .map(|_| {
                    let nums = reader.read_num_line::<u16>();
                    let num_cards = nums[0] as usize;
                    (0..num_cards)
                        .map(|cn| (nums[1 + 2 * cn], nums[2 + 2 * cn]))
                        .collect()
                })
                .collect();

            let t = reader.read_int();

            for case in 1..=t {
                let (_n, c) = reader.read_tuple_2::<usize>();
                let stack_indexes = reader.read_num_line::<usize>();
                let stacks = stack_indexes
                    .iter()
                    .map(|si| {
                        assert_eq!(c, premade_stacks[*si].len());
                        premade_stacks[*si].iter().cloned().collect::<VecDeque<_>>()
                    })
                    .collect();

                if case != 12 {
                    //continue;

                }

                println!("Solving {}", case);

                writeln!(
                    buffer,
                    "Case #{}: {}",
                    case,
                    if solve(&stacks) {
                        "POSSIBLE"
                    } else {
                        "IMPOSSIBLE"
                    }
                )
                .unwrap();
            }
        },
    );
}

fn solve(stacks: &Vec<VecDeque<(u16, u16)>>) -> bool
{
    let mut suit_to_cards: HashMap<u16, Vec<u16>> = HashMap::new();
    let mut king_suit_to_stack: BiMap<u16, usize> = BiMap::new();
    let mut last_ace_suit_to_stack: HashMap<u16, usize> = HashMap::new();

    for &(value, suit) in stacks.iter().flatten() {
        suit_to_cards.entry(suit).or_insert(Vec::new()).push(value);
    }

    for cards in suit_to_cards.values_mut() {
        cards.sort();
    }

    for (idx, stack) in stacks.iter().enumerate() {
        debug!("Before  Stack #{}: {:?}", idx, stack);
        for (card_idx, &(value, suit)) in stack.iter().enumerate() {
            let suit_cards = &suit_to_cards[&suit];
            if card_idx == stack.len() - 1 && value == suit_cards[suit_cards.len() - 1] {
                last_ace_suit_to_stack.insert(suit, idx);
            }
            if suit_cards.len() > 1 && value == suit_cards[suit_cards.len() - 2] {
                king_suit_to_stack.insert(suit, idx);
            }
        }
    }

    if suit_to_cards.len() < stacks.len() {
        return true;
    }

    if suit_to_cards.len() > stacks.len() {
        return false;
    }

    for (idx, s) in stacks.iter().enumerate() {
        debug!("Stack #{}: {:?}", idx, s);
    }

    //stack => suit; these stacks have a unique ace at the bottom
    // Let us construct a graph in which vertices are the suits for which the ace begins the game at the bottom of some stack
    let vertices: BiMap<usize, u16> = last_ace_suit_to_stack
        .iter()
        .map(|(suit, stack_idx)| (*stack_idx, *suit))
        .collect();

    debug!("Vertices {:?}", vertices);

    if vertices.len() == stacks.len() {
        return true;
    }

    //We say that a vertex (suit) s is a source if the ace is the only card in this suit,
    let sources: Vec<u16> = vertices
        .iter()
        .filter(|(_, suit)| suit_to_cards[suit].len() == 1)
        .map(|(_, suit)| *suit)
        .collect();

    debug!("Sources {:?}", sources);

    // that s is a target if there is another ace (of a different suit) in the stack in which the ace of s is at the bottom
    let target: HashSet<u16> = vertices
        .iter()
        .filter(|&(stack_idx, suit)| {
            stacks[*stack_idx].iter().any(|(search_card, search_suit)| {
                suit != search_suit && suit_to_cards[search_suit].last().unwrap() == search_card
            })
        })
        .map(|(_, suit)| *suit)
        .collect();

    debug!("Targets: {:?}", target);

    //We add an edge from vertex s1 to a different vertex s2 if the king of s2 is in the stack that has the ace of s1 at the bottom.
    let mut edges: HashMap<u16, Vec<u16>> = HashMap::new();

    for (stack_idx_1, ace_suit_1) in vertices.iter() {
        /*
        We add an edge from vertex s1 to a different vertex s2
        if the king of s2 is in the stack that has the ace of s1 at the bottom.
        */

        if !king_suit_to_stack.contains_right(stack_idx_1) {
            continue;
        }
        let king_suit = king_suit_to_stack.get_by_right(stack_idx_1).unwrap();

        if !vertices.contains_right(&king_suit) {
            continue;
        }

        //let vertex_2_suit = vertices.get_by_right(&kingSuit).unwrap();

        if king_suit == ace_suit_1 {
            continue;
        }

        edges
            .entry(*ace_suit_1)
            .or_insert(Vec::new())
            .push(*king_suit);
    }

    println!("Starting DFS {}", sources.len());
    for source in sources {
        println!("DFS {}", source);
        if bfs(&edges, &mut HashSet::new(), source, &target) {
            return true;
        }
    }

    false
}

fn bfs(
    edges: &HashMap<u16, Vec<u16>>,
    _visited: &mut HashSet<u16>,
    v: u16,
    targets: &HashSet<u16>,
) -> bool
{
    let mut queue: VecDeque<u16> = VecDeque::new();
    let mut visited = HashSet::new();

    queue.push_back(v);
    visited.insert(v);

    while let Some(w) = queue.pop_front() {
        if targets.contains(&w) {
            return true;
        }
        if !edges.contains_key(&w) {
            continue;
        }
        for u in edges[&w].iter() {
            if visited.contains(u) {
                continue;
            }
            visited.insert(*u);
            queue.push_back(*u);
        }
    }

    return false;
}
/*
fn dfs(
    edges: &HashMap<u16, Vec<u16>>,
    visited: &mut HashSet<u16>,
    v: u16,
    targets: &HashSet<u16>,
) -> bool
{
    if targets.contains(&v) {
        return true;
    }
    visited.insert(v);
    let mut found = false;

    if !edges.contains_key(&v) {
        return false;
    }
    for w in edges[&v].iter() {
        if visited.contains(w) {
            continue;
        }
        found |= dfs(edges, visited, *w, targets);

        if found {
            break;
        }
    }

    found
}
*/