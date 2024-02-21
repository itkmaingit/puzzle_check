use crate::common::dataclass::{Attribute, BoardSize, Composition, Coordinate, Element, Structure};
use crate::specific::board_validation::BoardValidationFn;
use indicatif::{ProgressBar, ProgressStyle};
use rand::Rng;
use rayon::prelude::*;
use std::collections::HashMap;

pub struct OperateStructure {}

impl OperateStructure {
    // ---------------------------------------------------------------------------------------------------------------------
    // [overview]
    // 比較した構造体が同一のものであるかを確認（値チェックは行わず, 同じ座標, 同じ属性の元素を含んでいるかを確認する）
    // StructureならElement, CompositionどちらでもOK
    // ---------------------------------------------------------------------------------------------------------------------
    // [params]
    // s1: &Structure - 比較したい構造体 1
    // s2: &Structure - 比較したい構造体 2
    // ---------------------------------------------------------------------------------------------------------------------
    // [return]
    // bool - 同一の構造体であればtrue
    // ---------------------------------------------------------------------------------------------------------------------
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
                            // c2のentity内で, el1と同じCoordinateとAttributeを持つElementが存在するか
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
                        // c1のentity内で, el2と同じCoordinateとAttributeを持つElementが存在するか
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

    // ---------------------------------------------------------------------------------------------------------------------
    // [overview]
    // 構造体全てを含む集合から, ランダムに部分集合を作成する関数
    // バリデーションを入れることも可. バリデーションがいらなければnon_validationを入力
    // 実質的な無限ループに注意, 基本的に構造体のサイズに制限を入れずにboard_validation_fnを用意すると爆発する
    // ---------------------------------------------------------------------------------------------------------------------
    // [params]
    // set: &Vec<Structure> - 構造体全てを含む集合, 基本的にcombineの返り値を入れればよい.
    // board_validation_fn: &Vec<BoardValidationFn> - 盤面に存在する構造体同士が満たすべきバリデーションを入れる. 重なっていない(Mでない)を入力することが多い.
    // ---------------------------------------------------------------------------------------------------------------------
    // [return]
    // Vec<Structure> - 作成された部分集合
    // ---------------------------------------------------------------------------------------------------------------------
    pub fn random_subset_with_validation(
        set: &Vec<Structure>,
        board_validation_fn: &Vec<BoardValidationFn>,
    ) -> Vec<Structure> {
        let mut rng = rand::thread_rng(); // 乱数生成器のインスタンス
        'outer: loop {
            let mut subset = Vec::new();
            for item in set {
                // 各要素について, 50%の確率で部分集合に含める
                if rng.gen_bool(0.5) {
                    subset.push(item.clone());
                }
            }
            if subset.len() == 0 {
                continue 'outer;
            }
            for function in board_validation_fn {
                if !function(&subset) {
                    continue 'outer;
                }
            }
            return subset;
        }
    }

    // ---------------------------------------------------------------------------------------------------------------------
    // [overview]
    // 構造体全てを含む集合から, ランダムに構造体をただ一つだけ取り出す関数
    // 基本的にB(X)=1のconstraintsの時に使用する
    // ---------------------------------------------------------------------------------------------------------------------
    // [params]
    // structures: &Vec<Structure> - 構造体全てを含む集合, 基本的にcombineの返り値を入れればよい.
    // ---------------------------------------------------------------------------------------------------------------------
    // [return]
    // Structure - ランダムに取り出された構造体
    // ---------------------------------------------------------------------------------------------------------------------
    pub fn extract_random_structure(structures: &Vec<Structure>) -> Structure {
        let mut rng = rand::thread_rng(); // 乱数生成器のインスタンス
        let index = rng.gen_range(0..structures.len()); // ランダムなインデックスを生成
        return structures.get(index).unwrap().clone(); // ランダムに選ばれた要素を返す
    }

    // ---------------------------------------------------------------------------------------------------------------------
    // [overview]
    // for cut-off function
    // 同じ構造体同士を足し合わせることを前提とした関数（和集合）
    // constraintsでは使用してはならない(解がつぶれるため)
    // ---------------------------------------------------------------------------------------------------------------------
    // [params]
    // structure_1: &Structure - 足し合わせる構造体 1
    // structure_2: &Structure - 足し合わせる構造体 2
    // ---------------------------------------------------------------------------------------------------------------------
    // [return]
    // Structure - structure_1 ∪ structure_2 (足し合わされた構造体)
    // ---------------------------------------------------------------------------------------------------------------------
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

    // ---------------------------------------------------------------------------------------------------------------------
    // [overview]
    // for cut-off function
    // 同じ構造体同士を減ずる関数（差集合）
    // constraintsでは使用してはならない(解がつぶれるため)
    // ---------------------------------------------------------------------------------------------------------------------
    // [params]
    // minuend: &Structure - 元集合（基準集合）
    // subtrahend: &Structure - 取り除く集合
    // ---------------------------------------------------------------------------------------------------------------------
    // [return]
    // Structure - minuend \ subtrahend （minuend - subtrahend, 差集合）
    // ---------------------------------------------------------------------------------------------------------------------
    pub fn subtract_structures(minuend: &Structure, subtrahend: &Structure) -> Structure {
        if let (
            Structure::Composition(ref minuend_content),
            Structure::Composition(ref subtrahend_content),
        ) = (minuend, subtrahend)
        {
            let difference_entity: Vec<Structure> = minuend_content
                .entity
                .iter()
                .filter({
                    |&minuend_elem| {
                        !subtrahend_content.entity.iter().any(|subtrahend_elem| {
                            OperateStructure::compare_structures(minuend_elem, subtrahend_elem)
                        })
                    }
                })
                .cloned()
                .collect();

            return Structure::Composition(Composition::new(difference_entity));
        }
        unreachable!()
    }

    // ---------------------------------------------------------------------------------------------------------------------
    // [overview]
    // ある構造体から（主に）元素列から元素を取り出すための関数
    // 基本的に構造体内部に元素の解は保存されていないので, 元素列から解を抽出するために用いる
    // ---------------------------------------------------------------------------------------------------------------------
    // [params]
    // compares: &Vec<Structure> - 元素列
    // parent: &Structure - 構造体
    // ---------------------------------------------------------------------------------------------------------------------
    // [return]
    // Structure - minuend \ subtrahend （minuend - subtrahend, 差集合）
    // ---------------------------------------------------------------------------------------------------------------------
    pub fn extract_contains_structures(
        compares: &Vec<Structure>,
        parent: &Structure,
    ) -> Vec<Structure> {
        let mut contains: Vec<Structure> = Vec::new();

        if let Structure::Composition(ref parent_content) = parent {
            for child in parent_content.entity.iter() {
                // comparesの中で, pと比較してtrueになる要素を探す
                for compare in compares.iter() {
                    if OperateStructure::compare_structures(child, compare) {
                        // 条件に合致する場合, containsに追加
                        // ただし, containsに既に同じ要素がないかを確認
                        if !contains
                            .iter()
                            .any(|x| OperateStructure::compare_structures(x, compare))
                        {
                            contains.push(compare.clone());
                        }
                    }
                }
            }
        }
        return contains;
    }
}
