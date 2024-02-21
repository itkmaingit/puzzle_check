use crate::common::dataclass::{Attribute, BoardSize, Coordinate, Element, Structure};
use crate::common::relationship::{relationship, Relationship, D, H, M, V};
use std::collections::HashMap;

pub struct StructureFn {}

impl StructureFn {
    // ---------------------------------------------------------------------------------------------------------------------
    // ↓ for cell

    // ---------------------------------------------------------------------------------------------------------------------
    // [overview]
    // 特定の細胞の周りの辺の本数を計上する関数
    // ---------------------------------------------------------------------------------------------------------------------
    // [params]
    // cell: &Structure - cycleを調べたい細胞
    // Ep: &Vec<Structure> - 格子点辺の集合
    // board_size: &BoardSize - 盤面のサイズ
    // ---------------------------------------------------------------------------------------------------------------------
    // [return]
    // i32 - cycleの結果
    // ---------------------------------------------------------------------------------------------------------------------
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

    // ---------------------------------------------------------------------------------------------------------------------
    // [overview]
    // ある細胞が盤面の端であるか否かを確かめる関数
    // ---------------------------------------------------------------------------------------------------------------------
    // [params]
    // cell: &Structure - 端であるか否かを確かめたい細胞
    // board_size: &BoardSize - 盤面のサイズ
    // ---------------------------------------------------------------------------------------------------------------------
    // [return]
    // bool - 細胞が端である場合true
    // ---------------------------------------------------------------------------------------------------------------------
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

    // ---------------------------------------------------------------------------------------------------------------------
    // ↓ for graph

    // ---------------------------------------------------------------------------------------------------------------------
    // [overview]
    // グラフの端の元素を返す関数
    // ---------------------------------------------------------------------------------------------------------------------
    // [params]
    // line: &Structure - グラフ(combine([H,V,D], Ep) or combine([H,V,D], Ec))
    // ---------------------------------------------------------------------------------------------------------------------
    // [return]
    // Vec<Structure> - グラフの端の元素の配列(2つ)
    // ---------------------------------------------------------------------------------------------------------------------
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
                        _ => unreachable!(),
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
    // ---------------------------------------------------------------------------------------------------------------------
    // ↓ for element

    //TODO: C以外も実装
    // ---------------------------------------------------------------------------------------------------------------------
    // [overview]
    // ある元素に隣接する（縦横）元素を返す関数
    // ---------------------------------------------------------------------------------------------------------------------
    // [params]
    // compares: &Structure - 中心となる元素
    // parent: &Vec<Structure> - 元素と同一の元素列
    // ---------------------------------------------------------------------------------------------------------------------
    // [return]
    // bool - 隣接する元素
    // ---------------------------------------------------------------------------------------------------------------------
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
}
