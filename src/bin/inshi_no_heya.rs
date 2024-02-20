// label: cut-off, random, sparce expected
// name: inshi_no_heya

//長方形のサイズは現実的にsqrt(n*m)程度なのでcut-off

use indicatif::{ProgressBar, ProgressStyle};
use puzzle_check::common::function::{
    all_different, compare_structures, cycle, extract_contains_structures,
    extract_random_structure, power_set, progress_size, random_subset_with_validation,
};
use puzzle_check::common::initialize::initialize;
use puzzle_check::common::predicates::is_rectangle;
use puzzle_check::common::relationship::{relationship, Relationship, D, H, M, V};
use puzzle_check::common::{
    dataclass::{Attribute, BoardSize, Composition, Coordinate, Element, Structure},
    function::add_up_structures,
};
use puzzle_check::specific::board::non_validation;
use puzzle_check::specific::graph::only_cycle;
use puzzle_check::{
    common::combine::{combine, non_cutoff, ValidationFn},
    specific::board::BoardValidationFn,
};
use rayon::prelude::*;
use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use std::thread;

const n: i32 = 3;
const board_size: BoardSize = BoardSize(n, n);
const LOOP_NUMBERS: u64 = 100000;

fn size_limitation_n(area: &Structure) -> bool {
    if let Structure::Composition(ref area_content) = area {
        return area_content.entity.len() as i32 == n;
    } else {
        unreachable!()
    }
}

fn main() {
    let pb = ProgressBar::new(LOOP_NUMBERS);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("main    {bar:40.cyan/blue} {pos}/{len} {percent}% {eta}")
            .unwrap(),
    );

    let print_lock = Arc::new(Mutex::new(()));

    let (P, C, Ep, Ec) = initialize(&board_size);

    // ----------------------------------------------------------------------
    let room_R: Vec<Relationship> = vec![H, V];
    let not_room_R: Vec<Relationship> = vec![M];
    let cutoff_functions_for_room: Vec<ValidationFn> = vec![is_rectangle];
    let row_R: Vec<Relationship> = vec![H];
    let not_row_R: Vec<Relationship> = vec![D, V, M];
    let col_R: Vec<Relationship> = vec![V];
    let not_col_R: Vec<Relationship> = vec![H, D, M];
    let cutoff_functions_for_row: Vec<ValidationFn> = vec![size_limitation_n];
    let cutoff_functions_for_col: Vec<ValidationFn> = vec![size_limitation_n];
    let room_A = combine(room_R, not_room_R, &C, &cutoff_functions_for_room);
    let row_A = combine(row_R, not_row_R, &C, &cutoff_functions_for_row);
    let col_A = combine(col_R, not_col_R, &C, &cutoff_functions_for_col);

    // combineの確認---------------------------
    // println!("{:?}", A.len());
    // for a in A.iter() {
    //     println!("{:?}", a);
    // }
    // ---------------------------------------

    let board_validation_functions: Vec<BoardValidationFn> = vec![non_validation];

    let max_a = 99999;

    let P_domain: Vec<Option<i32>> = vec![None];
    let C_domain: Vec<Option<i32>> = (1..=n).map(Some).collect();
    let Ep_domain: Vec<Option<i32>> = vec![None];
    let Ec_domain: Vec<Option<i32>> = vec![None];
    let A_domain: Vec<Option<i32>> = (0..=max_a).map(Some).collect();

    let P_domain_size = P_domain.len();
    let C_domain_size = C_domain.len();
    let Ep_domain_size = Ep_domain.len();
    let Ec_domain_size = Ec_domain.len();
    let A_domain_size = A_domain.len();

    let total_combinations_P = P_domain_size.pow(P.len() as u32);
    let total_combinations_C = C_domain_size.pow(C.len() as u32);
    let total_combinations_Ep = Ep_domain_size.pow(Ep.len() as u32);
    let total_combinations_Ec = P_domain_size.pow(Ec.len() as u32);

    (0..LOOP_NUMBERS).into_par_iter().for_each(|_| {
        pb.inc(1);
        let print_lock_clone = Arc::clone(&print_lock);
        let mut B = Structure::Composition(Composition::new(vec![]));
        let mut power_A: Vec<Structure> = vec![];
        'inner: for i in 0..100000 {
            if let Structure::Composition(ref B_content) = B {
                if B_content.entity.len() == C.len() {
                    break 'inner;
                }
            }
            let new_area = extract_random_structure(&room_A);
            if relationship(&new_area, &B, M) {
                continue 'inner;
            }
            B = add_up_structures(&B, &new_area);
            power_A.push(new_area);
        }
        (0..total_combinations_P).into_par_iter().for_each(|pi| {
            let mut independent_P = P.clone();
            let mut index_pi = pi;

            for structure_p in independent_P.iter_mut() {
                if let Structure::Element(ref mut point) = structure_p {
                    let digit = index_pi % P_domain_size;
                    index_pi /= P_domain_size;
                    point.val = P_domain[digit];
                }
            }
            (0..total_combinations_Ep).into_par_iter().for_each(|epi| {
                let mut independent_Ep = Ep.clone();
                let mut index_epi = epi;

                for structure_ep in independent_Ep.iter_mut() {
                    if let Structure::Element(ref mut ep_content) = structure_ep {
                        let digit = index_epi % Ep_domain_size;
                        index_epi /= Ep_domain_size;
                        ep_content.val = Ep_domain[digit];
                    }
                }

                (0..total_combinations_Ec).into_par_iter().for_each(|eci| {
                    let mut independent_Ec = Ec.clone();
                    let mut index_eci = eci;

                    for structure_ec in independent_Ec.iter_mut() {
                        if let Structure::Element(ref mut ec_content) = structure_ec {
                            let digit = index_eci % Ec_domain_size;
                            index_eci /= Ec_domain_size;
                            ec_content.val = Ec_domain[digit];
                        }
                    }

                    (0..total_combinations_C).into_par_iter().for_each(|ci| {
                        let mut independent_C = C.clone();
                        let mut index_ci = ci;
                        let mut success = true;

                        for structure_c in independent_C.iter_mut() {
                            if let Structure::Element(ref mut c_content) = structure_c {
                                let digit = index_ci % C_domain_size;
                                index_ci /= C_domain_size;
                                c_content.val = C_domain[digit];
                            }
                        }
                        let mut independent_power_A = power_A.clone();
                        'inner: for row in row_A.iter() {
                            if !all_different(&independent_C, row) || !success {
                                success = false;
                                break 'inner;
                            }
                        }
                        'inner: for col in col_A.iter() {
                            if !all_different(&independent_C, col) || !success {
                                success = false;
                                break 'inner;
                            }
                        }
                        if success {
                            for area in independent_power_A.iter_mut() {
                                let mut value = 1;
                                let contains = extract_contains_structures(&independent_C, &area);
                                for element in contains.iter() {
                                    if let Structure::Element(ref element_content) = element {
                                        value *= element_content.val.unwrap();
                                    }
                                }
                                if let Structure::Composition(ref mut area_content) = area {
                                    area_content.val = Some(value);
                                }
                            }

                            let _lock = print_lock_clone.lock().unwrap();

                            println!("{:?}", independent_power_A);
                            println!("{:?}", independent_C);
                            println!("");
                        }
                    })
                })
            })
        })
    });
    pb.finish();
}
