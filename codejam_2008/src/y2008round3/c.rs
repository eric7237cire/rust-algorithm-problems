use bit_vec::BitVec;
use codejam::util::codejam::run_cases;
use codejam::util::vector_2d::Vector2d;
use std::io::Write;
use std::usize;

/*
Bipartite matching (fast, non recursive implementation)
Maximum independent set
Grid
*/
pub fn solve_all_cases()
{
    run_cases(
        &["C-small-practice", "C-large-practice"],
        "y2008round3",
        |reader, buffer| {
            let t = reader.read_int();

            for case_no in 1..=t {
                let (n_rows, n_cols) = reader.read_tuple_2();

                let mut chairs = BitVec::from_elem(n_cols * n_rows, false);

                for r in 0..n_rows {
                    for (c, ch) in reader.read_chars(n_cols).into_iter().enumerate() {
                        let idx = r * n_cols + c;
                        chairs.set(idx, ch == '.');
                    }
                }

                if case_no != 3 {
                    //continue;
                }
                println!("Solving case {}", case_no);

                writeln!(
                    buffer,
                    "Case #{}: {}",
                    case_no,
                    solve(&chairs, n_rows, n_cols)
                )
                .unwrap();
            }
        },
    );
}

const INVALID: usize = usize::MAX;

fn index_to_vec(idx: usize, n_cols: usize) -> Vector2d<isize>
{
    Vector2d::with_val((idx / n_cols) as isize, (idx % n_cols) as isize)
}
fn vec_to_index(v: &Vector2d<isize>, n_cols: usize) -> usize
{
    v.r() as usize * n_cols + v.c() as usize
}
fn vec_comp_to_index(r: isize, c: isize, n_cols: usize) -> usize
{
    r as usize * n_cols + c as usize
}

fn solve(chairs: &BitVec, n_rows: usize, n_cols: usize) -> usize
{
    let adj_vecs: [Vector2d<isize>; 6] = [
        Vector2d::with_val(-1, -1),
        Vector2d::with_val(0, -1),
        Vector2d::with_val(1, -1),
        Vector2d::with_val(1, 1),
        Vector2d::with_val(0, 1),
        Vector2d::with_val(-1, 1),
    ];

    //odd indexed columns may have 1 less
    let right_size = (n_cols / 2) * n_rows;
    let right_n_cols = right_size / n_rows;
    let left_size = n_rows * n_cols - right_size;
    let left_n_cols = left_size / n_rows;

    assert_eq!(left_n_cols, n_cols / 2 + n_cols % 2);
    assert_eq!(right_n_cols, n_cols / 2);
    assert_eq!(left_n_cols + right_n_cols, n_cols);
    assert_eq!(left_size + right_size, chairs.len());

    //even col index 0,2,4
    let mut match_left = vec![INVALID; left_size];

    //odd col index 1,3,5
    let mut match_right = vec![INVALID; right_size];

    //Storing values
    let mut queue: Vec<usize> = vec![0; left_size];
    let mut back = vec![0; left_size];

    let mut used_first = BitVec::from_elem(left_size, false);

    let mut cur_left_col_idx = 0;
    while cur_left_col_idx < left_size {
        let mut queue_head = 0;
        let mut queue_tail = 1;
        queue[0] = cur_left_col_idx;
        used_first.set(cur_left_col_idx, true);
        back[cur_left_col_idx] = INVALID;

        'bfs: loop {
            assert!(queue[queue_head] != INVALID);
            let top_queue_left_idx = queue[queue_head];
            let mut top_queue_loc = index_to_vec(top_queue_left_idx, left_n_cols);
            top_queue_loc.data[1] *= 2;

            debug!(
                "Top loc (even/left) {:?} idx {} left cols {} rows {} cols {}",
                top_queue_loc, top_queue_left_idx, left_n_cols, n_rows, n_cols
            );

            queue_head += 1;

            if !chairs[vec_to_index(&top_queue_loc, n_cols)] {
                break;
            }

            for adj_odd_loc in adj_vecs.iter().map(|adj| *adj + &top_queue_loc) {
                if adj_odd_loc.r() < 0
                    || adj_odd_loc.r() >= n_rows as isize
                    || adj_odd_loc.c() < 0
                    || adj_odd_loc.c() >= n_cols as isize
                    || !chairs[vec_to_index(&adj_odd_loc, n_cols)]
                {
                    continue;
                }
                assert_eq!(adj_odd_loc.c() % 2, 1);
                let adj_right_index =
                    vec_comp_to_index(adj_odd_loc.r(), adj_odd_loc.c() / 2, right_n_cols);

                debug!(
                    "Adj odd / right loc {:?} idx {} right size {} right cols {} rows {}",
                    adj_odd_loc, adj_right_index, right_size, right_n_cols, n_rows
                );

                //Found a non matched second index
                if match_right[adj_right_index] == INVALID {
                    let mut next_right_index = adj_right_index;
                    let mut next_left_index = top_queue_left_idx;

                    match_right[adj_right_index] = next_left_index;
                    //Applying the augmenting path
                    while back[next_left_index] != INVALID {
                        assert!(back[next_left_index] != INVALID);
                        assert!(match_left[next_left_index] != INVALID);
                        let prev = back[next_left_index];
                        let pnext = match_left[next_left_index];
                        match_left[next_left_index] = next_right_index;
                        match_right[pnext] = prev;
                        next_left_index = prev;
                        next_right_index = pnext;
                    }
                    match_left[next_left_index] = next_right_index;

                    break 'bfs;
                } else if !used_first[match_right[adj_right_index]] {
                    //Need to find a new matching for this value, put its left index on queue
                    used_first.set(match_right[adj_right_index], true);
                    queue[queue_tail] = match_right[adj_right_index];
                    queue_tail += 1;

                    back[match_right[adj_right_index]] = top_queue_left_idx;
                }
            }
            if queue_head == queue_tail {
                break;
            }
        }

        //Reset all values in queue
        for qj in queue.iter().take(queue_tail) {
            used_first.set(*qj, false);
        }

        cur_left_col_idx += 1;
    }

    let match_count = match_left.iter().filter(|&&e| e != INVALID).count();

    chairs.iter().filter(|c| *c).count() - match_count
}
