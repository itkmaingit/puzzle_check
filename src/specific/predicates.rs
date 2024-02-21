use crate::common::dataclass::{Attribute, BoardSize, Coordinate, Element, Structure};
use crate::common::operate_structures::OperateStructure;
use crate::common::relationship::{relationship, Relationship, D, H, M, V};

pub struct Predicates {}
impl Predicates {
    // ---------------------------------------------------------------------------------------------------------------------
    // [overview]
    // ある構造体を受け取り、その構造体に含まれる元素の解が全て異なることを確認する関数
    // ---------------------------------------------------------------------------------------------------------------------
    // [params]
    // compares: &Vec<Structure> - 元素列
    // parent: &Structure - 構造体
    // ---------------------------------------------------------------------------------------------------------------------
    // [return]
    // bool - 全て異なるときにtrue
    // ---------------------------------------------------------------------------------------------------------------------
    pub fn all_different(compares: &Vec<Structure>, parent: &Structure) -> bool {
        let contains = OperateStructure::extract_contains_structures(compares, parent);

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
}
