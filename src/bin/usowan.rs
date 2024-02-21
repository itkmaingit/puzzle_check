// label: cut-off, sparce expected, random
// name: usowan

use indicatif::{ProgressBar, ProgressStyle};
use puzzle_check::common::combine::combine;
use puzzle_check::common::initialize::initialize;
use puzzle_check::common::operate_structures::OperateStructure;
use puzzle_check::specific::structure_functions::StructureFn;

use puzzle_check::common::dataclass::{
    Attribute, BoardSize, Composition, Coordinate, Element, Structure,
};
use puzzle_check::common::relationship::{relationship, Relationship, D, H, M, V};
use puzzle_check::specific::board_validation::{BoardValidation, BoardValidationFn};
use puzzle_check::specific::cutoff::{Cutoff, CutoffFn};

use rayon::prelude::*;
use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use std::thread;

// random_subsetが終了しないためサイズ制限を導入
fn size_limitation(area: &Structure) -> bool {
    if let Structure::Composition(ref area_content) = area {
        return area_content.entity.len() as i32 == 2;
    } else {
        unreachable!()
    }
}

fn non_division(area: &Structure) -> bool {
    let mut side_cell_counts = 0;
    if let Structure::Composition(ref area_content) = area {
        for cell in area_content.entity.iter() {
            if StructureFn::is_side(cell, &board_size) {
                side_cell_counts += 1;
            }
        }
    }
    if side_cell_counts >= 2 {
        return false;
    }

    return true;
}

const n: i32 = 5;
const m: i32 = 5;
const black: i32 = -1;
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
    let R: Vec<Relationship> = vec![D];
    let not_R: Vec<Relationship> = vec![M];
    let cutoff_functions_room: Vec<CutoffFn> = vec![size_limitation, non_division];
    let A = combine(R, not_R, &C, &cutoff_functions_room);

    // combineの確認---------------------------
    // println!("{:?}", A.len());
    // for a in A.iter() {
    //     println!("{:?}", a);
    // }

    // ---------------------------------------

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

    (0..LOOP_NUMBERS).into_par_iter().for_each(|_| {
        let mut B = Structure::Composition(Composition::new(vec![]));
        let mut power_A: Vec<Structure> = vec![];

        let board_validation_fn: Vec<BoardValidationFn> = vec![
            BoardValidation::non_matching_structures,
            BoardValidation::non_horizontal_structures,
            BoardValidation::non_vertical_structures,
            BoardValidation::non_diagonal_structures,
        ];

        let power_A = OperateStructure::random_subset_with_validation(&A, &board_validation_fn);

        let mut independent_C = C.clone();
        let readonly_C = C.clone();

        'outer: for cell in independent_C.iter_mut() {
            let mut value = 0;
            for area in power_A.iter() {
                if let Structure::Composition(ref area_content) = area {
                    for black_cell in area_content.entity.iter() {
                        if OperateStructure::compare_structures(cell, black_cell) {
                            continue 'outer;
                        }
                    }
                }
            }
            for adjacent in StructureFn::adjacent(cell, &readonly_C) {
                for area in power_A.iter() {
                    if let Structure::Composition(ref area_content) = area {
                        if area_content.entity.iter().any(|black_cell| {
                            OperateStructure::compare_structures(&adjacent, black_cell)
                        }) {
                            value += 1;
                        }
                    }
                }
            }
            // add to probability
            if let Structure::Element(ref mut cell_content) = cell {
                cell_content.val = Some(value);
            }
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

                    println!("black: {:?} {:?}", power_A.len(), power_A);
                    println!("{:?}", independent_C);
                    println!("");
                })
            })
        });

        pb.inc(1);
    });
    pb.finish();
}
