use crate::common::dataclass::{Attribute, Composition, Coordinate, Element, Structure};
use crate::common::relationship::{relationship, Relationship, D, H, M, V};
use crate::specific::cutoff::CutoffFn;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::collections::{HashSet, VecDeque};

// ---------------------------------------------------------------------------------------------------------------------
// [overview]
// 組成演算を行う関数
// ---------------------------------------------------------------------------------------------------------------------
// [params]
// R: Vec<Relationship> - グラフ構造を展開するための隣接関係
// not_R: Vec<Relationship> - サブグラフの内部で満たしてはいけない隣接関係
// E: &Vec<Structure> - 構造体全てを含む集合
// cutoff_fn: &Vec<CutoffFn> - constraintsから、現れないことが分かっている構造体をあらかじめ省くための関数列（カットオフを行わないときはnon_cutoffを入力）
// ---------------------------------------------------------------------------------------------------------------------
// [return]
// Vec<Structure> - 組成演算の結果、作成される構造体全てを含む集合
// ---------------------------------------------------------------------------------------------------------------------
pub fn combine(
    R: Vec<Relationship>,
    not_R: Vec<Relationship>,
    E: &Vec<Structure>,
    cutoff_fn: &Vec<CutoffFn>,
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
            if !is_connected_graph(&s, &R) {
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

// ---------------------------------------------------------------------------------------------------------------------
// [overview]
// 構造体が連結グラフであることを保証する関数(BFS)
// ---------------------------------------------------------------------------------------------------------------------
// [params]
// structure: &Structure - チェックしたい構造体
// R: &Vec<Relationship> - 満たすべき隣接関係
// ---------------------------------------------------------------------------------------------------------------------
// [return]
// bool - 連結グラフであればtrue
// ---------------------------------------------------------------------------------------------------------------------
fn is_connected_graph(structure: &Structure, R: &Vec<Relationship>) -> bool {
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

// ---------------------------------------------------------------------------------------------------------------------
// [overview]
// 任意の配列から冪集合を作成する関数
// ---------------------------------------------------------------------------------------------------------------------
// [params]
// set: &[T] - 部分集合を作成したい配列
// pb: indicatif::ProgressBar - 計算量が簡単に爆発しうるので、プログレスバーを入力しておく
// ---------------------------------------------------------------------------------------------------------------------
// [return]
// bool - 作成された部分集合
// ---------------------------------------------------------------------------------------------------------------------
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
