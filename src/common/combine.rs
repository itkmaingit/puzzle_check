use crate::common::dataclass::{Attribute, Composition, Coordinate, Element, Structure};
use crate::common::function::{compare_structures, power_set};
use crate::common::relationship::{relationship, Relationship, D, H, M, V};
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::collections::{HashSet, VecDeque};

pub type ValidationFn = fn(&Structure) -> bool;

pub fn non_cutoff(_: &Structure) -> bool {
    return true;
}

pub fn combine(
    R: Vec<Relationship>,
    not_R: Vec<Relationship>,
    E: &Vec<Structure>,
    cutoff_fn: &Vec<ValidationFn>,
) -> Vec<Structure> {
    let pb_E = ProgressBar::new(2usize.pow(E.len() as u32) as u64);
    pb_E.set_style(
        ProgressStyle::default_bar()
            .template("subset  {bar:40.cyan/blue} {pos}/{len} {percent}% {eta}")
            .unwrap(),
    );

    let power_E = power_set(&E, &pb_E);
    pb_E.finish();
    let pb = ProgressBar::new(power_E.len() as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("combine {bar:40.cyan/blue} {pos}/{len} {percent}% {eta}")
            .unwrap(),
    );

    // let hp12 = Structure::Element(Element::new(Attribute::Hp, Coordinate(1, 2)));
    // let hp21 = Structure::Element(Element::new(Attribute::Hp, Coordinate(2, 1)));
    // let hp31 = Structure::Element(Element::new(Attribute::Hp, Coordinate(3, 1)));
    // let hp32 = Structure::Element(Element::new(Attribute::Hp, Coordinate(3, 2)));
    // let vp12 = Structure::Element(Element::new(Attribute::Vp, Coordinate(1, 2)));
    // let vp13 = Structure::Element(Element::new(Attribute::Vp, Coordinate(1, 3)));
    // let vp21 = Structure::Element(Element::new(Attribute::Vp, Coordinate(2, 1)));
    // let vp23 = Structure::Element(Element::new(Attribute::Vp, Coordinate(2, 3)));

    // let entity = vec![hp12, hp21, hp31, hp32, vp12, vp13, vp21, vp23];

    // let test_structure = Structure::Composition(Composition {
    //     val: None,
    //     entity: entity,
    // });

    // 並列化処理のためにrayonのpar_iterを使用
    let result: Vec<Structure> = power_E
        .par_iter()
        .filter_map(|e| {
            if e.is_empty() {
                pb.inc(1);
                return None;
            }

            if e.len() == 1 {
                pb.inc(1);
                let s = Structure::Composition(Composition {
                    val: None,
                    entity: e.clone(),
                });
                for validate in cutoff_fn {
                    if !validate(&s) {
                        return None;
                    }
                }
                return Some(s);
            }

            'outer: for x in e {
                let mut related = false;
                for y in e {
                    if x != y {
                        let mut not_related = true;
                        for &r in &not_R {
                            if relationship(x, y, r) {
                                pb.inc(1);
                                return None;
                            }
                        }

                        for &r in &R {
                            if relationship(x, y, r) {
                                related = true;
                                continue 'outer;
                            }
                        }
                    }
                }
                if !related {
                    pb.inc(1);
                    return None;
                }
            }

            pb.inc(1);
            let s = Structure::Composition(Composition {
                val: None,
                entity: e.clone(),
            });
            if !all_are_connected(&s, &R) {
                return None;
            }

            for validate in cutoff_fn {
                if !validate(&s) {
                    return None;
                }
            }

            Some(s)
        })
        .collect();

    pb.finish();
    return result;
}

fn all_are_connected(structure: &Structure, R: &Vec<Relationship>) -> bool {
    if let Structure::Composition(ref composition) = structure {
        let mut visited: HashSet<Structure> = HashSet::new();
        let mut queue: VecDeque<Structure> = VecDeque::new();
        let start_node = composition.entity[0].clone();
        queue.push_back(start_node);
        while let Some(current_node) = queue.pop_front() {
            'outer: for s in composition.entity.iter() {
                if visited.contains(&s) {
                    continue 'outer;
                }
                for &r in R {
                    if relationship(&s, &current_node, r) {
                        queue.push_back(s.clone());
                        visited.insert(s.clone());
                        continue 'outer;
                    }
                }
            }
        }
        return visited.len() == composition.entity.len();
    }
    unreachable!();
}
