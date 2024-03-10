use hashbrown::HashMap;
use std::fmt;
use itertools::Itertools; // 0.8.2

use algebra::Matrix;
use garside::{generate_descendants, generate_matrix_map};
use std::env;
use rand_pcg::Pcg32;
use rand::{Rng, SeedableRng};

mod algebra;
mod garside;

fn main() {
    let args: Vec<String> = env::args().collect();
    let p: i32 = args[1].parse().unwrap();
    let seed: u64 = if args.len() == 3 {
        args[2].parse().unwrap()
    } else { 0 };

    let mut rng = Pcg32::seed_from_u64(seed);

    let descendants = generate_descendants();
    let matrix_map = generate_matrix_map(p);

    let mut states: HashMap<i32, HashMap<i32, Vec<State>>> = HashMap::new();

    for i in 1..23 {
        let matrix = matrix_map[&i].clone();
        let factors = vec![i];
        let state = State::new(factors, matrix);
        let these_states = states.entry(2).or_default().entry(state.projlen()).or_default();
        these_states.push(state);
    }

    let mut l = 2;
    let mut lowest = *states.entry(l).or_default().keys().min().unwrap();
    let mut highest_seen_projlen = i32::MIN;
    let mut num_seen_by_projlen: HashMap<i32, i32> = HashMap::new();
    let res_size = 5000;
    println!("{:?}", states);
    loop {
        while states.entry(l).or_default().is_empty() {
                l += 1;
                println!("bump l {:?}",l);
                    for (key, value) in (states.entry(l).or_default().iter().sorted_by_key(|x| x.0 )).take(10) {
                     println!("{} / {}", key, value.len());
                    } 
                    for (key, value) in (states.entry(l).or_default().iter().sorted_by_key(|x| -x.0 )).take(10) {
                     println!("{} / {}", key, value.len());
                    }
        }
        if states.entry(l).or_default().entry(lowest).or_default().is_empty() {
            states.entry(l).or_default().remove(&lowest);
            
            lowest = *states.entry(l).or_default().keys().min().unwrap();
            if lowest > highest_seen_projlen {
                println!("Now considering elements with projlen {}", lowest);
                highest_seen_projlen = lowest;

            }
        }
        let these_states = states.entry(l).or_default().entry(lowest).or_default();
        let state = these_states.pop().unwrap();
        if these_states.is_empty() { 
            states.entry(l).or_default().remove(&lowest);
        }
        let last_factor = state.factors.last().unwrap();

        for descendant in &descendants[last_factor] {
            let matrix = matrix_map[descendant].clone();
            let new_state = state.append(*descendant, matrix);
            let this_projlen = new_state.projlen();
            *num_seen_by_projlen.entry(this_projlen).or_default() += 1;

            if this_projlen == 1 {
                println!("Found kernel element. Garside generators:");
                println!("{:?}", new_state.factors);
                return;
            }
                if this_projlen < 2*l +2 {
            let states_with_projlen = states.entry(l+1).or_default().entry(this_projlen).or_default();
            let mut added = false;
            if states_with_projlen.len() < res_size && this_projlen < 2*l+2 {
                states.entry(l+1).or_default().entry(this_projlen).or_default().push(new_state);
                added = true;
            } else {
                let x = rng.gen_range(0..states_with_projlen.len());
                if x < res_size  {
                    let _ = std::mem::replace(&mut states_with_projlen[x], new_state);
                    added = true;
                }
                }
            if added {
                if this_projlen < lowest {
                    lowest = this_projlen;
                }
            }
            }
        }
    }
}

pub struct State {
    pub factors: Vec<i32>,
    pub mat: Matrix,
}
impl State {
    pub fn new<'a>(factors: Vec<i32>, mat: Matrix) -> State {
        State { factors, mat }
    }

    pub fn projlen(&self) -> i32 {
        self.mat.projlen()
    }

    pub fn append(&self, factor: i32, mat: Matrix) -> State {
        let mut factors = self.factors.clone();
        factors.push(factor);
        let new_matrix: Matrix = &self.mat * &mat;
        State {
            factors,
            mat: new_matrix,
        }
    }
}


impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("State")
         .field("factors", &self.factors)
         .finish()
    }
}
