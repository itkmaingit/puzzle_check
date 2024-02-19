use crate::common::dataclass::{Attribute, BoardSize, Coordinate, Element, Structure};

pub fn initialize(
    board_size: &BoardSize,
) -> (
    Vec<Structure>,
    Vec<Structure>,
    Vec<Structure>,
    Vec<Structure>,
) {
    let n = board_size.0;
    let m = board_size.1;
    let mut P: Vec<Structure> = Vec::new();
    let mut C: Vec<Structure> = Vec::new();
    let mut Ep: Vec<Structure> = Vec::new();
    let mut Ec: Vec<Structure> = Vec::new();

    for i in 0..n + 1 {
        for j in 0..m + 1 {
            let coordinate = Coordinate(i + 1, j + 1);
            let point = Structure::Element(Element::new(Attribute::P, coordinate));
            P.push(point);
        }
    }

    for i in 0..n {
        for j in 0..m {
            let coordinate = Coordinate(i + 1, j + 1);
            let cell = Structure::Element(Element::new(Attribute::C, coordinate));
            C.push(cell);
        }
    }

    for i in 0..n + 1 {
        for j in 0..m {
            let coordinate = Coordinate(i + 1, j + 1);
            let point_horizon_edge = Structure::Element(Element::new(Attribute::Hp, coordinate));
            Ep.push(point_horizon_edge);
        }
    }

    for i in 0..n {
        for j in 0..m + 1 {
            let coordinate = Coordinate(i + 1, j + 1);
            let point_vertical_edge = Structure::Element(Element::new(Attribute::Vp, coordinate));
            Ep.push(point_vertical_edge);
        }
    }

    for i in 0..n {
        for j in 0..m - 1 {
            let coordinate = Coordinate(i + 1, j + 1);
            let cell_horizon_edge = Structure::Element(Element::new(Attribute::Hc, coordinate));
            Ec.push(cell_horizon_edge);
        }
    }

    for i in 0..n - 1 {
        for j in 0..m {
            let coordinate = Coordinate(i + 1, j + 1);
            let cell_vertical_edge = Structure::Element(Element::new(Attribute::Vc, coordinate));
            Ec.push(cell_vertical_edge);
        }
    }
    return (P, C, Ep, Ec);
}
