// label: cut-off
// name: numberplace(sudoku)

// disabled for combinatorial explosion

use indicatif::{ProgressBar, ProgressStyle};
use puzzle_check::common::relationship::{relationship, Relationship, D, H, M, V};
use puzzle_check::common::{
    dataclass::{Attribute, BoardSize, Composition, Coordinate, Element, Structure},
    function::add_up_structures,
};
use puzzle_check::common::{function::all_different, initialize::initialize};
use puzzle_check::common::{
    function::{
        compare_structures, cycle, extract_random_structure, power_set, progress_size,
        random_subset_with_validation,
    },
    predicates::is_square,
};
use puzzle_check::specific::board::non_validation;
use puzzle_check::specific::graph::only_cycle;
use puzzle_check::{
    common::combine::{combine, non_cutoff, ValidationFn},
    specific::board::BoardValidationFn,
};
use rayon::prelude::*;
use std::collections::HashSet;

fn size_limitation(area: &Structure) -> bool {
    if let Structure::Composition(ref area_content) = area {
        return area_content.entity.len() as i32 == 4;
    } else {
        unreachable!()
    }
}

const n: i32 = 4;
const m: i32 = 4;
const board_size: BoardSize = BoardSize(n, m);
const LOOP_NUMBERS: u64 = 1000;

fn main() {
    let pb = ProgressBar::new(LOOP_NUMBERS);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("main    {bar:40.cyan/blue} {pos}/{len} {percent}% {eta}")
            .unwrap(),
    );

    let (P, C, Ep, Ec) = initialize(&board_size);

    // ----------------------------------------------------------------------
    let R1: Vec<Relationship> = vec![H];
    let R2: Vec<Relationship> = vec![V];
    let R3: Vec<Relationship> = vec![H, V];
    let not_R1: Vec<Relationship> = vec![M];
    let not_R2: Vec<Relationship> = vec![M];
    let not_R3: Vec<Relationship> = vec![M];
    let cutoff_functions1: Vec<ValidationFn> = vec![size_limitation];
    let cutoff_functions2: Vec<ValidationFn> = vec![size_limitation];
    let cutoff_functions3: Vec<ValidationFn> = vec![size_limitation, is_square];
    let A1 = combine(R1, not_R1, &C, &cutoff_functions1);
    let A2 = combine(R2, not_R2, &C, &cutoff_functions2);
    let A3 = combine(R3, not_R3, &C, &cutoff_functions3);

    // combineの確認---------------------------
    // println!("{:?}", A.len());
    // for a in A.iter() {
    //     println!("{:?}", a);
    // }
    // ---------------------------------------

    let board_validation_functions: Vec<BoardValidationFn> = vec![non_validation];

    let P_domain: Vec<Option<i32>> = vec![None];
    let C_domain: Vec<Option<i32>> = (0..=4).map(Some).collect();
    let Ep_domain: Vec<Option<i32>> = vec![None];
    let Ec_domain: Vec<Option<i32>> = vec![None];
    let A_domain: Vec<Option<i32>> = vec![None];

    let P_domain_size = P_domain.len();
    let C_domain_size = C_domain.len();
    let Ep_domain_size = Ep_domain.len();
    let Ec_domain_size = Ec_domain.len();
    let A_domain_size = A_domain.len();

    let total_combinations_P = P_domain_size.pow(P.len() as u32);
    let total_combinations_C = C_domain_size.pow(C.len() as u32);
    let total_combinations_Ep = Ep_domain_size.pow(Ep.len() as u32);
    let total_combinations_Ec = P_domain_size.pow(Ec.len() as u32);

    let progress_size = total_combinations_P
        * total_combinations_C
        * total_combinations_Ep
        * total_combinations_Ec
        * LOOP_NUMBERS as usize;
    let pb = ProgressBar::new(progress_size as u64);

    (0..LOOP_NUMBERS).into_par_iter().for_each(|_| {
        let mut B = Structure::Composition(Composition::new(vec![]));
        let mut power_A: Vec<Structure> = vec![];
        let mut next = false;
        'inner: for i in 0..1000 {
            if let Structure::Composition(ref B_content) = B {
                if B_content.entity.len() == C.len() {
                    next = true;
                    break 'inner;
                }
            }
            let new_area = extract_random_structure(&A3);
            if relationship(&new_area, &B, M) {
                continue 'inner;
            }
            B = add_up_structures(&B, &new_area);
            power_A.push(new_area);
        }
        if next {
            (0..total_combinations_P).into_par_iter().for_each(|pi| {
                let mut independent_P = P.clone();
                let mut index_pi = pi;

                for structure_p in independent_P.iter_mut() {
                    if let Structure::Element(ref mut point_content) = structure_p {
                        let digit = index_pi % P_domain_size;
                        index_pi /= P_domain_size;
                        point_content.val = P_domain[digit];
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

                            for structure_c in independent_C.iter_mut() {
                                if let Structure::Element(ref mut c_content) = structure_c {
                                    let digit = index_ci % C_domain_size;
                                    index_ci /= C_domain_size;
                                    c_content.val = C_domain[digit];
                                }
                            }

                            let mut is_correct = true;
                            'inner: for area in power_A.iter() {
                                if !is_correct {
                                    break 'inner;
                                }
                                if !all_different(&independent_C, area) {
                                    is_correct = false;
                                }
                            }
                            'inner: for area in A1.iter() {
                                if !is_correct {
                                    break 'inner;
                                }
                                if !all_different(&independent_C, area) {
                                    is_correct = false;
                                }
                            }
                            'inner: for area in A2.iter() {
                                if !is_correct {
                                    break 'inner;
                                }
                                if !all_different(&independent_C, area) {
                                    is_correct = false;
                                }
                            }
                            if is_correct {
                                println!("{:?}", independent_C);
                                println!("");
                            }
                            pb.inc(1);
                        })
                    })
                })
            })
        }
    });
    pb.finish();
}
