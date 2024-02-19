// label: cut-off, sparce expected
// name: chocobanana

use indicatif::{ProgressBar, ProgressStyle};
use puzzle_check::common::function::{
    adjacent, compare_structures, cycle, extract_random_structure, power_set, progress_size,
    random_subset_with_validation,
};
use puzzle_check::common::initialize::initialize;
use puzzle_check::common::predicates::{is_not_rectangle, is_rectangle};
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
    let LOOP_NUMBERS = 100000;
    let pb = ProgressBar::new(LOOP_NUMBERS);

    let (P, C, Ep, Ec) = initialize(&board_size);

    // ----------------------------------------------------------------------
    let white_R: Vec<Relationship> = vec![H, V];
    let white_not_R: Vec<Relationship> = vec![M];
    let black_R: Vec<Relationship> = vec![H, V];
    let black_not_R: Vec<Relationship> = vec![M];
    let cutoff_functions_white: Vec<ValidationFn> = vec![is_not_rectangle];
    let cutoff_functions_black: Vec<ValidationFn> = vec![is_rectangle];
    let white_A = combine(white_R, white_not_R, &C, &cutoff_functions_white);
    let black_A = combine(black_R, black_not_R, &C, &cutoff_functions_black);

    // combineの確認---------------------------
    // println!("{:?}", white_A.len());
    // for a in white_A.iter() {
    //     println!("{:?}", a);
    // }

    // println!("{:?}", black_A.len());
    // for a in black_A.iter() {
    //     println!("{:?}", a);
    // }
    // ---------------------------------------

    let board_validation_functions: Vec<BoardValidationFn> = vec![non_validation];

    let P_domain: Vec<Option<i32>> = vec![None];
    let C_domain: Vec<Option<i32>> = vec![None];
    let Ep_domain: Vec<Option<i32>> = vec![None];
    let Ec_domain: Vec<Option<i32>> = vec![None];
    let white_A_domain: Vec<Option<i32>> = (1..=n * m).map(Some).collect();
    let black_A_domain: Vec<Option<i32>> = (1..=n * m).map(Some).collect();

    let P_domain_size = P_domain.len();
    let C_domain_size = C_domain.len();
    let Ep_domain_size = Ep_domain.len();
    let Ec_domain_size = Ec_domain.len();
    let white_A_domain_size = white_A_domain.len();
    let blackA_domain_size = black_A_domain.len();

    let total_combinations_P = P_domain_size.pow(P.len() as u32);
    let total_combinations_C = C_domain_size.pow(C.len() as u32);
    let total_combinations_Ep = Ep_domain_size.pow(Ep.len() as u32);
    let total_combinations_Ec = P_domain_size.pow(Ec.len() as u32);

    (0..LOOP_NUMBERS).into_par_iter().for_each(|_| {
        let mut all_B = Structure::Composition(Composition::new(vec![]));
        let mut white_B = Structure::Composition(Composition::new(vec![]));
        let mut black_B = Structure::Composition(Composition::new(vec![]));
        let mut power_white_A: Vec<Structure> = vec![];
        let mut power_black_A: Vec<Structure> = vec![];
        let mut next = false;
        'inner: for i in 0..100000 {
            if let Structure::Composition(ref all_B_content) = all_B {
                if all_B_content.entity.len() == C.len()
                    && power_black_A.len() >= 2
                    && power_white_A.len() >= 2
                {
                    next = true;
                    break 'inner;
                }
            }
            let mut new_area: Structure;
            if i % 2 == 0 {
                new_area = extract_random_structure(&white_A);
                if relationship(&new_area, &white_B, M)
                    || relationship(&new_area, &white_B, H)
                    || relationship(&new_area, &white_B, V)
                    || relationship(&new_area, &black_B, M)
                {
                    continue 'inner;
                }
                white_B = add_up_structures(&white_B, &new_area);
                all_B = add_up_structures(&all_B, &new_area);
                power_white_A.push(new_area);
            } else {
                new_area = extract_random_structure(&black_A);
                if relationship(&new_area, &black_B, M)
                    || relationship(&new_area, &black_B, H)
                    || relationship(&new_area, &black_B, V)
                    || relationship(&new_area, &white_B, M)
                {
                    continue 'inner;
                }
                all_B = add_up_structures(&all_B, &new_area);
                black_B = add_up_structures(&black_B, &new_area);
                power_black_A.push(new_area);
            }
        }
        for white_area in power_white_A.iter_mut() {
            if let Structure::Composition(ref mut white_area_content) = white_area {
                white_area_content.val = Some(white_area_content.entity.len() as i32);
            }
        }
        for black_area in power_black_A.iter_mut() {
            if let Structure::Composition(ref mut black_area_content) = black_area {
                black_area_content.val = Some(black_area_content.entity.len() as i32);
            }
        }
        if next {
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

                            println!("white: {:?} {:?}", power_white_A.len(), power_white_A);
                            println!("black: {:?} {:?}", power_black_A.len(), power_black_A);
                            println!("");
                        })
                    })
                })
            })
        }
        pb.inc(1);
    });
}
