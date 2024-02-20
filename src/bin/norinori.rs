// label: cut-off, sparce expected
// name: norinori

use indicatif::{ProgressBar, ProgressStyle};
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
use puzzle_check::{
    common::function::{
        adjacent, compare_structures, cycle, extract_random_structure, power_set, progress_size,
        random_subset_with_validation,
    },
    specific::board::{
        non_horizontal_structures, non_matching_structures, non_vertical_structures,
    },
};
use puzzle_check::{common::initialize::initialize, specific::board};
use rayon::prelude::*;
use std::collections::HashSet;

fn size_limitation(area: &Structure) -> bool {
    if let Structure::Composition(ref area_content) = area {
        return area_content.entity.len() as i32 == 2;
    } else {
        unreachable!()
    }
}

const n: i32 = 4;
const m: i32 = 4;
const board_size: BoardSize = BoardSize(n, m);
const LOOP_NUMBERS: u64 = 1000;
const black: i32 = 1;

fn main() {
    let pb = ProgressBar::new(LOOP_NUMBERS);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("main    {bar:40.cyan/blue} {pos}/{len} {percent}% {eta}")
            .unwrap(),
    );

    let (P, C, Ep, Ec) = initialize(&board_size);

    // ----------------------------------------------------------------------
    let room_R: Vec<Relationship> = vec![H, V];
    let room_not_R: Vec<Relationship> = vec![M];
    let black_R: Vec<Relationship> = vec![H, V];
    let black_not_R: Vec<Relationship> = vec![M];
    let cutoff_functions_room: Vec<ValidationFn> = vec![non_cutoff];
    let cutoff_functions_black: Vec<ValidationFn> = vec![size_limitation];
    let room_A = combine(room_R, room_not_R, &C, &cutoff_functions_room);
    let black_A = combine(black_R, black_not_R, &C, &cutoff_functions_black);

    // combineの確認---------------------------
    // println!("{:?}", room_A.len());
    // for a in room_A.iter() {
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
    let room_A_domain: Vec<Option<i32>> = vec![None];
    let black_A_domain: Vec<Option<i32>> = vec![None];

    let P_domain_size = P_domain.len();
    let C_domain_size = C_domain.len();
    let Ep_domain_size = Ep_domain.len();
    let Ec_domain_size = Ec_domain.len();
    let room_A_domain_size = room_A_domain.len();
    let blackA_domain_size = black_A_domain.len();

    let total_combinations_P = P_domain_size.pow(P.len() as u32);
    let total_combinations_C = C_domain_size.pow(C.len() as u32);
    let total_combinations_Ep = Ep_domain_size.pow(Ep.len() as u32);
    let total_combinations_Ec = P_domain_size.pow(Ec.len() as u32);

    (0..LOOP_NUMBERS).into_par_iter().for_each(|_| {
        let mut room_B = Structure::Composition(Composition::new(vec![]));
        let mut power_room_A: Vec<Structure> = vec![];
        let mut next = false;
        'inner: for i in 0..1000 {
            if let Structure::Composition(ref room_B_content) = room_B {
                if room_B_content.entity.len() == C.len() {
                    next = true;
                    break 'inner;
                }
            }

            let new_area = extract_random_structure(&room_A);

            if relationship(&new_area, &room_B, M) {
                continue 'inner;
            }
            room_B = add_up_structures(&room_B, &new_area);
            power_room_A.push(new_area);
        }
        let board_validation_fn: Vec<BoardValidationFn> = vec![
            non_matching_structures,
            non_horizontal_structures,
            non_vertical_structures,
        ];
        let mut power_black_A = random_subset_with_validation(&black_A, &board_validation_fn);

        let mut independent_C = C.clone();

        for black_area in power_black_A.iter_mut() {
            if let Structure::Composition(ref mut black_area_content) = black_area {
                for cell in black_area_content.entity.iter() {
                    for compare_cell in independent_C.iter_mut() {
                        if compare_structures(cell, &compare_cell) {
                            if let Structure::Element(ref mut compare_cell_content) = compare_cell {
                                compare_cell_content.val = Some(black);
                            }
                        }
                    }
                }
            }
        }
        if next {
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

                        let mut success = true;
                        'inner: for room_A in power_room_A.iter() {
                            let mut black_numbers = 0;
                            if let Structure::Composition(ref room_A_content) = room_A {
                                for cell in room_A_content.entity.iter() {
                                    for compare_cell in independent_C.iter() {
                                        if compare_structures(cell, compare_cell) {
                                            if let Structure::Element(ref compare_cell_content) =
                                                compare_cell
                                            {
                                                if compare_cell_content.val == Some(black) {
                                                    black_numbers += 1;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            if black_numbers != 2 {
                                success = false;
                                break 'inner;
                            }
                        }

                        if success {
                            println!("room: {:?} {:?}", power_room_A.len(), power_room_A);
                            println!("black: {:?} {:?}", power_black_A.len(), power_black_A);
                            println!("");
                        }
                    })
                })
            })
        }
        pb.inc(1);
    });
    pb.finish();
}
