use codejam::util::codejam::run_cases;
use std::io::Write;

/*
*/
pub fn solve_all_cases()
{
    run_cases(
        &["D-small-practice",
            //"D-large-practice"
            ],
        "y2008beta",
        |reader, buffer| {
            let t = reader.read_int();

            for case_no in 1..=t {
                let positions = reader.read_num_line();
                let values = reader.read_num_line();

                if case_no != 1 {
                     continue;
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
struct Hex
{
    x: i16,
    y: i16,
    label: u16,
}

impl Hex
{
    fn new(x: i16, y: i16, label: u16) -> Self
    {
        Hex { x, y, label }
    }

    //   	#Number of jumps from one hex to another
    fn distance(&self, rhs: &Hex) -> i16
    {
        let lhs = self;
        let cy = (lhs.y - rhs.y).abs(); //change in y
        let cx = (lhs.x - rhs.x).abs(); //change in x
        if cy > cx {
            cy
        } else {
            (cy + (cx - cy) / 2)
        } //  #each cy gives you a free x move.  Each jump is 2
    }
}
//		@diag1, @diag2, @diag3 = -@x == @y, @x == @y, @y == 0

fn solve(positions: &[u16], values: &[u16]) -> u16
{
    let hex_size = positions.len() as i16;
    assert_eq!(values.len(), hex_size as usize);

    let small_hex_row_len = (hex_size + 1) / 2;

    debug!("S={} top/bottom row width={}", hex_size, small_hex_row_len);

    let mut hex_num = 1;
    let mut y = hex_size - small_hex_row_len;
    let mut hexes = Vec::new();

    let mut create_hexes = |row_len: i16| {
        for x in (-(row_len - 1)..row_len).step_by(2) {
            hexes.push(Hex::new(x, y, hex_num));
            hex_num += 1;
        }

        y -= 1;
    };

    for row_len in small_hex_row_len..=hex_size {
        create_hexes(row_len);
    }

    for row_len in (small_hex_row_len..=hex_size - 1).rev() {
        create_hexes(row_len);
    }

    for h in hexes.iter() {
        debug!("Hex x {} y {} label {}", h.x, h.y, h.label);
    }
    47
}
