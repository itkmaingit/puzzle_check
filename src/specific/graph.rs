use crate::common::dataclass::{Attribute, Coordinate, Element, Structure};
use crate::common::relationship::{relationship, Relationship, D, H, M, V};
use std::collections::HashMap;

pub fn only_cycle(G: &Structure) -> bool {
    let mut counts: HashMap<Coordinate, i32> = HashMap::new();
    if let Structure::Composition(ref g) = G {
        if g.entity.len() % 2 == 1 {
            return false;
        }
        for edge in &g.entity {
            if let Structure::Element(ref e) = edge {
                match e.attr {
                    Attribute::Hp | Attribute::Hc => {
                        let points = e.coor.horizon_points();
                        *counts.entry(points.0.clone()).or_insert(0) += 1;
                        *counts.entry(points.1.clone()).or_insert(0) += 1;
                    }
                    Attribute::Vp | Attribute::Vc => {
                        let points = e.coor.vertical_points();
                        *counts.entry(points.0.clone()).or_insert(0) += 1;
                        *counts.entry(points.1.clone()).or_insert(0) += 1;
                    }
                    _ => panic!(), // _と=>の間にスペースを追加
                }
            }
        }
    }
    let result = counts.values().all(|&count| count == 2);

    return result;
}

pub fn only_line(G: &Structure) -> bool {
    let mut counts: HashMap<Coordinate, i32> = HashMap::new();
    if let Structure::Composition(ref g) = G {
        for edge in &g.entity {
            if let Structure::Element(ref e) = edge {
                match e.attr {
                    Attribute::Hp | Attribute::Hc => {
                        let points = e.coor.horizon_points();
                        *counts.entry(points.0.clone()).or_insert(0) += 1;
                        *counts.entry(points.1.clone()).or_insert(0) += 1;
                    }
                    Attribute::Vp | Attribute::Vc => {
                        let points = e.coor.vertical_points();
                        *counts.entry(points.0.clone()).or_insert(0) += 1;
                        *counts.entry(points.1.clone()).or_insert(0) += 1;
                    }
                    _ => panic!(), // _と=>の間にスペースを追加
                }
            }
        }
    }
    // 個数が1か2であることを確認
    let valid_counts = counts.values().all(|&count| count == 1 || count == 2);

    // 個数が1である要素の数をカウント
    let single_counts = counts.values().filter(|&&count| count == 1).count();

    // 個数が1である要素がちょうど2つだけ存在するかどうかをチェック
    return valid_counts && single_counts == 2;
}
