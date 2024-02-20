// label: cut-off, sparce expected, random
// name: hitori

use indicatif::{ProgressBar, ProgressStyle};
use puzzle_check::common::relationship::{relationship, Relationship, D, H, M, V};
use puzzle_check::common::{
    dataclass::{Attribute, BoardSize, Composition, Coordinate, Element, Structure},
    function::{add_up_structures, subtract_structures},
};
use puzzle_check::common::{
    function::all_different,
    predicates::{is_not_rectangle, is_rectangle},
};
use puzzle_check::specific::board::non_validation;
use puzzle_check::specific::graph::only_cycle;
use puzzle_check::{
    common::combine::{combine, non_cutoff, ValidationFn},
    specific::board::BoardValidationFn,
};
use puzzle_check::{
    common::function::{
        adjacent, compare_structures, cycle, extract_random_structure, is_side, power_set,
        progress_size, random_subset_with_validation,
    },
    specific::board::{
        non_diagonal_structures, non_horizontal_structures, non_matching_structures,
        non_vertical_structures,
    },
};
use puzzle_check::{common::initialize::initialize, specific::board};
use rayon::prelude::*;
use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use std::thread;

// random_subsetが終了しないためサイズ制限を導入
fn size_limitation(area: &Structure) -> bool {
    if let Structure::Composition(ref area_content) = area {
        return area_content.entity.len() as i32 <= 2;
    } else {
        unreachable!()
    }
}
fn size_limitation_n(area: &Structure) -> bool {
    if let Structure::Composition(ref area_content) = area {
        return area_content.entity.len() as i32 == n;
    } else {
        unreachable!()
    }
}

pub fn non_division(area: &Structure) -> bool {
    let mut side_cell_counts = 0;
    if let Structure::Composition(ref area_content) = area {
        for cell in area_content.entity.iter() {
            if is_side(cell, &board_size) {
                side_cell_counts += 1;
            }
        }
    }
    if side_cell_counts >= 2 {
        return false;
    }

    return true;
}

const n: i32 = 3;
const black: i32 = -1;
const board_size: BoardSize = BoardSize(n, n);
const LOOP_NUMBERS: u64 = 1000;

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
    let R: Vec<Relationship> = vec![D];
    let not_R: Vec<Relationship> = vec![M];
    let row_R: Vec<Relationship> = vec![H];
    let not_row_R: Vec<Relationship> = vec![D, V, M];
    let col_R: Vec<Relationship> = vec![V];
    let not_col_R: Vec<Relationship> = vec![H, D, M];
    let cutoff_functions: Vec<ValidationFn> = vec![size_limitation, non_division];
    let cutoff_functions_for_different: Vec<ValidationFn> = vec![size_limitation_n];
    let A = combine(R, not_R, &C, &cutoff_functions);
    let row_A = combine(row_R, not_row_R, &C, &cutoff_functions_for_different);
    let col_A = combine(col_R, not_col_R, &C, &cutoff_functions_for_different);

    // combineの確認---------------------------
    // println!("{:?}", row_A.len());
    // for a in row_A.iter() {
    //     println!("{:?}", a);
    // }
    // println!("{:?}", col_A.len());
    // for a in col_A.iter() {
    //     println!("{:?}", a);
    // }

    // ---------------------------------------

    let P_domain: Vec<Option<i32>> = vec![None];
    let C_domain: Vec<Option<i32>> = (1..=n).map(Some).collect();
    let Ep_domain: Vec<Option<i32>> = vec![None];
    let Ec_domain: Vec<Option<i32>> = vec![None];
    let A_domain: Vec<Option<i32>> = vec![None];

    let P_domain_size = P_domain.len();
    let C_domain_size = C_domain.len();
    let Ep_domain_size = Ep_domain.len();
    let Ec_domain_size = Ec_domain.len();
    let A_domain_size = A_domain.len();

    let total_combinations_P = P_domain_size.pow(P.len() as u32);
    let total_combinations_Ep = Ep_domain_size.pow(Ep.len() as u32);
    let total_combinations_Ec = P_domain_size.pow(Ec.len() as u32);

    (0..LOOP_NUMBERS).into_par_iter().for_each(|_| {
        let print_lock_clone = Arc::clone(&print_lock);
        let mut B = Structure::Composition(Composition::new(vec![]));
        let mut power_A: Vec<Structure> = vec![];

        let board_validation_fn: Vec<BoardValidationFn> = vec![
            non_matching_structures,
            non_horizontal_structures,
            non_vertical_structures,
            non_diagonal_structures,
        ];

        let power_A = random_subset_with_validation(&A, &board_validation_fn);

        let readonly_C = C.clone();
        let mut pseudo_C = Structure::Composition(Composition::new(readonly_C));
        for area in power_A.iter() {
            pseudo_C = subtract_structures(&pseudo_C, area);
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
                    if let Structure::Composition(ref pseudo_C_content) = pseudo_C {
                        let total_combinations_C =
                            C_domain_size.pow(pseudo_C_content.entity.len() as u32);
                        (0..total_combinations_C).into_par_iter().for_each(|ci| {
                            let mut index_ci = ci;
                            if let Structure::Composition(ref pseudo_C_content) = pseudo_C {
                                let mut independent_C = pseudo_C_content.clone();
                                let mut success = true;

                                for structure_c in independent_C.entity.iter_mut() {
                                    if let Structure::Element(ref mut c_content) = structure_c {
                                        let digit = index_ci % C_domain_size;
                                        index_ci /= C_domain_size;
                                        c_content.val = C_domain[digit];
                                    }
                                }
                                for row in row_A.iter() {
                                    if !all_different(&independent_C.entity, row) {
                                        success = false;
                                    }
                                }
                                for col in col_A.iter() {
                                    if !all_different(&independent_C.entity, col) {
                                        success = false;
                                    }
                                }

                                if success {
                                    let _lock = print_lock_clone.lock().unwrap();
                                    println!("black: {:?} {:?}", power_A.len(), power_A);
                                    println!("{:?}", independent_C);
                                    println!("");
                                }
                            }
                        })
                    }
                })
            })
        });

        pb.inc(1);
    });
    pb.finish();
}
