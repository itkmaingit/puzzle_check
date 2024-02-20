use crate::common::dataclass::{Attribute, BoardSize, Coordinate, Element, Structure};
use crate::common::function::is_side;
use crate::common::relationship::{relationship, Relationship, D, H, M, V};

// カットオフ条件（正しい盤面である）を満たしていればTrueを返す
pub type BoardValidationFn = fn(B: &Vec<Structure>) -> bool;

pub fn non_validation(_: &Vec<Structure>) -> bool {
    return true;
}

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
