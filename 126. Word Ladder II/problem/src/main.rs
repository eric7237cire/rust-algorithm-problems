
fn convert_vector(vs: Vec<&str>) -> Vec<String>
{
    vs.iter().map(|&e| e.to_string()).collect::<Vec<_>>()
}

pub struct Solution {

}

impl Solution {
    pub fn find_ladders(begin_word: String, end_word: String, word_list: Vec<String>) -> Vec<Vec<String>> {
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