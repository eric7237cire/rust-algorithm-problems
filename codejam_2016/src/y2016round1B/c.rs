use codejam::util::codejam::run_cases;

use itertools::Itertools;
//use std::cmp::min;
use bit_vec::BitVec;
use std::collections::HashMap;
use std::i64;
use std::io::Write;
use codejam::algo::graph::flow::FlowGraph;
/*


*/
pub fn solve_all_cases()
{
    run_cases(
        &[
            "C-small-practice",
            "C-large-practice"
        ],
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

const INVALID: i16 = -1;
const MAX_N: i16 = 1000;

fn solve(topics: &[Vec<String>]) -> usize
{
    let mut first_word_ids: HashMap<&str, i16> = HashMap::new();
    let mut second_word_ids: HashMap<&str, i16> = HashMap::new();
    let mut first_words = vec![String::new(); MAX_N as usize];
    let mut second_words = vec![String::new(); MAX_N as usize];



    let edges: Vec<[i16; 2]> = topics
        .iter()
        .map(|topic_words| {
            let next_first_id = first_word_ids.len() as i16;
            let next_second_id = second_word_ids.len() as i16;

            let first_id = *first_word_ids
                .entry(&topic_words[0])
                .or_insert(next_first_id);
            let second_id = *second_word_ids
                .entry(&topic_words[1])
                .or_insert(next_second_id);

            first_words[first_id as usize] = topic_words[0].clone();
            second_words[second_id as usize] = topic_words[1].clone();

            /*
            println!(
                "Edge from {}/{} to {}/{}",
                topic_words[0], first_id, topic_words[1], second_id
            );*/

            [first_id, second_id]
        })
        .collect();

    let source = first_word_ids.len() + second_word_ids.len();
    let sink = first_word_ids.len() + second_word_ids.len() + 1;
    let mut graph = FlowGraph::new(sink+1, 4);
    let a_start = 0usize;
    let b_start = first_word_ids.len();

    for a in 0..first_word_ids.len() {
        graph.add_edge(source, a, 1, 1);
    }

    //6 nodes in B
    for b in b_start..b_start + second_word_ids.len() {
        graph.add_edge(b, sink, 1, 1);
    }

    for edge in edges.iter() {
        graph.add_edge(a_start + edge[0] as usize, b_start + edge[1] as usize, 1, 1);
    }

    let (flow_amt, flow) = graph.dinic(source, sink);

    //create edges from first word to second word

    let mut matchL = vec![INVALID; first_word_ids.len()];

    //match second words
    let mut matchR = vec![INVALID; second_word_ids.len()];

    //Storing values
    let mut queue: Vec<i16> = vec![0; first_word_ids.len()];
    let mut back = vec![0; first_word_ids.len()];

    let mut used_first = BitVec::from_elem(first_word_ids.len(), false);
    let mut ans = 0;
    let mut cur_first_word_id = 0i16;
    while cur_first_word_id < first_word_ids.len() as i16 {
        let mut queueHead = 0;
        let mut queueTail = 1;
        queue[0] = cur_first_word_id;
        used_first.set(cur_first_word_id as usize, true);
        back[cur_first_word_id as usize] = INVALID;
        let mut found = false;
        'bfs: loop {
            assert!(queue[queueHead] >= 0);
            let mut top_queue_first_word = queue[queueHead];
            queueHead += 1;

            let first_to_second_edges: Vec<[i16; 2]> = edges
                .iter()
                .filter(|edge| edge[0] == top_queue_first_word)
                .cloned()
                .collect();
            for (j, adj_second_edge) in first_to_second_edges.iter().enumerate() {
                assert_eq!(top_queue_first_word, adj_second_edge[0]);

                let adj_second_index = adj_second_edge[1];
                //Found a non matched second index
                if matchR[adj_second_index as usize] == INVALID {
                    let mut next_second_index = adj_second_index;
                    matchR[adj_second_index as usize] = top_queue_first_word;
                    //Applying the augmenting path
                    while back[top_queue_first_word as usize] >= 0 {
                        assert!(back[top_queue_first_word as usize] >= 0);
                        assert!(matchL[top_queue_first_word as usize] >= 0);
                        let prev = back[top_queue_first_word as usize];
                        let pnext = matchL[top_queue_first_word as usize];
                        matchL[top_queue_first_word as usize] = next_second_index;
                        matchR[pnext as usize] = prev;
                        top_queue_first_word = prev;
                        next_second_index = pnext;
                    }
                    matchL[top_queue_first_word as usize] = next_second_index;
                    found = true;
                    break 'bfs;
                } else if !used_first[matchR[adj_second_index as usize] as usize] {
                    //Need to find a new matching for this value, put its left index on queue
                    used_first.set(matchR[adj_second_index as usize] as usize, true);
                    queue[queueTail] = matchR[adj_second_index as usize];
                    queueTail += 1;

                    back[matchR[adj_second_index as usize] as usize] = top_queue_first_word;
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
            used_first.set(queue[j] as usize, false);
        }

        cur_first_word_id += 1;
    }

    let match_count = matchL.iter().filter(|&&e| e >= 0).count();

    assert_eq!(flow_amt as usize, match_count);

    topics.len() - match_count - (first_word_ids.len() - match_count) - (second_word_ids.len() - match_count)
}
