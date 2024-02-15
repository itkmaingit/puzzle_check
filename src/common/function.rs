use crate::common::dataclass::{Composition, Element, Structure};
use indicatif::{ProgressBar, ProgressStyle};
use rand::Rng;
use rayon::prelude::*; // 乱数生成器を使用するために必要

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
        _ => false,
    }
}

pub fn random_subset<T: Clone>(set: &[T]) -> Vec<T> {
    let mut rng = rand::thread_rng(); // 乱数生成器のインスタンス
    let mut subset = Vec::new();

    for item in set {
        // 各要素について、50%の確率で部分集合に含める
        if rng.gen_bool(0.5) {
            subset.push(item.clone());
        }
    }

    subset
}
