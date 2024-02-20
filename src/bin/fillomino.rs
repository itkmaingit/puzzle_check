// label: cut-off, random, sparce expected
// name: fiilomino

// Cのdomainは現実的に{1..sqrt(n*m)}のために制限

use indicatif::{ProgressBar, ProgressStyle};
use puzzle_check::common::function::{
    compare_structures, cycle, extract_random_structure, power_set, progress_size,
    random_subset_with_validation,
};
use puzzle_check::common::initialize::initialize;
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

const n: i32 = 4;
const m: i32 = 5;
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
    let R: Vec<Relationship> = vec![H, V];
    let not_R: Vec<Relationship> = vec![M];
    let cutoff_functions: Vec<ValidationFn> = vec![non_cutoff];
    let A = combine(R, not_R, &C, &cutoff_functions);

    // combineの確認---------------------------
    // println!("{:?}", A.len());
    // for a in A.iter() {
    //     println!("{:?}", a);
    // }
    // ---------------------------------------

    let board_validation_functions: Vec<BoardValidationFn> = vec![non_validation];

    let max_c = ((board_size.0 * board_size.1) as f64).sqrt() as i32;

    let P_domain: Vec<Option<i32>> = vec![None];
    let C_domain: Vec<Option<i32>> = (0..=max_c).map(Some).collect();
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

    (0..LOOP_NUMBERS).into_par_iter().for_each(|_| {
        let mut B = Structure::Composition(Composition::new(vec![]));
        let mut power_A: Vec<Structure> = vec![];
        'inner: loop {
            if let Structure::Composition(ref B_content) = B {
                if B_content.entity.len() == C.len() {
                    break 'inner;
                }
            }
            let new_area = extract_random_structure(&A);
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

                    let mut independent_C = C.clone();
                    for area in power_A.iter() {
                        for structure_c in independent_C.iter_mut() {
                            {
                                if let Structure::Composition(ref a_content) = area {
                                    if a_content
                                        .entity
                                        .iter()
                                        .any(|cell| compare_structures(cell, structure_c))
                                    {
                                        if let Structure::Element(ref mut c_content) = structure_c {
                                            c_content.val = Some(a_content.entity.len() as i32);
                                        }
                                    }
                                }
                            }
                        }
                    }
                    pb.inc(1);
                    println!("{:?}", independent_C);
                    println!("{:?}", power_A);
                    println!("");
                })
            })
        })
    });
    pb.finish();
}
