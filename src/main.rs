#![allow(unused)]
use std::collections::{HashSet, BinaryHeap,HashMap,VecDeque};
use ordered_float::NotNan; // 2.7.0
use std::cmp::Reverse;

// references backtracking algo from https://towardsdatascience.com/solving-sudoku-with-ai-d6008993c7de


// problem from https://en.wikipedia.org/wiki/Sudoku
static TESTPROB: [usize; 81]= [
        5,3,0,0,7,0,0,0,0,
        6,0,0,1,9,5,0,0,0,
        0,9,8,0,0,0,0,6,0,
        8,0,0,0,6,0,0,0,3,
        4,0,0,8,0,3,0,0,1,
        7,0,0,0,2,0,0,0,6,
        0,6,0,0,0,0,2,8,0,
        0,0,0,4,1,9,0,0,5,
        0,0,0,0,8,0,0,7,9];

const DIM: usize = 9;
const DIM2: usize = DIM*DIM;

const DEBUG: bool = false;

fn alldiff(doms: &[HashSet<usize>], cur_neighbors: &HashSet<usize>, val: usize) -> bool {
    let mut ret: bool = true;

    for n in cur_neighbors.iter() {
        if doms[*n].len() == 0 || (doms[*n].contains(&val) && doms[*n].len() <= 1) {
            ret = false;
            break;
        };
    };

    ret
}
fn backtracking(_to_solve: [usize; DIM2], _doms: [HashSet<usize>; DIM2], _to_pick: &BinaryHeap<(Reverse<usize>, usize)>, _rn: &[HashSet<usize>], _cn: &[HashSet<usize>], _bn: &[HashSet<usize>]) -> (Option<[usize; DIM2]>, bool) {

    let mut picked: usize = DIM2;
    let mut to_pick = _to_pick.clone();
    let mut found_sol: bool = false;
    let mut ret_sol: Option<[usize;DIM2]> = None;
    let cur_choice = to_pick.pop();
    picked = match cur_choice {
        Some((domlen, x)) => x,
        None => DIM2
    };

    if picked == DIM2 {
        let mut allone: bool = true;

        for x in _doms.iter() {
            if x.len() != 1 {
                allone  = false;
                break;
            };
        };

        if allone == true {
          ret_sol = Some(_to_solve.clone());
          found_sol = true;
        };

    }
    else if picked < DIM2 {
        if DEBUG == true {
            println!("running on {}", picked);
        };
        let mut poss_values: Vec<usize> = Vec::new();
        for i in _doms[picked].iter() {
            //println!("{}", i);
            poss_values.push(*i);   
        };

        poss_values.sort();
        if DEBUG == true {
            println!("{:?}", poss_values);
        };
        for x in poss_values.iter() {
            if DEBUG == true {
                println!("checking {}", x);
            };
            let mut satisfies: bool = true;
            satisfies = satisfies & alldiff(&_doms, &_rn[picked], *x);
            if satisfies == true {
                satisfies = satisfies & alldiff(&_doms, &_cn[picked], *x);
            };
            if satisfies == true {
                satisfies = satisfies & alldiff(&_doms, &_bn[picked], *x);
            };

            if satisfies == true {
                let cur_solved: bool = false;
                let mut to_solve = _to_solve.clone();
                to_solve[picked] = *x;
                let mut doms = _doms.clone();
                doms[picked].clear();
                doms[picked].insert(*x);
                for n in _rn[picked].iter() {
                    if *n != picked {
                        doms[*n].remove(x);
                    };
                };

                for n in _cn[picked].iter() {
                    if *n != picked {
                        doms[*n].remove(x);
                    };
                };

                for n in _bn[picked].iter() {
                    if *n != picked {
                        doms[*n].remove(x);
                    };
                };


                                
                let bres = backtracking(to_solve, doms, &to_pick, _rn, _cn, _bn);
                if bres.1 == true { 
                    found_sol = true;
                    ret_sol = bres.0;
                    break;

                };
           
            };


        };
    };

    (ret_sol, found_sol)
}
fn solve(_to_solve: &[usize; DIM2]) -> Option<[usize; DIM2]>{
       let mut sp: [HashSet<usize>; DIM2] = [(); DIM2].map(|_| HashSet::from([1,2,3,4,5,6,7,8,9]));
       let mut rn: [HashSet<usize>; DIM2] = [(); DIM2].map(|_| HashSet::from([])); //row neighbors
       let mut cn: [HashSet<usize>; DIM2] = [(); DIM2].map(|_| HashSet::from([])); //col neighbors
       let mut bn: [HashSet<usize>; DIM2] = [(); DIM2].map(|_| HashSet::from([])); //block neighbors

       let block_idx: [usize; DIM] = [0,1,2,9,10,11,18,19,20];
       let mut given: HashSet<usize> = HashSet::new();
       let mut to_solve = _to_solve.clone();
        for i in 0..DIM2 {
            let row_start = (i/DIM) * DIM;
            for j in row_start..(row_start + DIM) {
                if j != i {
                    rn[i].insert(j);
                };
            };

            let col_start = i % DIM;
            let mut cur_col = col_start;
            while cur_col < DIM2 {
                if cur_col != i {
                    cn[i].insert(cur_col);
                };
                cur_col += DIM;
            };

            let block_start = ((i/DIM)/3 * (DIM2/3)) + (col_start/3 * 3);
            for j in block_idx.iter() {
                let cur_idx = block_start + j;
                if cur_idx != i {
                    bn[i].insert(cur_idx);
                };
            };
            //println!("{}, {}, {}, {}", i, row_start, col_start, block_start);
            let cval = to_solve[i];
            if cval > 0 {
                given.insert(i);
                sp[i].clear();
                sp[i].insert(cval);
                for j in rn[i].iter() {
                    sp[*j].remove(&cval);
                };
                for j in cn[i].iter() {
                    sp[*j].remove(&cval);
                };

                for j in bn[i].iter() {
                    sp[*j].remove(&cval);
                };
            };
       };

        if DEBUG == true {
            for (i,x) in sp.iter().enumerate() {
                println!("{}, {:?}", i, x);
            };
        };

       let mut to_pick: BinaryHeap<(Reverse<usize>, usize)> = BinaryHeap::new();

       for i in 0..DIM2 {
           if !given.contains(&i) {
                let clen = sp[i].len();
                if clen > 1 {
                    let to_insert = (Reverse(clen), i);
                    to_pick.push(to_insert);
                } else if clen == 1 {
                    let mut soleval:usize = 0;
                    for j in sp[i].drain() {
                        soleval = j;
                    };
                    sp[i].insert(soleval);
                    to_solve[i] = soleval;
                };
           };
       };
    let (ret_sol, found_sol) = backtracking(to_solve, sp, &to_pick, &rn, &cn, &bn);
    ret_sol
}

fn main() {
    let soln = solve(&TESTPROB);
    match soln {
        Some(x) => {
            for y in x.chunks(DIM) {
                println!("{:?}", y);
            };
        },

        None => println!("no solution found"),
        
    };
}
