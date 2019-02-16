
use codejam::util::codejam::run_cases;
//use itertools::Itertools;

use std::io::Write;
use codejam::util::bitvec64::BitVec64;
use std::collections::HashMap;
use codejam::util::vector_2d::Vector2d;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

/*
Dynamic programming
*/
pub fn solve_all_cases()
{
    run_cases(
        &["D-small-practice",
            //"D-large-practice"
            ],
        "y2008practice",
        |reader, buffer| {

            let t = reader.read_int();

            for case_no in 1..=t {
                let (ni, ns, pg) = reader.read_tuple_3();

                let mut items = reader.read_string_line();
                let mut items_map:HashMap<String,usize> = HashMap::new();
                assert_eq!(ni, items.len());
                let mut is_perishable = BitVec64::new();

                for i in items.iter_mut()
                {
                    if i.chars().last().unwrap() == '!' {
                        assert_eq!('!', i.remove(i.len() - 1));
                        is_perishable.set( items_map.len(), true);
                    }

                    items_map.insert(i.clone(), items_map.len());
                }

                let mut stores: Vec<Store> = (0..ns).map( |_| {
                    let s = reader.read_string();
                    Store::new(&s, ni, &items_map)
                }).collect();

                stores.insert(0, Store::new(&"0 0".to_string(), ni, &items_map));

                let mut memo = Memo::new(stores.len(), ni);

                if case_no != 1 {
                    //continue;
                }

                println!("Solving case {}", case_no);

                let ans = memo.doit( pg as f64, &is_perishable, &mut stores);

                writeln!(buffer, "Case #{}: {:.7}", case_no, ans).unwrap();
            }
        },
    );
}


struct Stock {

spent: f64,
//basket count
bct: u32,
loc: usize,
//from location?
cf: usize,
mask: BitVec64
}

impl Eq for Stock {

}
impl PartialEq for Stock {
    fn eq(&self, rhs: &Stock) -> bool {
        self.spent == rhs.spent &&
            self.mask.data == rhs.mask.data &&
            self.cf == rhs.cf

    }
}

