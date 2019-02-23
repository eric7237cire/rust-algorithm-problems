// maybe like Problem D. Descending in the Dark round 2 2012

/*
Bipartite matching
Grid
BFS
Cycles
Hard
*/
use codejam::algo::graph::flow::*;
use codejam::algo::graph::*;
use codejam::util::grid::constants::*;
use codejam::util::grid::{Grid, GridCoord, GridRowColVec};

//use std::thread;
use bimap::BiMap;
use bit_vec::BitVec;
use indexmap::IndexSet;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::default::Default;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::io::Write;
use std::sync::mpsc::channel;
use std::time::Instant;
use threadpool::ThreadPool;

pub fn solve_all_cases()
{
    run_cases(
        &["D-small-practice", "D-large-practice"],
        "y2017round2",
        |reader, buffer| {

            let t = reader.read_int();

            for case in 1..=t {
                let (c, r, m) = reader.read_tuple_3::<usize>();
                let mut grid: Grid<Tile> = Grid::new(r, c);
                for r in 0..r {
                    let row = reader.read_chars(c);
                    for (c, t) in row.iter().enumerate() {
                        grid[(r, c)] = Tile::from(*t);
                    }
                }

                let s = solve(case, &mut grid, m);
                write!(buffer, "{}", s).unwrap();
            }

        },
    );
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Tile
{
    Empty,
    Building,
    Soldier,
    Turret,
}

use self::Tile::*;
use codejam::util::codejam::run_cases;

impl Tile
{
    fn to_char(self) -> char
    {
        match self {
            Empty => '.',
            Building => '#',
            Soldier => 'S',
            Turret => 'T',
        }
    }
}

impl From<char> for Tile
{
    fn from(item: char) -> Self
    {
        match item {
            '.' => Empty,
            '#' => Building,
            'S' => Soldier,
            'T' => Turret,
            _ => panic!("Character not recognized: {}", item),
        }
    }
}
impl Display for Tile
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result
    {
        write!(f, "{}", self.to_char())
    }
}

impl Default for Tile
{
    fn default() -> Tile
    {
        Empty
    }
}

//problem specific code
fn reachable(grid: &Grid<Tile>, location: &GridCoord) -> HashSet<GridRowColVec>
{
    let mut r = HashSet::new();
    //debug!("\nTracing {} starting at {}", location, direction);

    for direction in DIRECTIONS.iter() {
        let mut loc: GridRowColVec = location.convert();

        for _ in 0..=grid.R + grid.C {
            loc += direction;

            if let Some(tile) = grid.get_value(&loc) {
                match *tile {
                    Building => {
                        break;
                    }
                    _ => {
                        r.insert(loc.clone());
                    }
                };
            } else {
                break;
            }
        }
    }

    r
}

/*
impl<L, R> FromIterator<(L, R)> for BiMap<L, R>
{
    fn from_iter<I: IntoIterator<Item = (L, R)>>(iter: I) -> Self
    {
        let mut c = BiMap::new();

        for i in iter {
            c.insert(i.0, i.1);
        }

        c
    }
}*/

