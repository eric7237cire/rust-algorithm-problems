use codejam::util::codejam::run_cases;
use hungarian::minimize;
use std::io::Write;

/*
Hungarian algorithm
Munkres - Kuhn

Hexagon board
*/
pub fn solve_all_cases()
{
    run_cases(
        &["D-small-practice", "D-large-practice"],
        "y2008beta",
        |reader, buffer| {
            let t = reader.read_int();

            for case_no in 1..=t {
                let positions = reader.read_num_line();
                let values = reader.read_num_line();

                if case_no != 1 {
                    //continue;
                }

                println!("Solving case {}", case_no);

                writeln!(
                    buffer,
                    "Case #{}: {}",
                    case_no,
                    solve(positions.as_slice(), values.as_slice())
                )
                .unwrap();
            }
        },
    );
}

/*
    #y is the row, 0 for origin
    #x goes by 2's, each alternating row is offset

    #diags are the longest lines in the hex / \ and --

*/
#[derive(Debug)]
struct Hex
{
    x: i16,
    y: i16,
    label: usize,
}

impl Hex
{
    fn new(x: i16, y: i16, label: usize) -> Self
    {
        Hex { x, y, label }
    }

    //   	#Number of jumps from one hex to another
    fn distance(&self, rhs: &Hex) -> u16
    {
        let lhs = self;
        let cy = (lhs.y - rhs.y).abs() as u16; //change in y
        let cx = (lhs.x - rhs.x).abs() as u16; //change in x
        if cy > cx {
            cy
        } else {
            (cy + (cx - cy) / 2)
        } //  #each cy gives you a free x move.  Each jump is 2
    }
}

fn solve(positions: &[u16], values: &[u16]) -> u16
{
    let hex_size: usize = positions.len();
    assert_eq!(values.len(), hex_size as usize);

    let small_hex_row_len = (hex_size + 1) / 2;

    debug!("S={} top/bottom row width={}", hex_size, small_hex_row_len);

    let mut hex_num = 1;
    let mut y = (hex_size - small_hex_row_len) as i16;
    let mut hexes = Vec::new();

    let mut create_hexes = |row_len: i16| {
        for x in (-(row_len - 1)..row_len).step_by(2) {
            hexes.push(Hex::new(x, y, hex_num));
            hex_num += 1;
        }

        y -= 1;
    };

    for row_len in small_hex_row_len..=hex_size {
        create_hexes(row_len as i16);
    }

    for row_len in (small_hex_row_len..hex_size).rev() {
        create_hexes(row_len as i16);
    }

    let diag1 = hexes.iter().filter(|h| h.x == h.y).collect();
    let diag2 = hexes.iter().filter(|h| h.x == -h.y).collect();
    let diag3 = hexes.iter().filter(|h| 0 == h.y).collect();

    let diags: [Vec<&Hex>; 3] = [diag1, diag2, diag3];

    for h in hexes.iter() {
        debug!("Hex x {} y {} label {}", h.x, h.y, h.label);
    }

    let min_cost: u16 = diags
        .iter()
        .enumerate()
        .map(|(d, diag)| {
            debug!("Diag {} is {:?}", d, diag);

            //create weight matrix
            let cost_matrix: Vec<u16> = (0..hex_size * hex_size)
                .map(|row_col| {
                    //determines which initial pos/val
                    let row = row_col / hex_size;
                    //determines where in the diagonal
                    let col = row_col % hex_size;

                    let pos = positions[row as usize] as usize;
                    let cost = hexes[pos - 1].distance(diag[col as usize]) * values[row as usize];
                    assert_eq!(hexes[pos - 1].label, pos);

                    cost
                })
                .collect();

            let assignment = minimize(&cost_matrix, hex_size as usize, hex_size as usize);

            debug!("Assignment {:?}", assignment);

            assignment
                .iter()
                .enumerate()
                .map(|(idx, choice)| cost_matrix[idx * hex_size as usize + choice.unwrap()])
                .sum()
        })
        .min()
        .unwrap();

    min_cost
}
