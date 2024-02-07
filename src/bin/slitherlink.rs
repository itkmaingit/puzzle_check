use puzzle_check::common::combine::combine;
use puzzle_check::common::dataclass::{Attribute, Composition, Coordinate, Element, Structure};
use puzzle_check::common::relationship::{relationship, Relationship, D, H, M, V};
use std::collections::HashSet;

fn main() {
    let n = 2;
    let m = 2;
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
            let cell_vertical_edge = Structure::Element(Element::new(Attribute::Hc, coordinate));
            Ec.push(cell_vertical_edge);
        }
    }

    // ----------------------------------------------------------------------
    let R: HashSet<Relationship> = vec![H, D, V].into_iter().collect();
    let G = combine(R, Ep);
    for g in G.iter() {
        println!("{:?}", g);
    }
}
