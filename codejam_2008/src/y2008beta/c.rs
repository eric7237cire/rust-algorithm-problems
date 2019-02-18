use codejam::util::codejam::run_cases;
use itertools::Itertools;
use std::cmp::min;
use std::collections::HashMap;
use std::io::Write;
use std::u32;

/*
*/
pub fn solve_all_cases()
{
    run_cases(
        &[
            "C-small-practice",
            "C-large-practice"
        ],
        "y2008beta",
        |reader, buffer| {
            let t = reader.read_int();

            for case_no in 1..=t {
                let line1 = reader.read_string_line();
                let num_roads: u8 = line1[0].parse().unwrap();
                let starting_city = line1[1].clone();

                let roads: Vec<(String, String, u32)> = (0..num_roads)
                    .map(|_| {
                        let line = reader.read_string_line();
                        assert_eq!(3, line.len());
                        (line[0].clone(), line[1].clone(), line[2].parse().unwrap())
                    })
                    .collect();

                if case_no > 1 {
                   // continue;
                }

                println!("Solving case {}", case_no);

                writeln!(
                    buffer,
                    "Case #{}: {}",
                    case_no,
                    solve(&starting_city, &roads)
                        .iter()
                        .map(|f| format!("{:.7}", f))
                        .join(" ")
                )
                .unwrap();
            }
        },
    );
}

const DIST_INFINITY: u32 = u32::MAX / 2;

fn get_city_id(
    name: &str,
    cities_to_id: &mut HashMap<String, usize>,
    cities: &mut Vec<String>,
) -> usize
{
    let id_len = cities_to_id.len();
    let id = *cities_to_id.entry(name.to_string()).or_insert(id_len);
    if id >= cities.len() {
        cities.push(name.to_string());
    }
    id
}

fn find_all_shortest_paths(
    dest_node: usize,
    dist: &Vec<Vec<u32>>,
    roads: &Vec<Road>,
    paths: &mut Vec<Vec<usize>>,
)
{
    let min_dist = dist[0][dest_node];

    for road in roads
        .iter()
        .filter(|road| road.city_from == 0 && road.cost <= min_dist && min_dist - road.cost == dist[road.city_to][dest_node])
    {
        paths.push(vec![road.road_id]);
    }

    debug!("Starting shortest paths: {:?}", paths);

    let mut done = false;

    while !done {
        done = true;

        for path_idx in 0..paths.len() {
            let last_road = &roads[*paths[path_idx].last().unwrap()];
            let source_node = last_road.city_to;
            let min_dist = dist[source_node][dest_node];

            debug!("path_idx {} from {} min_dist {}", path_idx, source_node, min_dist);

            for (idx, r_tup) in roads
                .iter()
                .filter(|road| {
                    road.city_from == source_node
                        && road.cost <= min_dist
                        && min_dist - road.cost == dist[road.city_to][dest_node]
                })
                .enumerate()
            {
                if idx == 0 {
                    paths[path_idx].push(r_tup.road_id);
                    done = false;
                } else {
                    let mut new_path = paths[path_idx].clone();
                    new_path.pop();
                    new_path.push(r_tup.road_id);
                    paths.push(new_path);
                }
            }

            debug!("after paths: {:?}", paths);
        }
    }
}

struct Road
{
    city_from: usize,
    city_to: usize,
    cost: u32,
    road_id: usize,
}

fn solve(starting_city: &str, roads: &[(String, String, u32)]) -> Vec<f64>
{
    let mut cities_to_id: HashMap<String, usize> = HashMap::new();
    let mut cities: Vec<String> = Vec::new();

    let starting_id = get_city_id(&starting_city, &mut cities_to_id, &mut cities);

    assert_eq!(0, starting_id);

    let roads: Vec<Road> = roads
        .iter()
        .enumerate()
        .map(|(r_idx, r)| {
            let city_from = get_city_id(&r.0, &mut cities_to_id, &mut cities);
            let city_to = get_city_id(&r.1, &mut cities_to_id, &mut cities);
            Road {
                city_from,
                city_to,
                cost: r.2,
                road_id: r_idx,
            }
        })
        .collect();

    let num_cities = cities_to_id.len();

    debug!("Num cities {}", num_cities);

    let mut dist = vec![vec![DIST_INFINITY; num_cities]; num_cities];

    for r in roads.iter() {
        dist[r.city_from][r.city_to] = min(dist[r.city_from][r.city_to], r.cost);
    }

    for k in 0..num_cities {
        dist[k][k] = 0;
    }
    for k in 0..num_cities {
        for i in 0..num_cities {
            for j in 0..num_cities {
                dist[i][j] = min(dist[i][j], dist[i][k] + dist[k][j]);
            }
        }
    }

    for i in 0..num_cities {
        for j in 0..num_cities {
            if i == j {
                continue;
            }
            debug!("From city {} to {} is {}", cities[i], cities[j], dist[i][j]);
        }
    }

    //Need to know total # of reachable cities
    let reachable: Vec<usize> = (1..num_cities)
        .filter(|city| dist[0][*city] < DIST_INFINITY)
        .collect();

    let mut road_probs = vec![0f64; roads.len()];

    for &dest_city in reachable.iter() {
        let mut paths: Vec<Vec<usize>> = Vec::new();

        find_all_shortest_paths(dest_city, &dist, &roads, &mut paths);

        debug!(
            "Shortest paths from starting city {} to {}/{}.  # of paths: {}",
            cities[0],
            dest_city,
            cities[dest_city],
            paths.len()
        );

        for (paths_idx, path) in paths.iter().enumerate() {
            debug!("Shortest path {} of {}", paths_idx+1, paths.len());

            for (path_idx, r_idx) in path.iter().enumerate() {
                let road = &roads[*r_idx];
                debug!(
                    "Path step #{} of {}.  Road #{}: {}/{} => {}/{} cost: {}",
                    path_idx+1,
                    path.len(),
                    r_idx+1,
                    road.city_from,
                    cities[road.city_from],
                    road.city_to,
                    cities[road.city_to],
                    road.cost
                );

                road_probs[*r_idx] += (1.0 / reachable.len() as f64) / paths.len() as f64;
            }
        }
    }

    road_probs
}
