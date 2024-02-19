// label: cut-off, random, sparce expected
// name: shikaku

//長方形のサイズは現実的にn*m/2程度なのでcut-off

use indicatif::{ProgressBar, ProgressStyle};
use puzzle_check::common::function::{
    compare_structures, cycle, extract_random_structure, power_set, progress_size,
    random_subset_with_validation,
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

const n: i32 = 4;
const m: i32 = 4;

fn main() {
    let board_size: BoardSize = BoardSize(n, m);
    let LOOP_NUMBERS = 1000;
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
    let cutoff_functions: Vec<ValidationFn> = vec![is_rectangle];
    let A = combine(R, not_R, &C, &cutoff_functions);

    // combineの確認---------------------------
    // println!("{:?}", A.len());
    // for a in A.iter() {
    //     println!("{:?}", a);
    // }
    // ---------------------------------------

    let board_validation_functions: Vec<BoardValidationFn> = vec![non_validation];

    let max_a = board_size.0 * board_size.1 / 2 as i32;

    let P_domain: Vec<Option<i32>> = vec![None];
    let C_domain: Vec<Option<i32>> = vec![None];
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

    (0..1000).into_par_iter().for_each(|_| {
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
            let mut compute_P = P.clone();
            let mut index_pi = pi;

            for structure_p in compute_P.iter_mut() {
                if let Structure::Element(ref mut point) = structure_p {
                    let digit = index_pi % P_domain_size;
                    index_pi /= P_domain_size;
                    point.val = P_domain[digit];
                }
            }
            (0..total_combinations_Ep).into_par_iter().for_each(|epi| {
                let mut compute_Ep = Ep.clone();
                let mut index_epi = epi;

                for structure_ep in compute_Ep.iter_mut() {
                    if let Structure::Element(ref mut ep_content) = structure_ep {
                        let digit = index_epi % Ep_domain_size;
                        index_epi /= Ep_domain_size;
                        ep_content.val = Ep_domain[digit];
                    }
                }

                (0..total_combinations_Ec).into_par_iter().for_each(|eci| {
                    let mut compute_Ec = Ec.clone();
                    let mut index_eci = eci;

                    for structure_ec in compute_Ec.iter_mut() {
                        if let Structure::Element(ref mut ec_content) = structure_ec {
                            let digit = index_eci % Ec_domain_size;
                            index_eci /= Ec_domain_size;
                            ec_content.val = Ec_domain[digit];
                        }
                    }

                    (0..total_combinations_C).into_par_iter().for_each(|ci| {
                        let mut compute_C = C.clone();
                        let mut index_ci = ci;

                        for structure_c in compute_C.iter_mut() {
                            if let Structure::Element(ref mut c_content) = structure_c {
                                let digit = index_ci % C_domain_size;
                                index_ci /= C_domain_size;
                                c_content.val = C_domain[digit];
                            }
                        }
                        let mut compute_power_A = power_A.clone();
                        for area in compute_power_A.iter_mut() {
                            if let Structure::Composition(ref mut area_content) = area {
                                let size = area_content.entity.len();
                                area_content.val = Some(size as i32);
                            }
                        }
                        pb.inc(1);
                        println!("{:?}", compute_power_A);
                        println!("");
                    })
                })
            })
        })
    });
    pb.finish();
}
