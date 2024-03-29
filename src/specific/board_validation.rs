use crate::common::dataclass::{Attribute, BoardSize, Coordinate, Element, Structure};
use crate::common::relationship::{relationship, Relationship, D, H, M, V};

// カットオフ条件（正しい盤面である）を満たしていればTrueを返す
pub type BoardValidationFn = fn(B: &Vec<Structure>) -> bool;

pub struct BoardValidation {}

impl BoardValidation {
    // ---------------------------------------------------------------------------------------------------------------------
    // ↓　non_validation

    // ---------------------------------------------------------------------------------------------------------------------
    // [overview]
    // バリデーションを行わない関数
    // ---------------------------------------------------------------------------------------------------------------------
    // [params]
    // _: &Vec<Structure> - 使用しない
    // ---------------------------------------------------------------------------------------------------------------------
    // [return]
    // bool - always true
    // ---------------------------------------------------------------------------------------------------------------------
    pub fn non_validation(_: &Vec<Structure>) -> bool {
        return true;
    }

    // ---------------------------------------------------------------------------------------------------------------------
    // ↓　relationship

    // ---------------------------------------------------------------------------------------------------------------------
    // [overview]
    // 盤面に存在する構造体同士が一致していないことを保証する関数
    // ---------------------------------------------------------------------------------------------------------------------
    // [params]
    // B: &Vec<Structure> - 盤面に存在する構造体の列
    // ---------------------------------------------------------------------------------------------------------------------
    // [return]
    // bool - 何一つとして一致していないときtrue
    // ---------------------------------------------------------------------------------------------------------------------
    pub fn non_matching_structures(B: &Vec<Structure>) -> bool {
        for s1 in B.iter() {
            for s2 in B.iter() {
                if s1 != s2 {
                    if relationship(s1, s2, M) {
                        return false;
                    }
                }
            }
        }
        return true;
    }

    // ---------------------------------------------------------------------------------------------------------------------
    // [overview]
    // 盤面に存在する構造体同士が横隣接していないことを保証する関数
    // ---------------------------------------------------------------------------------------------------------------------
    // [params]
    // B: &Vec<Structure> - 盤面に存在する構造体の列
    // ---------------------------------------------------------------------------------------------------------------------
    // [return]
    // bool - 何一つとして横隣接していないときtrue
    // ---------------------------------------------------------------------------------------------------------------------
    pub fn non_horizontal_structures(B: &Vec<Structure>) -> bool {
        for s1 in B.iter() {
            for s2 in B.iter() {
                if s1 != s2 {
                    if relationship(s1, s2, H) {
                        return false;
                    }
                }
            }
        }
        return true;
    }

    // ---------------------------------------------------------------------------------------------------------------------
    // [overview]
    // 盤面に存在する構造体同士が一致していないことを保証する関数
    // ---------------------------------------------------------------------------------------------------------------------
    // [params]
    // B: &Vec<Structure> - 盤面に存在する構造体の列
    // ---------------------------------------------------------------------------------------------------------------------
    // [return]
    // bool - 何一つとして縦隣接していないときtrue
    // ---------------------------------------------------------------------------------------------------------------------
    pub fn non_vertical_structures(B: &Vec<Structure>) -> bool {
        for s1 in B.iter() {
            for s2 in B.iter() {
                if s1 != s2 {
                    if relationship(s1, s2, V) {
                        return false;
                    }
                }
            }
        }
        return true;
    }

    // ---------------------------------------------------------------------------------------------------------------------
    // [overview]
    // 盤面に存在する構造体同士が斜隣接していないことを保証する関数
    // ---------------------------------------------------------------------------------------------------------------------
    // [params]
    // B: &Vec<Structure> - 盤面に存在する構造体の列
    // ---------------------------------------------------------------------------------------------------------------------
    // [return]
    // bool - 何一つとして斜隣接していないときtrue
    // ---------------------------------------------------------------------------------------------------------------------
    pub fn non_diagonal_structures(B: &Vec<Structure>) -> bool {
        for s1 in B.iter() {
            for s2 in B.iter() {
                if s1 != s2 {
                    if relationship(s1, s2, D) {
                        return false;
                    }
                }
            }
        }
        return true;
    }
}
