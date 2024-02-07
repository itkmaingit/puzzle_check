use crate::common::dataclass::{Composition, Element, Structure};
use crate::common::relationship::{relationship, Relationship, D, H, M, V};
use std::collections::HashSet;

fn power_set<T: Clone>(set: &[T]) -> Vec<Vec<T>> {
    let mut result = vec![Vec::new()];
    for item in set {
        for subset in result.clone() {
            let mut new_subset = subset.clone();
            new_subset.push(item.clone());
            result.push(new_subset);
        }
    }
    result
}

pub fn combine(R: HashSet<Relationship>, E: Vec<Structure>) -> Vec<Structure> {
    let whole_R: HashSet<Relationship> = vec![H, D, M, V].into_iter().collect();
    let not_R: HashSet<Relationship> = whole_R.difference(&R).cloned().collect();
    let power_E = power_set(&E);
    let mut result: Vec<Structure> = Vec::new();
    'outer: for e in power_E {
        if e.len() == 0 {
            continue;
        }

        if e.len() == 1 {
            let e_composed = Composition {
                val: None,
                entity: e.clone(),
            };
            let match_e: Structure = Structure::Composition(e_composed);
            result.push(match_e);
            continue;
        }
        // if e.len() >= 4 {
        //     println!("{:?}", e);
        //     println!("------");
        // }
        'mid_outer: for x in &e {
            let mut related = false;
            for y in &e {
                let mut not_related = true;
                if x != y {
                    'inner: for &r in &R {
                        if relationship(x, y, r) {
                            related = true;
                            break 'inner;
                        }
                    }
                    'inner: for &r in &not_R {
                        if relationship(x, y, r) {
                            not_related = false;
                            break 'inner;
                        }
                    }
                    if !not_related {
                        continue 'outer;
                    }
                }
            }
            if !related {
                continue 'outer;
            }
        }
        let e_composed = Composition {
            val: None,
            entity: e,
        };
        let match_e: Structure = Structure::Composition(e_composed);
        result.push(match_e);
    }

    return result;
}
