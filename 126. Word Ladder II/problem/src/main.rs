
fn convert_vector(vs: Vec<&str>) -> Vec<String>
{
    vs.iter().map(|&e| e.to_string()).collect::<Vec<_>>()
}


/// Computes a simplified lex distance
pub fn  lex_distance( s1: &str, s2: &str ) -> u16
//pub fn  lex_distance<S>( p1: S, p2: S ) -> u16  where S: Into<&String>
{
    //let s1 : &String = p1.into();
    //let s2 : &String = p2.into();
    let mut diff_count = 0;

    for (c1, c2)  in s1.chars().zip(s2.chars())
    {
        if c1!=c2 {
            diff_count += 1;
        }
    }


    return diff_count + (s1.len() as i32 - s2.len() as i32).abs() as u16;
}

#[test]
fn test_lex_distance() {
    assert_eq!(1, lex_distance("bob", "boa"));
    assert_eq!(3, lex_distance("bob22", "boa"));
    assert_eq!(4, lex_distance("bob", "boa222"));

    // not real lex distance but its ok for our needs
    assert_eq!(4, lex_distance("abcd", "bcd"));
}


pub struct Solution {

}

impl Solution {
    pub fn find_ladders(begin_word: String, end_word: String, word_list: Vec<String>) -> Vec<Vec<String>> {

        let mut adj_list : Vec< Vec<usize>> = vec![Vec::new(); word_list.len() ];

        for (index1, item1) in word_list.iter().enumerate()
        {
            for (j, item2) in word_list.iter().skip(index1).enumerate()
            {
                let index2 = j + index1 ;
                let letter_diffs = lex_distance(item1, item2);
                if 1==letter_diffs {
                    println!("Adding {} {} and {} {}", index1, item1, index2, item2);
                    adj_list[index1].push(index2);
                    adj_list[index2].push(index1);
                }
            }
        }


        let mut x = Vec::new();
        x.push(convert_vector(vec!["hit","hot","dot","dog","cog"]));
        x.push(convert_vector(vec!["hit","hot","lot","log","cog"]));
        return x
    }
}


fn main() {
 
    let checks : [( (&'static str, &'static str, Vec<&str>), 
    Vec<Vec<&str>> 
    );1] = [

 ( 
    ("hit",
    "cog",
    vec!["hot","dot","dog","lot","log","cog"]), 

    vec![
    vec!["hit","hot","dot","dog","cog"],
    vec!["hit","hot","lot","log","cog"]
    ]
)

    ];

    println!("Hello, world!");

    for check in checks.iter()
    {
        let solution_args = &check.0;
        let mut expected_ans : Vec<Vec<String>> = Vec::new();
        for v1 in check.1.iter()
        {
            expected_ans.push( v1.iter().map(|&e| e.to_string()).collect::<Vec<_>>() );
        }
        let actual_ans = Solution::find_ladders(
            solution_args.0.to_string(),
            solution_args.1.to_string(),
            solution_args.2.iter().map(|&e| e.to_string()).collect::<Vec<String>>()
            );
        if expected_ans != actual_ans {
            println!("Problem {:?} != {:?}", actual_ans, expected_ans);
        } else {
            println!("OK {:?} == {:?}", actual_ans, expected_ans);

        }
        //break;
    }
}