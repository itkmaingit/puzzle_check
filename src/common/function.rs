use crate::common::dataclass::{Attribute, BoardSize, Composition, Coordinate, Element, Structure};
use crate::specific::board::BoardValidationFn;
use indicatif::{ProgressBar, ProgressStyle};
use rand::Rng;
use rayon::prelude::*;
use std::collections::HashMap;

pub fn power_set<T: Clone + Send + Sync>(set: &[T], pb: &ProgressBar) -> Vec<Vec<T>> {
    if set.is_empty() {
        return vec![Vec::new()];
    }

    // 最初の要素を取り除いた残りのセット
    let tail = &set[1..];

    // 残りのセットに対する冪集合を再帰的に計算
    let tail_subsets = power_set(tail, pb);

    // 最初の要素
    let head = &set[0];

    // 残りのセットの冪集合の各サブセットに対して、
    // 最初の要素を含むバージョンと含まないバージョンの両方を生成
    tail_subsets
        .into_par_iter()
        .flat_map(|subset| {
            let mut with_head = subset.clone();
            with_head.push(head.clone());
            pb.inc(1);
            vec![subset, with_head]
        })
        .collect()
}

pub fn progress_size(domain_size: usize, set_size: usize) -> u64 {
    return (domain_size as u64).pow(set_size as u32) as u64;
}

pub fn compare_structures(s1: &Structure, s2: &Structure) -> bool {
    match (s1, s2) {
        (Structure::Composition(c1), Structure::Composition(c2)) => {
            // Composition内のentityの数が同じであることを確認
            if c1.entity.len() != c2.entity.len() {
                return false;
            }

            // c1のすべてのElementがc2にも存在するかチェック
            c1.entity.iter().all(|e1| {
                match e1 {
                    Structure::Element(el1) => {
                        // c2のentity内で、el1と同じCoordinateとAttributeを持つElementが存在するか
                        c2.entity.iter().any(|e2| match e2 {
                            Structure::Element(el2) => el1.coor == el2.coor && el1.attr == el2.attr,
                            _ => false,
                        })
                    }
                    _ => false,
                }
            }) &&
            // c2のすべてのElementがc1にも存在するかチェック（双方向の確認）
            c2.entity.iter().all(|e2| {
                match e2 {
                    Structure::Element(el2) => {
                        // c1のentity内で、el2と同じCoordinateとAttributeを持つElementが存在するか
                        c1.entity.iter().any(|e1| match e1 {
                            Structure::Element(el1) => el1.coor == el2.coor && el1.attr == el2.attr,
                            _ => false,
                        })
                    }
                    _ => false,
                }
            })
        }
        (Structure::Element(e1), Structure::Element(e2)) => {
            return e1.attr == e2.attr && e1.coor == e2.coor;
        }
        _ => false,
    }
}

//計算量の爆発に注意、基本的にCompositionのサイズに制限を入れずにboard_validation_fnを用意すると爆発する
pub fn random_subset_with_validation(
    set: &Vec<Structure>,
    board_validation_fn: &Vec<BoardValidationFn>,
) -> Vec<Structure> {
    let mut rng = rand::thread_rng(); // 乱数生成器のインスタンス
    'outer: loop {
        let mut subset = Vec::new();
        for item in set {
            // 各要素について、50%の確率で部分集合に含める
            if rng.gen_bool(0.5) {
                subset.push(item.clone());
            }
        }
        for function in board_validation_fn {
            if !function(&subset) {
                continue 'outer;
            }
        }
        return subset;
    }
}

pub fn extract_random_structure(items: &Vec<Structure>) -> Structure {
    let mut rng = rand::thread_rng(); // 乱数生成器のインスタンス
    let index = rng.gen_range(0..items.len()); // ランダムなインデックスを生成
    return items.get(index).unwrap().clone(); // ランダムに選ばれた要素を返す
}

pub fn cycle(cell: &Structure, Ep: &Vec<Structure>, board_size: &BoardSize) -> i32 {
    if let Structure::Element(ref cell_content) = cell {
        if cell_content.attr != Attribute::C {
            panic!("cycleでC以外の要素が渡されました！");
        }
        let x = cell_content.coor.1;
        let y = cell_content.coor.0;
        let n = board_size.0;
        let m = board_size.1;

        let h_top = (y - 1) * m + x - 1;
        let h_bottom = y * m + x - 1;
        let v_top = m * (n + 1) + (y - 1) * (m + 1) + x - 1;
        let v_bottom = m * (n + 1) + (y - 1) * (m + 1) + x;

        let mut result = 0;

        if let Structure::Element(ref ep_content) = Ep[h_top as usize] {
            result += ep_content.val.unwrap();
        }
        if let Structure::Element(ref ep_content) = Ep[h_bottom as usize] {
            result += ep_content.val.unwrap();
        }
        if let Structure::Element(ref ep_content) = Ep[v_top as usize] {
            result += ep_content.val.unwrap();
        }
        if let Structure::Element(ref ep_content) = Ep[v_bottom as usize] {
            result += ep_content.val.unwrap();
        }

        return result;
    }
    unreachable!("cycleでC以外の要素が渡されました！");
}

