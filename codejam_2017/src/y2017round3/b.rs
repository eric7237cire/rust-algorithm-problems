use bit_set::BitSet;
use codejam::algo::graph::DiGraph;
use codejam::util::codejam::run_cases;
use std::io::Write;

/*
DFS
Spanning tree
Bridges
Cycles
Connected components
Double edges
Undirected graph
BFS
*/
pub fn solve_all_cases()
{
    run_cases(
        &["B-small-practice", "B-large-practice"],
        "y2017round3",
        |reader, buffer| {
            let t = reader.read_int();

            for case in 1..=t {
                let (f, p) = reader.read_tuple_2();
                let mut g = DiGraph::new();
                for f in 1..=f {
                    g.add_vertex(f);
                }
                let p = (0..p)
                    .map(|_| {
                        let (f1, f2) = reader.read_tuple_2();
                        g.add_edge(f1, f2);
                        (f1, f2)
                    })
                    .collect::<Vec<_>>();

                write!(buffer, "{}", solve(case, &g, &p, f)).unwrap();
            }
        },
    );
}

fn solve(case_no: u32, g: &DiGraph, p: &[(usize, usize)], f: usize) -> String
{
    debug!("\n\n\nSolving case {}", case_no);

    //Create an undirected graph with duplicates when u->v and v-> already exist in P
    let mut g_undirected = g.clone();
    for (u, v) in g_undirected.edges().collect::<Vec<_>>() {
        if g_undirected.has_edge(v, u) && v < u {
            g_undirected.add_edge_dups_ok(v, u);
            g_undirected.add_edge_dups_ok(u, v);
        } else {
            g_undirected.add_edge(v, u);
        }
    }

    debug!(
        "P is\n{:?}\nUndirected Graph is\n{:?}\n",
        p,
        g_undirected.edges().collect::<Vec<_>>(),
    );

    let mut edge_values: Vec<(usize, usize, i64)> = Vec::new();

    let mut bfs_visited = BitSet::new();

    for f in 1..=f {
        if bfs_visited.contains(f) {
            continue;
        }

        let cc = g_undirected.bfs(f).collect::<Vec<_>>();
        bfs_visited.extend(cc.clone());

        let mut sub_g = g_undirected.subgraph(&cc);
        //for (u, v) in subG.edges().collect::<Vec<_>>() {}
        debug!("CC {:?}\nsubG {:?}", cc, sub_g.edges().collect::<Vec<_>>());

        //spanning tree
        let mut st = DiGraph::new();

        let mut discovery_order = Vec::new();

        dfs(&mut discovery_order, &mut st, &sub_g, cc[0]);

        for st_edge in st.edges() {
            sub_g.remove_undirected_edge(st_edge.0, st_edge.1);
        }

        debug!(
            "For sub graph {:?} spanning tree is {:?}",
            sub_g.edges().collect::<Vec<_>>(),
            st.edges().collect::<Vec<_>>()
        );

        debug!("Discovery order is {:?} ", discovery_order);

        //Direct all edges in root-to-leaf direction
        for sub_g_edge in sub_g.edges().collect::<Vec<_>>() {
            let pos1 = discovery_order
                .iter()
                .position(|&d| d == sub_g_edge.0)
                .unwrap();
            let pos2 = discovery_order
                .iter()
                .position(|&d| d == sub_g_edge.1)
                .unwrap();

            if pos1 > pos2 {
                sub_g.remove_edge(sub_g_edge.0, sub_g_edge.1);
            }
        }

        debug!(
            "For sub graph directed root->leaf {:?}",
            sub_g.edges().collect::<Vec<_>>()
        );

        //root is automatically balanced
        discovery_order.reverse();
        discovery_order.pop();

        for current_node in discovery_order {
            let tree_children: Vec<_> = st.edges_from(current_node).collect();
            let tree_parents: Vec<_> = st.edges_to(current_node).collect();
            assert_eq!(tree_parents.len(), 1);
            let tree_parent = tree_parents[0];

            let mut balanced_value: i64 = 0;

            //These are edges not in the spanning tree that we assign 1 to
            for v in sub_g.edges_to(current_node) {
                //non_tree_edges_ancestor {
                /*Direct all edges in root-to-leaf direction
                 (we reverse or split edges after solving, as explained above).
                  We assign edges not in the tree a value of 1,
                that is, they send positive news from nodes to descendants. */

                edge_values.push((v, current_node, 1));
                balanced_value += 1;
            }

            //These are previously seen edges not in the spanning tree that we need to account for
            for _ in sub_g.edges_from(current_node) {
                //these have already been assigned
                balanced_value -= 1;
            }

            for t in tree_children {
                balanced_value -= edge_values
                    .iter()
                    //only count edges in the spanning tree which won't have a positive value
                    .filter(|&ev| ev.0 == current_node && ev.1 == t && ev.2 != 1)
                    .map(|ev| ev.2)
                    .sum::<i64>();
                //.get( &(dis_node, t) ).unwrap();
            }

            edge_values.push((tree_parent, current_node, -balanced_value));
        }
    }

    debug!("Edge values are {:?}", edge_values);

    if edge_values.iter().any(|ev| ev.2 == 0) {
        return format!("Case #{}: IMPOSSIBLE\n", case_no);
    }

    let mut ans: Vec<i64> = Vec::new();
    for fe in p {
        if let Some(pos) = edge_values.iter().position(|&e| e.0 == fe.0 && e.1 == fe.1) {
            ans.push(edge_values[pos].2);
            edge_values.remove(pos);
            continue;
        } else if let Some(pos) = edge_values.iter().position(|&e| e.0 == fe.1 && e.1 == fe.0) {
            ans.push(-edge_values[pos].2);
            edge_values.remove(pos);
            continue;
        } else {
            return format!("Case #{}: IMPOSSIBLE\n", case_no);
        }
    }

    /*
    let mut check_sums = vec![0; F];

    for (p, a) in P.iter().zip(ans.iter()) {
        check_sums[p.0 - 1] -= *a;
        check_sums[p.1 - 1] += *a;
    }

    if check_sums.iter().any(|cs| *cs != 0) {
        println!("Check sum failed: {:?} case {}", check_sums, case_no);
    }*/

    format!(
        "Case #{}: {}\n",
        case_no,
        ans.iter()
            .map(|a| a.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    )
}

fn dfs(discovery_order: &mut Vec<usize>, st: &mut DiGraph, sub_g: &DiGraph, u: usize)
{
    discovery_order.push(u);
    for v in sub_g.edges_from(u) {
        if !st.has_vertex(v) {
            //root to leaf direction
            st.add_edge(u, v);

            dfs(discovery_order, st, sub_g, v);
        }
    }
}