fn solve<'a>(case_no: u32, grid: &mut Grid<Tile>, m_soldier_limit: usize) -> String
{
    debug!(
        "Solving case {}\nM={}\n",
        case_no, m_soldier_limit
    );

    //original solider & turret index to location map
    let s_map = grid
        .filter_by_val(&Soldier)
        .enumerate()
        .collect::<BiMap<_, _>>();
    let turret_locations = grid.filter_by_val(&Turret).collect::<Vec<_>>();

    //precalucate what squares a turret can reach
    let turret_reachable_squares_list = turret_locations
        .iter()
        .map(|t_loc| reachable(&grid, &t_loc))
        .collect::<Vec<_>>();

    let t_map = turret_locations
        .into_iter()
        .enumerate()
        .collect::<BiMap<_, _>>();

    let s = grid.filter_by_val(&Soldier).count();
    let t = grid.filter_by_val(&Turret).count();

    //Construct the initial Graph

    let g_edges = build_graph(
        &grid,
        false,
        m_soldier_limit,
        &s_map,
        &t_map,
        &turret_reachable_squares_list,
    );

    let mut g = FlowGraph::new(2 + s + t, 4);

    for uv in g_edges {
        g.add_edge(uv.0, uv.1, 1, 1);
    }

    let source = s + t;
    let sink = s + t + 1;

    let vertex_to_string = |v: usize| match v {
        s if s < s => format!("Soldier #{} ({:?})", s + 1, *s_map.get_by_left(&s).unwrap()),
        t if t >= s && t < s + t => format!(
            "Turret #{} ({:?})",
            t - s + 1,
            *t_map.get_by_left(&(t - s)).unwrap()
        ),
        v if v == sink => "Sink".to_string(),
        _source => "Source".to_string(),
    };

    //BFS for each soldier

    //will be in left to right order, then top down order

    //Now find max matching of G (G has an edge from soldier s to turret t if and only if soldier s can destroy turret t after all other turrets have been destroyed)
    for s in 0..s {
        g.add_edge(source, s, 1, 1);
    }

    for t in s..s + t {
        g.add_edge(t, sink, 1, 1);
    }

    let (r, flow) = g.dinic(source, sink);

    let mut ans = format!("Case #{}: {}\n", case_no, r);

    //Compute initial matching
    let mut m = flow
        .iter()
        .enumerate()
        .filter(|&(_e, f)| *f > 0)
        //map to u->v
        .map(|(e, _f)| (g.graph.endp[e ^ 1], g.graph.endp[e]))
        //leave out source and sink nodes
        .filter(|&(u, v)| u != source && v != sink)
        .collect::<Vec<_>>();

    debug!(
        "Edges in M initial matching=\n{}\n",
        m.iter()
            .map(|&(u, v)| format!("{}->{}", vertex_to_string(u), vertex_to_string(v)))
            .collect::<Vec<_>>()
            .join("\n")
    );

    let mut r = r;

    while r > 0 {
        //Let us define the graph G' with the same nodes as G, but an edge between soldier s and turret t only exists in G' if s can destroy t with the other turrets active
        let g_prime = build_graph(
            &grid,
            true,
            m_soldier_limit,
            &s_map,
            &t_map,
            &turret_reachable_squares_list,
        );

        //Now build graph H
        let mut h = Graph::new(s + t, 4);

        let soldiers_in_m = m.iter().map(|&(s, _t)| s).collect::<Vec<_>>();

        for &(s, t) in g_prime.iter() {
            if soldiers_in_m.contains(&s) {
                h.add_edge(s, t);
            }
        }
        for &(s, t) in m.iter() {
            h.add_edge(t, s);
        }

        debug!(
            "Current matching M =\n{}\n",
            m.iter()
                .map(|&(u, v)| format!("{}->{}", vertex_to_string(u), vertex_to_string(v)))
                .collect::<Vec<_>>()
                .join("\n")
        );

        debug!(
            "Edges in G'=\n{}\n",
            g_prime
                .iter()
                .map(|&(u, v)| format!("{}->{}", vertex_to_string(u), vertex_to_string(v)))
                .collect::<Vec<_>>()
                .join("\n")
        );

        debug!(
            "Edges in H=\n{}\n",
            h.edges()
                .map(|(u, v)| format!("{}->{}", vertex_to_string(u), vertex_to_string(v)))
                .collect::<Vec<_>>()
                .join("\n")
        );

        let turrets_in_m = m.iter().map(|&(_s, t)| t).collect::<Vec<_>>();
        //find an edge (s,t') where t' is not in m
        let st_prime = g_prime.iter().find(|&(_s, t)| !turrets_in_m.contains(t));

        if st_prime.is_some() {
            let &(s, t) = st_prime.unwrap();
            debug!("Found (s,t') s={} t'={}", s, t - s);
            ans += &format!("{} {}\n", s + 1, t - s + 1);

            grid[s_map.get_by_left(&s).unwrap()] = Empty;
            grid[t_map.get_by_left(&(t - s)).unwrap()] = Empty;
            r -= 1;

            //Also remove from current matching
            let to_remove = m
                .iter()
                .position(|&(s_in_m, _t)| s_in_m == s)
                .expect("Soldier should be in mapping");
            m.remove(to_remove);

            continue;
        }

        //Now we need to find a cycle

        //Start at a soldier in H
        let soldier_in_h = h.edges().filter(|&(u, _v)| u <= s).next().unwrap().0;

        let mut cycle_edges = VecDeque::new();
        let mut edge = (
            soldier_in_h,
            h.adj_list_with_edges(soldier_in_h).next().unwrap().1,
        );
        let mut visited = BitVec::from_elem(h.num_v(), false);

        while !visited[edge.0] {
            visited.set(edge.0, true);
            cycle_edges.push_back(edge);
            debug!(
                "pushed Edge {:?} ",
                format!("{}->{}", vertex_to_string(edge.0), vertex_to_string(edge.1))
            );
            //adj list returns an (internal edge index, next vertex)
            edge = (edge.1, h.adj_list_with_edges(edge.1).next().unwrap().1);
            debug!("Edge {:?} ", edge);
        }

        //cut to the actual cycle found
        let cycle_end = cycle_edges.back().unwrap().1;
        let cycle_start = cycle_edges
            .iter()
            .position(|&(u, _v)| u == cycle_end)
            .unwrap();
        cycle_edges.drain(0..cycle_start);

        debug!(
            "Cycle C =\n{}\n",
            cycle_edges
                .iter()
                .map(|&(u, v)| format!("{}->{}", vertex_to_string(u), vertex_to_string(v)))
                .collect::<Vec<_>>()
                .join("\n")
        );

        //Consider a new matching M' of G consisting of the edges of M whose reverse is not in C, p
        // lus the edges in C whose reverse is not in M. That is, M' is M but exchanging the edges
        // present in C in some direction. M' in this case is also a matching of G of the same size as M

        //because it is a cycle, we know we have new edges from G' to replace the ones removed from M
        let mut m_new: Vec<(usize, usize)> = Vec::new();
        m_new.extend(m.iter().filter(|&&(u, v)| !cycle_edges.contains(&(v, u))));
        m_new.extend(cycle_edges.iter().filter(|&&(u, v)| !m.contains(&(v, u))));

        debug!(
            "New matching M =\n{}\n",
            m_new
                .iter()
                .map(|&(u, v)| format!("{}->{}", vertex_to_string(u), vertex_to_string(v)))
                .collect::<Vec<_>>()
                .join("\n")
        );

        //Find all edges from G' which are actions we can take
        let st_actions = m_new
            .iter()
            .filter(|&uv| g_prime.contains(uv))
            .collect::<Vec<_>>();
        for &&(s, t) in st_actions.iter() {
            debug!("Taking actions from g' s {} t {}", s + 1, t + 1 - s);
            ans += &format!("{} {}\n", s + 1, t - s + 1);

            grid[s_map.get_by_left(&s).unwrap()] = Empty;
            grid[t_map.get_by_left(&(t - s)).unwrap()] = Empty;

            r -= 1;
        }

        m = m_new;
    }

    ans
}

