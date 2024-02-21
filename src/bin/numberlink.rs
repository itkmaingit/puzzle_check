// label: cut-off, random
// name: chocobanana

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
const LOOP_NUMBERS: u64 = 100000;

fn main() {
    let pb = ProgressBar::new(LOOP_NUMBERS);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("main    {bar:40.cyan/blue} {pos}/{len} {percent}% {eta}")
            .unwrap(),
    );

    let (P, C, Ep, Ec) = initialize(&board_size);

    // ----------------------------------------------------------------------
    let R: Vec<Relationship> = vec![H, V, D];
    let not_R: Vec<Relationship> = vec![M];
    let cutoff_functions: Vec<CutoffFn> = vec![Cutoff::only_line];
    let L = combine(R, not_R, &Ec, &cutoff_functions);

    // combineの確認---------------------------
    // println!("{:?}", L.len());
    // for a in L.iter() {
    //     println!("{:?}", a);
    // }

    // ---------------------------------------

    let P_domain: Vec<Option<i32>> = vec![None];
    let C_domain: Vec<Option<i32>> = vec![None];
    let Ep_domain: Vec<Option<i32>> = vec![None];
    let Ec_domain: Vec<Option<i32>> = vec![Some(0), Some(1)];
    let L_domain: Vec<Option<i32>> = (1..=n * m).map(Some).collect();

    let P_domain_size = P_domain.len();
    let C_domain_size = C_domain.len();
    let Ep_domain_size = Ep_domain.len();
    let Ec_domain_size = Ec_domain.len();
    let L_domain_size = L_domain.len();

    let total_combinations_P = P_domain_size.pow(P.len() as u32);
    let total_combinations_C = C_domain_size.pow(C.len() as u32);
    let total_combinations_Ep = Ep_domain_size.pow(Ep.len() as u32);
    let total_combinations_Ec = P_domain_size.pow(Ec.len() as u32);

    (0..LOOP_NUMBERS).into_par_iter().for_each(|_| {
        let mut B = Structure::Composition(Composition::new(vec![]));
        let mut power_L: Vec<Structure> = vec![];
        let mut next = false;
        'inner: for i in 0..100000 {
            if let Structure::Composition(ref B_content) = B {
                if B_content.entity.len() + power_L.len() == C.len() {
                    next = true;
                    break 'inner;
                }
            }
            let mut new_line: Structure;

            new_line = OperateStructure::extract_random_structure(&L);
            if relationship(&new_line, &B, M)
                || relationship(&new_line, &B, H)
                || relationship(&new_line, &B, V)
                || relationship(&new_line, &B, D)
            {
                continue 'inner;
            }
            B = OperateStructure::add_up_structures(&B, &new_line);
            power_L.push(new_line);
        }
        let mut independent_Ec = Ec.clone();
        for (i, line) in power_L.iter_mut().enumerate() {
            if let Structure::Composition(ref mut line_content) = line {
                line_content.val = Some(i as i32);
                for edge in line_content.entity.iter() {
                    for compare_edge in independent_Ec.iter_mut() {
                        if OperateStructure::compare_structures(edge, &compare_edge) {
                            if let Structure::Element(ref mut compare_edge_content) = compare_edge {
                                compare_edge_content.val = Some(1);
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

                        println!("line: {:?} {:?}", power_L.len(), power_L);
                        println!("");
                    })
                })
            })
        }
        pb.inc(1);
    });
    pb.finish();
}