pub fn is_side(cell: &Structure, board_size: &BoardSize) -> bool {
    if let Structure::Element(ref cell_content) = cell {
        if cell_content.attr != Attribute::C {
            panic!("is_sideがcell以外に対して呼ばれました！");
        }
        let x = cell_content.coor.1;
        let y = cell_content.coor.0;

        let side_x = board_size.1;
        let side_y = board_size.0;

        return x == side_x || y == side_y;
    }
    panic!("is_sideがcell以外に対して呼ばれました！");
}

pub fn line_edgepoints(line: &Structure) -> Vec<Structure> {
    let mut counts: HashMap<Coordinate, i32> = HashMap::new();
    let mut is_point = true;
    if let Structure::Composition(ref l) = line {
        for edge in &l.entity {
            if let Structure::Element(ref e) = edge {
                match e.attr {
                    Attribute::Hp | Attribute::Hc => {
                        let points = e.coor.horizon_points();
                        *counts.entry(points.0.clone()).or_insert(0) += 1;
                        *counts.entry(points.1.clone()).or_insert(0) += 1;
                        if e.attr == Attribute::Hp {
                            is_point = true;
                        } else {
                            is_point = false;
                        }
                    }
                    Attribute::Vp | Attribute::Vc => {
                        let points = e.coor.vertical_points();
                        *counts.entry(points.0.clone()).or_insert(0) += 1;
                        *counts.entry(points.1.clone()).or_insert(0) += 1;
                        if e.attr == Attribute::Vp {
                            is_point = true;
                        } else {
                            is_point = false;
                        }
                    }
                    _ => unreachable!(), // _と=>の間にスペースを追加
                }
            }
        }
    }

    let single_coors: Vec<Coordinate> = counts
        .iter()
        .filter(|(_, &count)| count == 1)
        .map(|(coor, _)| coor.clone())
        .collect();

    let mut result: Vec<Structure> = Vec::new();

    if is_point {
        for coor in single_coors.iter() {
            let edgepoint = Structure::Element(Element::new(Attribute::P, coor.clone()));
            result.push(edgepoint);
        }
    } else {
        for coor in single_coors.iter() {
            let edgepoint = Structure::Element(Element::new(Attribute::C, coor.clone()));
            result.push(edgepoint);
        }
    }

    return result;
}

// for cut-off function
// 同じ構造体同士を足し合わせることを前提とした関数
pub fn add_up_structures(structure_1: &Structure, structure_2: &Structure) -> Structure {
    if let (Structure::Composition(ref s1), Structure::Composition(ref s2)) =
        (structure_1, structure_2)
    {
        // s1とs2の要素をクローンして新しいVecに結合
        let add_up_entity: Vec<Structure> = s1
            .entity
            .iter()
            .cloned()
            .chain(s2.entity.iter().cloned())
            .collect();
        return Structure::Composition(Composition::new(add_up_entity));
    }
    unreachable!()
}

pub fn extract_contains_structures(
    compares: &Vec<Structure>,
    parent: &Structure,
) -> Vec<Structure> {
    let mut contains: Vec<Structure> = Vec::new();

    if let Structure::Composition(ref parent_content) = parent {
        for child in parent_content.entity.iter() {
            // comparesの中で、pと比較してtrueになる要素を探す
            for compare in compares.iter() {
                if compare_structures(child, compare) {
                    // 条件に合致する場合、containsに追加
                    // ただし、containsに既に同じ要素がないかを確認
                    if !contains.iter().any(|x| compare_structures(x, compare)) {
                        contains.push(compare.clone());
                    }
                }
            }
        }
    }
    return contains;
}

pub fn all_different(compares: &Vec<Structure>, parent: &Structure) -> bool {
    let contains = extract_contains_structures(compares, parent);

    for x in contains.iter() {
        for y in contains.iter() {
            if x != y {
                if let (Structure::Element(ref x_content), Structure::Element(ref y_content)) =
                    (x, y)
                {
                    if x_content.val == y_content.val {
                        return false;
                    }
                }
            }
        }
    }
    return true;
}

//TODO: C以外も実装
pub fn adjacent(element: &Structure, parent: &Vec<Structure>) -> Vec<Structure> {
    if let Structure::Element(ref element_content) = element {
        match element_content.attr {
            Attribute::C => {
                let x = element_content.coor.1;
                let y = element_content.coor.0;
                let mut result: Vec<Structure> = Vec::new();
                let coor_top = Coordinate(y - 1, x);
                let coor_bottom = Coordinate(y + 1, x);
                let coor_left = Coordinate(y, x - 1);
                let coor_right = Coordinate(y, x + 1);

                for child in parent.iter() {
                    if let Structure::Element(ref child_content) = child {
                        if child_content.coor == coor_top
                            || child_content.coor == coor_bottom
                            || child_content.coor == coor_left
                            || child_content.coor == coor_right
                        {
                            result.push(child.clone());
                        }
                    }
                }
                return result;
            }
            _ => unreachable!(),
        }
    }
    unreachable!();
}