impl Stock {
    fn new(spent: f64, mask: BitVec64, loc: usize) -> Self
    {
        Stock {
            spent,
            mask,
            loc,
            cf: 0,
            bct: mask.pop_count()
        }
    }
    fn new2(spent: f64, mask: BitVec64, loc: usize, b: u32, cf: usize) -> Self {
        Stock {
            spent,
            mask,
            loc,
            cf,
            bct: b
        }
    }
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for Stock {
    fn cmp(&self, rhs: &Stock) -> Ordering {
        rhs.spent.partial_cmp(&self.spent).unwrap().then_with(||
        rhs.loc.cmp(&self.loc)
        ).then_with( || rhs.mask.data.cmp(&self.mask.data))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for Stock {
    fn partial_cmp(&self, rhs: &Stock) -> Option<Ordering> {
        Some(self.cmp(rhs))
    }
}

struct Store {
    x: i32,
    y: i32,
    //prices[item index] = price
    prices: Vec<i32>,
    //Which items are in stock stk[0] = item index1, stk[1] = item index 2
    stk: Vec<usize>
}

impl Store {
    fn new(s: &String, n: usize, items: &HashMap<String, usize>) -> Self
    {
        let mut prices = vec![-1; n];
        let st: Vec<&str>  = s.split(|c| c == ' ' || c == ':' || c == '\n').collect();
	    let x : i32 = st[0].parse().unwrap();
	    let y: i32 = st[1].parse().unwrap();
	    let mut stk = Vec::new();
	    //let mut j = 0;
	    for chunk in st.chunks_exact(2).skip(1) {
            let item_str = chunk[0];
            let price = chunk[1];
            let item_idx = items[item_str];
            prices[item_idx] = price.parse().unwrap();
            stk.push(item_idx);
        }

        Store { x, y, prices, stk}

	}

    fn dist(&self, xp: i32, yp: i32) -> f64 { return Vector2d::with_val(self.x, self.y).pyth_distance( &Vector2d::with_val(xp, yp))}

}

const EPS: f64 = 1e-10;
const INVALID_COST:f64 = 1e100;

struct Memo
{
    //bcost[store][binary mask of items] = cost
    bcost: Vec<Vec<f64>>
}

impl Memo
{
    fn new(ns: usize, ni: usize) -> Self {
        Memo {
            bcost: vec![vec![INVALID_COST; 1<<ni]; ns]

        }
    }
    fn doit(&mut self, gp: f64, per: &BitVec64, sts: &mut Vec < Store > ) -> f64 {
        let ni = sts[0].prices.len();
        //let ns = sts.len();

	    let all = (1<<ni)-1;


        let mut ub = 0.0;
        //looks like the cheapest prices
        let mut chp = vec![INVALID_COST; ni];

        //0 is home
        for  store in sts.iter().skip(1) {
            //gas cost
            let gc = store.dist(0, 0)*2.*gp;

            for &item_idx in store.stk.iter()  {
                let tc = gc + store.prices[item_idx] as f64;
                if tc < chp[item_idx] {
                    chp[item_idx] = tc;
                }
            }
		}

        ub += chp.iter().sum::<f64>();

        assert!(ub < INVALID_COST);

        self.bcost[0][all] = ub;

        //Remove items that are definitely not optimal
        for  store in sts.iter_mut().skip(1) {

            let keep:Vec<bool> = store.stk.iter().map( |&item_idx|
            {
                store.prices[item_idx] as f64 <= chp[item_idx]
            }).collect();

            for (j, k) in keep.iter().enumerate().rev() {
                if !k {
                    store.stk.remove(j);
                }
            }
/*
            store.stk.retain(|&item_idx|
                store.prices[item_idx] as f64 <= chp[item_idx]
            );
*/
		}


        self.bcost[0][0] = 0.0;
        let mut pq = BinaryHeap::new();

        pq.push(Stock::new2(0.0, BitVec64::new(), 0, 0, sts.len()));

        while let Some(s) = pq.pop() {

            debug!("Examining Stock( Spent={},Mask={:0>width$b},Location={},Basket Count={},Cf={})",
                   s.spent, s.mask.data, s.loc, s.bct, s.cf,
            width=ni
            );

            if self.bcost[0][all] <= s.spent {
                continue;
            }
            if self.bcost[s.loc][s.mask.data] < s.spent {
                continue;
            }
            if s.loc > 0 {
                let store = &sts[s.loc];
                let mut stk: Vec<usize> = Vec::new();
                for &item_index in store.stk.iter() {
                    if !s.mask.get(item_index) {
                        stk.push(item_index);
                    }
                }

                if stk.is_empty() {
                    continue;
                }
                let n = 1<<stk.len();
                //mask
                for m in 1..n  {
                    let mut cost = s.spent;
                    //new mask
                    let mut nm = s.mask;
                    //perishable count
                    let mut pr = 0;
                    let mut nct = s.bct;
                    let mask = BitVec64::with_val(m);

                    for (i, &item_index) in stk.iter().enumerate() {
                        if !mask.get(i) {
                            continue;
                        }

                        cost += store.prices[item_index] as f64;

                        nm.set(item_index, true);
                        nct += 1;
                        if per.get(item_index) {
                            pr += 1;
                        }
                    }

                    if self.bcost[s.loc][nm.data] < cost+EPS {
                        continue;
                    }

                    if pr > 0 {
                        let gc = store.dist(0,0) * gp + cost;

                        if self.bcost[0][nm.data] > gc+EPS {
                            self.bcost[0][nm.data] = gc;
                            pq.push(Stock::new2(gc, nm, 0, nct, s.loc));
                            debug!(" Push Stock({},{:b},{},{},{})\n", gc, nm.data, 0, nct, s.loc);
                        }
                    } else {
                        self.bcost[s.loc][nm.data] = cost;
                        for (i, other_store) in sts.iter().enumerate() {
                            if s.loc == i {
                                continue;
                            }
                            let gc = other_store.dist(store.x, store.y) * gp + cost;

                            if self.bcost[i][nm.data] > gc+EPS {
                                self.bcost[i][nm.data] = gc;
                                pq.push(Stock::new2(gc, nm,
                                    i, nct, s.loc));
                                debug!(" Push Stock({},{},{},{},{})\n", gc, nm.data, i, nct, s.loc);
                            }
                        }
                    }
                }
            } else {
                for (i,store) in sts.iter().enumerate().skip(1) {
                    let gc = store.dist(0,0) *gp + s.spent;

                    if self.bcost[i][s.mask.data] > gc+EPS {
                        self.bcost[i][s.mask.data] = gc;
                        pq.push(Stock::new2(gc, s.mask, i, s.bct, 0));
                        debug!(" Push Stock({},{},{},{},{})\n", gc, s.mask.data, i, s.bct, 0);
                    }
                }
            }
        }


	    self.bcost[0][all]
    }
}

#[cfg(test)]
mod tests
{

}
