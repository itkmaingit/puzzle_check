use crate::common::dataclass::{Attribute, Composition, Coordinate, Element, Structure};
use crate::common::function::{compare_structures, power_set};
use crate::common::relationship::{relationship, Relationship, D, H, M, V};
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::collections::HashSet;

pub type ValidationFn = fn(&Structure) -> bool;

pub fn combine(
    R: HashSet<Relationship>,
    E: &Vec<Structure>,
    cutoff_fn: &Vec<ValidationFn>,
) -> Vec<Structure> {
    let whole_R: HashSet<Relationship> = vec![H, D, M, V].into_iter().collect();
    let not_R: HashSet<Relationship> = whole_R.difference(&R).cloned().collect();
    let pb_E = ProgressBar::new(2usize.pow(E.len() as u32) as u64);
    let power_E = power_set(&E, &pb_E);
    let pb = ProgressBar::new(power_E.len() as u64);

    let hp12 = Structure::Element(Element::new(Attribute::Hp, Coordinate(1, 2)));
    let hp21 = Structure::Element(Element::new(Attribute::Hp, Coordinate(2, 1)));
    let hp31 = Structure::Element(Element::new(Attribute::Hp, Coordinate(3, 1)));
    let hp32 = Structure::Element(Element::new(Attribute::Hp, Coordinate(3, 2)));
    let vp12 = Structure::Element(Element::new(Attribute::Vp, Coordinate(1, 2)));
    let vp13 = Structure::Element(Element::new(Attribute::Vp, Coordinate(1, 3)));
    let vp21 = Structure::Element(Element::new(Attribute::Vp, Coordinate(2, 1)));
    let vp23 = Structure::Element(Element::new(Attribute::Vp, Coordinate(2, 3)));

    let entity = vec![hp12, hp21, hp31, hp32, vp12, vp13, vp21, vp23];

    let test_structure = Structure::Composition(Composition {
        val: None,
        entity: entity,
    });

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
            // if e.len() == 8 {
            //     let s = Structure::Composition(Composition {
            //         val: None,
            //         entity: e.clone(),
            //     });
            //     if compare_structures(&s, &test_structure) {
            //         println!("{:?}", e);
            //     }
            // }

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

            for validate in cutoff_fn {
                if !validate(&s) {
                    return None;
                }
            }

            Some(s)
        })
        .collect();

    result
}
