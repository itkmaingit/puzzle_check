// label: cut-off
// name: slitherlink

use indicatif::{ProgressBar, ProgressStyle};
use puzzle_check::common::dataclass::{
    Attribute, BoardSize, Composition, Coordinate, Element, Structure,
};
use puzzle_check::common::function::{
    compare_structures, cycle, extract_random_structure, power_set, progress_size,
    random_subset_with_validation,
};
use puzzle_check::common::initialize::initialize;
use puzzle_check::common::relationship::{relationship, Relationship, D, H, M, V};
use puzzle_check::specific::board::non_validation;
use puzzle_check::specific::graph::only_cycle;
use puzzle_check::{
    common::combine::{combine, ValidationFn},
    specific::board::BoardValidationFn,
};
use rayon::prelude::*;
use std::collections::HashSet;

const n: i32 = 3;
const m: i32 = 3;

fn main() {
    let board_size: BoardSize = BoardSize(3, 3);

    let (P, C, Ep, Ec) = initialize(&board_size);
    // ----------------------------------------------------------------------
    let R: Vec<Relationship> = vec![H, D, V];
    let not_R: Vec<Relationship> = vec![M];
    let cutoff_functions: Vec<ValidationFn> = vec![only_cycle];
    let G = combine(R, not_R, &Ep, &cutoff_functions);

    // combineの確認---------------------------
    // println!("{:?}", G.len());
    // for g in G.iter() {
    //     println!("{:?}", g);
    // }
    // ---------------------------------------

    let board_validation_functions: Vec<BoardValidationFn> = vec![non_validation];

    let P_domain: Vec<Option<i32>> = vec![None];
    let C_domain: Vec<Option<i32>> = (0..=4).map(Some).collect();
    let Ep_domain: Vec<Option<i32>> = (0..=1).map(Some).collect();
    let Ec_domain: Vec<Option<i32>> = vec![None];
    let G_domain: Vec<Option<i32>> = vec![None];

    let P_domain_size = P_domain.len();
    let C_domain_size = C_domain.len();
    let Ep_domain_size = Ep_domain.len();
    let Ec_domain_size = Ec_domain.len();
    let G_domain_size = G_domain.len();

    let total_combinations_P = P_domain_size.pow(P.len() as u32);
    let total_combinations_C = C_domain_size.pow(C.len() as u32);
    let total_combinations_Ep = Ep_domain_size.pow(Ep.len() as u32);
    let total_combinations_Ec = P_domain_size.pow(Ec.len() as u32);

    (0..total_combinations_P).into_par_iter().for_each(|pi| {
        let mut compute_P = P.clone();
        let mut index_pi = pi;

        for structure_p in compute_P.iter_mut() {
            if let Structure::Element(ref mut point_content) = structure_p {
                let digit = index_pi % P_domain_size;
                index_pi /= P_domain_size;
                point_content.val = P_domain[digit];
            }
        }
        (0..total_combinations_C).into_par_iter().for_each(|ci| {
            let mut compute_C = C.clone();
            let mut index_ci = ci;

            for structure_c in compute_C.iter_mut() {
                if let Structure::Element(ref mut cell_content) = structure_c {
                    let digit = index_ci % C_domain_size;
                    index_ci /= C_domain_size;
                    cell_content.val = C_domain[digit];
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

                'board_reset: for graph in G.iter() {
                    let mut compute_Ep = Ep.clone();
                    for structure_ep in compute_Ep.iter_mut() {
                        {
                            if let Structure::Composition(ref g_content) = graph {
                                if g_content
                                    .entity
                                    .iter()
                                    .any(|edge| compare_structures(edge, structure_ep))
                                {
                                    if let Structure::Element(ref mut ep_content) = structure_ep {
                                        ep_content.val = Some(1);
                                    }
                                } else {
                                    if let Structure::Element(ref mut ep_content) = structure_ep {
                                        ep_content.val = Some(0);
                                    }
                                }
                            }
                        }
                    }

                    for cell in compute_C.iter() {
                        if let Structure::Element(ref cell_content) = cell {
                            if cell_content.val.unwrap() != cycle(cell, &compute_Ep, &board_size) {
                                continue 'board_reset;
                            }
                        }
                    }
                    println!("{:?}", compute_C);
                    println!("{:?}", compute_Ep);
                    println!("");
                }
            })
        })
    })
}
