use crate::common::dataclass::{Attribute, Coordinate, Element, Structure};
use crate::common::relationship::{relationship, Relationship, D, H, M, V};
use std::collections::HashMap;

// cutoff関数の型
pub type CutoffFn = fn(&Structure) -> bool;

pub struct Cutoff {}

impl Cutoff {
    // ---------------------------------------------------------------------------------------------------------------------
    // ↓　non_cutoff

    // ---------------------------------------------------------------------------------------------------------------------
    // [overview]
    // カットオフを行わない関数
    // ---------------------------------------------------------------------------------------------------------------------
    // [params]
    // _: &Structure - 使用しない
    // ---------------------------------------------------------------------------------------------------------------------
    // [return]
    // bool - always true
    // ---------------------------------------------------------------------------------------------------------------------
    pub fn non_cutoff(_: &Structure) -> bool {
        return true;
    }

    // ---------------------------------------------------------------------------------------------------------------------
    // ↓　for graph

    // ---------------------------------------------------------------------------------------------------------------------
    // [overview]
    // グラフがサイクル（単一閉曲線）であることを保証する関数
    // G = combine([H, V, D], Ep) or combine([H, V, D], Ec)
    // ---------------------------------------------------------------------------------------------------------------------
    // [params]
    // G: &Structure - グラフ
    // ---------------------------------------------------------------------------------------------------------------------
    // [return]
    // bool - サイクルであるときtrue
    // ---------------------------------------------------------------------------------------------------------------------
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

    // ---------------------------------------------------------------------------------------------------------------------
    // [overview]
    // グラフがパス（交差のない曲線）であることを保証する関数
    // G = combine([H, V, D], Ep) or combine([H, V, D], Ec)
    // ---------------------------------------------------------------------------------------------------------------------
    // [params]
    // G: &Structure - グラフ
    // ---------------------------------------------------------------------------------------------------------------------
    // [return]
    // bool - パスであるときtrue
    // ---------------------------------------------------------------------------------------------------------------------
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

    // ---------------------------------------------------------------------------------------------------------------------
    // ↓　for area

    // ---------------------------------------------------------------------------------------------------------------------
    // [overview]
    // 領域が長方形（内部充足）であるかを確認する関数
    // A = combine([H, V], C)
    // ---------------------------------------------------------------------------------------------------------------------
    // [params]
    // area: &Structure - 長方形かどうかを確かめたい領域
    // ---------------------------------------------------------------------------------------------------------------------
    // [return]
    // bool - 長方形であるときtrue
    // ---------------------------------------------------------------------------------------------------------------------
    pub fn is_rectangle(area: &Structure) -> bool {
        let mut minX = std::i32::MAX;
        let mut minY = std::i32::MAX;
        let mut maxX = -std::i32::MAX;
        let mut maxY = -std::i32::MAX;

        if let Structure::Composition(ref area_content) = area {
            for cell in area_content.entity.iter() {
                if let Structure::Element(ref cell_content) = cell {
                    let x = cell_content.coor.1;
                    let y = cell_content.coor.0;
                    minX = std::cmp::min(minX, x);
                    minY = std::cmp::min(minY, y);
                    maxX = std::cmp::max(maxX, x);
                    maxY = std::cmp::max(maxY, y);
                }
            }
            let width = maxX - minX + 1;
            let height = maxY - minY + 1;
            let size = width * height;

            return area_content.entity.len() as i32 == size;
        }

        unreachable!();
    }

    // ---------------------------------------------------------------------------------------------------------------------
    // [overview]
    // 領域が長方形（内部充足）でないことを確認する関数
    // A = combine([H, V], C)
    // ---------------------------------------------------------------------------------------------------------------------
    // [params]
    // area: &Structure - 長方形でないかを確かめたい領域
    // ---------------------------------------------------------------------------------------------------------------------
    // [return]
    // bool - 長方形でないときtrue
    // ---------------------------------------------------------------------------------------------------------------------
    pub fn is_not_rectangle(area: &Structure) -> bool {
        let mut minX = std::i32::MAX;
        let mut minY = std::i32::MAX;
        let mut maxX = -std::i32::MAX;
        let mut maxY = -std::i32::MAX;

        if let Structure::Composition(ref area_content) = area {
            for cell in area_content.entity.iter() {
                if let Structure::Element(ref cell_content) = cell {
                    let x = cell_content.coor.1;
                    let y = cell_content.coor.0;
                    minX = std::cmp::min(minX, x);
                    minY = std::cmp::min(minY, y);
                    maxX = std::cmp::max(maxX, x);
                    maxY = std::cmp::max(maxY, y);
                }
            }
            let width = maxX - minX + 1;
            let height = maxY - minY + 1;
            let size = width * height;

            return area_content.entity.len() as i32 != size;
        }

        unreachable!();
    }

    // ---------------------------------------------------------------------------------------------------------------------
    // [overview]
    // 領域が正方形（内部充足）であるかを確認する関数
    // A = combine([H, V], C)
    // ---------------------------------------------------------------------------------------------------------------------
    // [params]
    // area: &Structure - 正方形かどうかを確かめたい領域
    // ---------------------------------------------------------------------------------------------------------------------
    // [return]
    // bool - 正方形であるときtrue
    // ---------------------------------------------------------------------------------------------------------------------
    pub fn is_square(area: &Structure) -> bool {
        let mut minX = std::i32::MAX;
        let mut minY = std::i32::MAX;
        let mut maxX = -std::i32::MAX;
        let mut maxY = -std::i32::MAX;

        if let Structure::Composition(ref area_content) = area {
            for cell in area_content.entity.iter() {
                if let Structure::Element(ref cell_content) = cell {
                    let x = cell_content.coor.1;
                    let y = cell_content.coor.0;
                    minX = std::cmp::min(minX, x);
                    minY = std::cmp::min(minY, y);
                    maxX = std::cmp::max(maxX, x);
                    maxY = std::cmp::max(maxY, y);
                }
            }
            let width = maxX - minX + 1;
            let height = maxY - minY + 1;
            let size = width * height;

            return area_content.entity.len() as i32 == size && width == height;
        }

        unreachable!();
    }
}
