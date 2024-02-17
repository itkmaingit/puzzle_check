use crate::common::dataclass::{Attribute, BoardSize, Composition, Element, Structure};
use crate::specific::board::BoardValidationFn;
use indicatif::{ProgressBar, ProgressStyle};
use rand::Rng;
use rayon::prelude::*;

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

//
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

pub fn extract_random_structure(items: &Vec<Structure>) -> Vec<Structure> {
    let mut rng = rand::thread_rng(); // 乱数生成器のインスタンス
    let index = rng.gen_range(0..items.len()); // ランダムなインデックスを生成
    return vec![items.get(index).unwrap().clone()]; // ランダムに選ばれた要素を返す
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