fn build_graph(
    grid: &Grid<Tile>,
    is_g_prime: bool,
    m: usize,
    s_mapping: &BiMap<usize, GridCoord>,
    t_mapping: &BiMap<usize, GridCoord>,
    turret_reachable_squares_list: &Vec<HashSet<GridRowColVec>>,
) -> IndexSet<(usize, usize)>
{
    let mut g: IndexSet<(usize, usize)> = IndexSet::new();

    let turret_locations = grid.filter_by_val(&Turret).collect::<HashSet<_>>();

    /*

        for (turret_index, turret_squares) in turret_squares_list.iter().enumerate() {
            debug!("Turret {} can see {:?}", turret_index, turret_squares);
        }
    */
    let soldier_locations = grid.filter_by_val(&Soldier).collect::<Vec<_>>();

    let s = soldier_locations.len();
    let t = turret_reachable_squares_list.len();

    for (_soldier_index, soldier_loc) in soldier_locations.iter().enumerate() {
        //debug!("BFS search on soldier {} @ {}", soldier_index, soldier_loc);

        //Node is location, distance, seen_turret
        let mut queue: VecDeque<(GridRowColVec, usize, bool)> = VecDeque::new();
        let mut visited = BitVec::from_elem(grid.C * grid.R, false);

        queue.push_back((soldier_loc.convert(), 0, false));
        visited.set(soldier_loc.data[0] * grid.C + soldier_loc.data[1], true);

        while !queue.is_empty() {
            let (loc, dist, seen_turret) = queue.pop_front().unwrap();

            let visible_turrets = turret_reachable_squares_list
                .iter()
                .enumerate()
                .filter(|(turret_index, turret_squares)| {
                    turret_locations.contains(t_mapping.get_by_left(turret_index).unwrap())
                        && turret_squares.contains(&loc)
                })
                .map(|(turret_index, _)| turret_index);

            let mut turret_visible = false;
            for turret_index in visible_turrets {
                turret_visible = true;
                if !is_g_prime || (!seen_turret && is_g_prime) {
                    let s_vertex = *s_mapping.get_by_right(soldier_loc).unwrap();

                    //The turret index is already using the original grids index

                    /*debug!("Found s{} t{} mapped to soldier {} => {} at loc {}",
                    soldier_index, turret_index, s_vertex, t_vertex, loc);*/
                    g.insert((s_vertex, s_mapping.len() + turret_index));
                }
            }

            //no need to queue once we have been shot by a turret
            if is_g_prime && turret_visible {
                continue;
            }
            /*
            debug!(
                "Viewing {} dist {} seen turret? {} turret visible? {}",
                loc, dist, seen_turret, turret_visible
            );*/

            for dir in DIRECTIONS.iter() {
                let new_loc = loc.clone() + dir;

                if let Some(tile) = grid.get_value(&new_loc) {
                    if *tile == Building {
                        continue;
                    }

                    let new_loc_index = (new_loc.data[0] * grid.C as i64 + new_loc.data[1]) as usize;
                    if visited[new_loc_index] {
                        continue;
                    }
                    visited.set(new_loc_index, true);
                    let new_dist = dist + 1;

                    if new_dist > m {
                        continue;
                    }

                    let new_seen_turret = seen_turret || turret_visible;

                    queue.push_back((new_loc, new_dist, new_seen_turret));
                }
            }
        }
    }

    debug!("Built graph from\n  S={} T={}", s, t);
    g
}

/*
impl Display for Grid<Tile>
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result
    {
        for r in 0..self.R {
            for c in 0..self.C {
                if let Err(err) = write!(f, "{}", self[(r, c)]) {
                    return Err(err);
                }
            }
            if let Err(err) = writeln!(f, "") {
                return Err(err);
            }
        }
        write!(f, "")
    }
}*/
