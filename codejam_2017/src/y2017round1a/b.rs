use codejam::util::codejam::run_cases;
use std::io::Write;

/*
scan line
constraints
greedy
floating point issues

code not mine, just translation
*/
pub fn solve_all_cases()
{
    run_cases(
        &["B-small-practice", "B-large-practice"],
        "y2008round1a",
        |reader, buffer| {
            let t = reader.read_int();

            for case_no in 1..=t {
                //handle input / output

                let (n, p) = reader.read_tuple_2();

                let r: Vec<u32> = reader.read_num_line();

                let mut q: Vec<Vec<u32>> = Vec::new();
                for _ in 0..n {
                    q.push(reader.read_num_line());
                }

                println!("Solving case {}", case_no);

                writeln!(buffer, "Case #{}: {}", case_no,
                         solve(case_no, n, p, &r, &q)
                ).unwrap();

            }
        },
    );
}

fn solve(case_no: u32, n: u8, p: u8, r: &Vec<u32>, q: &Vec<Vec<u32>>) -> String
{
    debug!("\nStarting solve");
    let mut events: Vec<_> = Vec::new();
    for i in 0..n as usize {
        let required_amount = r[i];

        for p in 0..p as usize {
            let package_size = q[i][p];

            // problem is floating point
            //min_servings = math.ceil(package_size / (1.1 * required_amount) )
            // max_servings = math.floor( package_size / (.9 * required_amount) )

            let max_servings = (10 * package_size) / (9 * required_amount);
            let mut min_servings =
                (10 * package_size + 11 * required_amount - 1) / (11 * required_amount);

            debug!(
                "For ingredient {i}, package # {p}. \
                 Required per serving = {required_amount} \
                 Package size = {package_size} \
                 Min = {min_servings} Max = {max_servings}",
                i = i,
                p = p,
                required_amount = required_amount,
                package_size = package_size,
                min_servings = min_servings,
                max_servings = max_servings
            );

            if min_servings == 0 {
                min_servings = 1;
            }

            if min_servings > max_servings {
                continue;
            }

            events.push((min_servings, false, i, package_size));
            events.push((max_servings, true, i, package_size));
        }
    }

    // Code based on https://www.go-hero.net/jam/17/name/Nore
    events.sort();
    let mut cnt = 0;
    let mut counts = vec![Vec::new(); n.into()];
    let mut remv = vec![0; n as usize];
    for (boundary, is_upper_bound, ingredient_index, package_size) in events {
        debug!(
            "Saw event Boundary={} {} ingredient={} package={}",
            boundary, is_upper_bound, ingredient_index, package_size
        );

        debug!("Counts={:?}, remv={:?}", counts, remv);
        if is_upper_bound {
            if remv[ingredient_index] > 0 {
                remv[ingredient_index] -= 1;
            }
            // elif yy in counts[i]:
            else {
                let index = counts[ingredient_index]
                    .iter()
                    .position(|x| *x == package_size)
                    .unwrap();
                counts[ingredient_index].remove(index);
            }
        } else {
            counts[ingredient_index].push(package_size);
            let min_count_len = counts.iter().map(|c| c.len()).min().unwrap();
            if min_count_len > 0 {
                cnt += 1;
                for ii in 0..n as usize {
                    let min_index = counts[ii]
                        .iter()
                        .enumerate()
                        .min_by_key(|&(_, item)| item)
                        .unwrap()
                        .0;
                    counts[ii].remove(min_index);
                    remv[ii] += 1;
                }
            }
        }
    }

    format!("Case #{}: {}\n", case_no, cnt)
}
