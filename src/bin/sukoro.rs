// label: cut-off, sparce expected
// name: sukoro

// sizeは基本的にn*m/3>=なのでそれに制限

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

const n: i32 = 4;
const m: i32 = 4;
const board_size: BoardSize = BoardSize(n, m);
const LOOP_NUMBERS: u64 = 1000;

fn size_limitation(area: &Structure) -> bool {
    if let Structure::Composition(ref area_content) = area {
        return area_content.entity.len() as i32 >= n * m / 3;
    }
    unreachable!();
}

fn main() {
    let pb = ProgressBar::new(LOOP_NUMBERS);

    let (P, C, Ep, Ec) = initialize(&board_size);

    // ----------------------------------------------------------------------
    let R: Vec<Relationship> = vec![H, V];
    let not_R: Vec<Relationship> = vec![M];
    let cutoff_functions: Vec<CutoffFn> = vec![size_limitation];
    let A = combine(R, not_R, &C, &cutoff_functions);

    // combineの確認---------------------------
    // println!("{:?}", A.len());
    // for a in A.iter() {
    //     println!("{:?}", a);
    // }
    // ---------------------------------------

    let board_validation_functions: Vec<BoardValidationFn> = vec![BoardValidation::non_validation];

    let max_a = board_size.0 * board_size.1 / 2 as i32;

    let P_domain: Vec<Option<i32>> = vec![None];
    let C_domain: Vec<Option<i32>> = vec![None, Some(1), Some(2), Some(3), Some(4)];
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

                for area in A.iter() {
                    let mut independent_C = C.clone();
                    let mut B: Vec<Structure> = Vec::new();
                    if let Structure::Composition(ref area_content) = area {
                        for cell in &area_content.entity {
                            for compare_cell in independent_C.iter() {
                                if OperateStructure::compare_structures(&cell, compare_cell) {
                                    if !B.iter().any(|x| {
                                        OperateStructure::compare_structures(x, compare_cell)
                                    }) {
                                        B.push(compare_cell.clone());
                                    }
                                }
                            }
                        }
                    }

                    for compare_cell in independent_C.iter_mut() {
                        let iindependent_C = C.clone();
                        if B.iter()
                            .any(|x| OperateStructure::compare_structures(&compare_cell, x))
                        {
                            let adjacents = StructureFn::adjacent(&compare_cell, &iindependent_C);

                            let mut val = 0;
                            for adjacent in adjacents {
                                if B.iter()
                                    .any(|x| OperateStructure::compare_structures(&adjacent, x))
                                {
                                    val += 1
                                }
                            }
                            if let Structure::Element(ref mut compare_cell_content) = compare_cell {
                                compare_cell_content.val = Some(val);
                            }
                        }
                    }

                    let mut success = true;
                    'outer: for cell in independent_C.iter() {
                        if let Structure::Element(ref cell_content) = cell {
                            if cell_content.val != None {
                                for adjacent in StructureFn::adjacent(&cell, &independent_C) {
                                    if let Structure::Element(ref adjacent_content) = adjacent {
                                        if adjacent_content.val != None {
                                            if cell_content.val.unwrap()
                                                == adjacent_content.val.unwrap()
                                            {
                                                success = false;
                                                break 'outer;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    pb.inc(1);
                    if success {
                        println!("{:?}", area);
                        println!("{:?}", independent_C);
                        println!("");
                    }
                }
            })
        })
    });
    pb.finish();
}
