
fn next_dir_loc(loc: &mut Vector2d<i64>, dir: &Vector2d<i64>, grid: &Grid<String>)
    -> Vector2d<i64>
{
    let loc_loc = &*loc;

    let new_dir = match dir {
        &NORTH => {
            if grid[loc_loc] == "/" {
                EAST
            } else {
                WEST
            }
        }
        &SOUTH => {
            if grid[loc_loc] == "/" {
                WEST
            } else {
                EAST
            }
        }
        &EAST => {
            if grid[loc_loc] == "/" {
                NORTH
            } else {
                SOUTH
            }
        }
        &WEST => {
            if grid[loc_loc] == "/" {
                SOUTH
            } else {
                NORTH
            }
        }
        _ => panic!("odd direction"),
    };

    *loc += &new_dir;

    new_dir
}

#[allow(dead_code)]
fn trace_path(
    initial_loc: &Vector2d<i64>,
    grid: &Grid<String>,
    initial_direction: &Vector2d<i64>,
) -> usize
{
    assert!(grid[initial_loc] == "/" || grid[initial_loc] == "\\");

    //Without lover ring
    let R = grid.R - 2;
    let C = grid.C - 2;

    let mut loc: Vector2d<i64> = initial_loc.clone();
    let mut dir: Vector2d<i64> = initial_direction.clone();

    loop {
        dir = next_dir_loc(&mut loc, &dir, grid);

        if let Some(lover) = grid_coords_to_lover(loc.data[0] as usize, loc.data[1] as usize, R, C)
        {
            return lover;
        }
    }
}

#[allow(dead_code)]
fn solve_brute_force(R: usize, C: usize, lovers: &[usize]) -> String
{
    //need 2 * R * C nodes
    //top is even, bottom is odd

    //Go through every subset
    assert!(R * C <= 16);

    let mut matches = vec![0; 2 * (R + C) + 1];
    for matched in lovers.chunks_exact(2) {
        let L1 = matched[0];
        let L2 = matched[1];
        matches[L1] = L2;
        matches[L2] = L1;
    }

    debug!("Matches: {:?}", matches);

    for subset in 0..1 << (R * C) {
        let mut grid: Grid<String> = Grid::new(R + 2, C + 2);
        let mut grid_garden: Grid<char> = Grid::new(R, C);

        for row in 0..R {
            for col in 0..C {
                let index = row * C + col;
                let is_forward = (subset >> index) & 1 > 0;
                grid[(row + 1, col + 1)] = if is_forward {
                    "/".to_string()
                } else {
                    "\\".to_string()
                };

                grid_garden[(row, col)] = if is_forward { '/' } else { '\\' };
            }
        }

        let mut lover_locations = Vec::new();

        let mut add_lover = |r, c, label: usize, initial_dir| {
            grid[(r, c)] = (label + 1).to_string();
            assert_eq!(Some(label + 1), grid_coords_to_lover(r, c, R, C));

            lover_locations.push((
                (label + 1),
                Vector2d::with_val(r as i64, c as i64),
                initial_dir,
            ));
        };

        //top
        for label in 0..C {
            let r = 0;
            let c = 1 + label;

            add_lover(r, c, label, SOUTH);
        }
        //right
        for label in C..C + R {
            let r = 1 + label - C;
            let c = C + 1;
            add_lover(r, c, label, WEST);
        }
        //bottom
        for label in C + R..2 * C + R {
            let r = 1 + R;
            let c = 2 * C + R - label;
            add_lover(r, c, label, NORTH);
        }
        //left
        for label in 2 * C + R..2 * (R + C) {
            let r = 2 * (R + C) - label;
            let c = 0;
            add_lover(r, c, label, EAST);
        }

        debug!("Subset {:b} Grid\n{:#.3?}\n", subset, grid);

        let mut valid = true;

        //top
        for (lover, loc, dir) in lover_locations {
            let other_lover = trace_path(&(loc.clone() + &dir), &grid, &dir);
            if matches[lover] != other_lover {
                valid = false;
                debug!(
                    "Lover {} in location {:?} matched with {}.  Valid? {}",
                    lover, loc, other_lover, valid
                );
                //break;
            }
        }

        if valid {
            return (0..R)
                .map(|r| {
                    (0..C)
                        .map(|c| grid_garden[&Vector2d::with_val(r, c)])
                        .join("")
                })
                .join("\n");
        }
    }

    "IMPOSSIBLE".to_string()
}


fn grid_coords_to_lover(r: usize, c: usize, R: usize, C: usize) -> Option<usize>
{
    //top
    if r == 0 {
        return Some(c);
    }
    //right
    if c == C + 1 {
        return Some(r + C);
    }
    //bottom
    if r == 1 + R {
        return Some(2 * C + R - c + 1);
    }
    //left
    if c == 0 {
        return Some(2 * (R + C) - r + 1);
    }

    None
}
