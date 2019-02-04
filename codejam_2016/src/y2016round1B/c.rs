use codejam::util::codejam::run_cases;

//use itertools::Itertools;
//use std::cmp::min;
use bit_vec::BitVec;
use std::collections::HashMap;
use std::usize;

use std::io::Write;
/*
Minimum edge cover
Direct/Simple Bipartite implementation

*/
pub fn solve_all_cases()
{
    run_cases(
        &["C-small-practice", "C-large-practice"],
        "y2016round1B",
        |reader, buffer| {
            let t = reader.read_int();

            for case_no in 1..=t {
                let N = reader.read_int();

                let topics: Vec<Vec<String>> = (0..N).map(|_| reader.read_string_line()).collect();

                if case_no != 4 {
                    //continue;
                }

                println!("Solving case {}", case_no);

                writeln!(buffer, "Case #{}: {}", case_no, solve(&topics)).unwrap();
            }
        },
    );
}

const INVALID: usize = usize::MAX;
const MAX_N: usize = 1000;

fn solve(topics: &[Vec<String>]) -> usize
{
    let mut first_word_ids: HashMap<&str, usize> = HashMap::new();
    let mut second_word_ids: HashMap<&str, usize> = HashMap::new();
    let mut first_words = vec![String::new(); MAX_N];
    let mut second_words = vec![String::new(); MAX_N];

    let edges: Vec<[usize; 2]> = topics
        .iter()
        .map(|topic_words| {
            let next_first_id = first_word_ids.len();
            let next_second_id = second_word_ids.len();

            let first_id = *first_word_ids
                .entry(&topic_words[0])
                .or_insert(next_first_id);
            let second_id = *second_word_ids
                .entry(&topic_words[1])
                .or_insert(next_second_id);

            first_words[first_id] = topic_words[0].clone();
            second_words[second_id] = topic_words[1].clone();

            /*
            println!(
                "Edge from {}/{} to {}/{}",
                topic_words[0], first_id, topic_words[1], second_id
            );*/

            [first_id, second_id]
        })
        .collect();

    //create edges from first word to second word

    let mut matchL = vec![INVALID; first_word_ids.len()];

    //match second words
    let mut matchR = vec![INVALID; second_word_ids.len()];

    //Storing values
    let mut queue: Vec<usize> = vec![0; first_word_ids.len()];
    let mut back = vec![0; first_word_ids.len()];

    let mut used_first = BitVec::from_elem(first_word_ids.len(), false);

    let mut cur_first_word_id = 0;
    while usize::from(cur_first_word_id) < first_word_ids.len() {
        let mut queueHead = 0;
        let mut queueTail = 1;
        queue[0] = cur_first_word_id;
        used_first.set(cur_first_word_id, true);
        back[cur_first_word_id] = INVALID;
        //let mut found = false;
        'bfs: loop {
            assert!(queue[queueHead] != INVALID);
            let top_queue_first_word = queue[queueHead];
            queueHead += 1;

            for adj_second_edge in edges.iter().filter(|edge| edge[0] == top_queue_first_word) {
                assert_eq!(top_queue_first_word, adj_second_edge[0]);

                let adj_second_index = usize::from(adj_second_edge[1]);
                //Found a non matched second index
                if matchR[adj_second_index] == INVALID {
                    let mut next_second_index = adj_second_index;
                    let mut next_first_index = top_queue_first_word;

                    matchR[adj_second_index] = next_first_index;
                    //Applying the augmenting path
                    while back[next_first_index] != INVALID {
                        assert!(back[next_first_index] != INVALID);
                        assert!(matchL[next_first_index] != INVALID);
                        let prev = back[next_first_index];
                        let pnext = matchL[next_first_index];
                        matchL[next_first_index] = next_second_index;
                        matchR[pnext] = prev;
                        next_first_index = prev;
                        next_second_index = pnext;
                    }
                    matchL[next_first_index] = next_second_index;
                    // found = true;
                    break 'bfs;
                } else if !used_first[matchR[adj_second_index]] {
                    //Need to find a new matching for this value, put its left index on queue
                    used_first.set(matchR[adj_second_index], true);
                    queue[queueTail] = matchR[adj_second_index];
                    queueTail += 1;

                    back[matchR[adj_second_index]] = top_queue_first_word;
                }
            }
            if queueHead == queueTail {
                break;
            }
        }

        /*
                println!(
                    "After match attempt of first index {}.\nMatch Left:\n{}\nMatch Right:\n{}\n\
        Queue:\n{}\ntail: {}
                    ",
                    cur_first_word_id,
                    matchL
                        .iter()
                        .enumerate()
                        .map(|(first_id, second_id)| format!(
                            "{} => {}",
                            first_words[first_id as usize],
                            if *second_id < 0 {
                                "Invalid".to_string()
                            } else {
                                second_words[*second_id as usize].clone()
                            }
                        ))
                        .join("; "),
                    matchR
                        .iter()
                        .enumerate()
                        .map(|(second_id, first_id)| format!(
                            "{} => {}",
                            second_words[second_id as usize],
                            if *first_id < 0 {
                                "Invalid".to_string()
                            } else {
                                first_words[*first_id as usize].clone()
                            }
                        ))
                        .join("; "),
                    queue.iter().enumerate().map(
                        | (pos, first_id)
                        | format!(
                            "Queue pos #{} = {}",
                            pos,
                            first_words[*first_id as usize].clone()
                        )).join("\n"),
                    queueTail
                );*/

        //Reset all dice values in queue
        for j in 0..queueTail {
            used_first.set(queue[j], false);
        }

        cur_first_word_id += 1;
    }

    let match_count = matchL.iter().filter(|&&e| e != INVALID).count();

    topics.len()
        - match_count
        - (first_word_ids.len() - match_count)
        - (second_word_ids.len() - match_count)
}
