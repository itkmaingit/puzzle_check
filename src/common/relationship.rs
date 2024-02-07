use crate::common::dataclass::{Attribute, Element, Structure};

pub type Relationship = fn(&Element, &Element) -> bool;

pub fn relationship(X: &Structure, Y: &Structure, R: fn(&Element, &Element) -> bool) -> bool {
    if let (Structure::Composition(ref x), Structure::Composition(ref y)) = (X, Y) {
        for x_entity in &x.entity {
            for y_entity in &y.entity {
                if relationship(&x_entity, &y_entity, R) {
                    return true;
                }
            }
        }
    } else if let (Structure::Element(ref x), Structure::Element(ref y)) = (X, Y) {
        return R(x, y);
    }
    return false;
}

pub fn H(X: &Element, Y: &Element) -> bool {
    let dx = X.coor.1 - Y.coor.1;
    let dy = X.coor.0 - Y.coor.0;

    if dx.abs() == 1 && dy == 0 {
        match (&X.attr, &Y.attr) {
            (Attribute::P, Attribute::P)
            | (Attribute::C, Attribute::C)
            | (Attribute::Hp, Attribute::Hp)
            | (Attribute::Hc, Attribute::Hc) => return true,
            _ => false,
        };
    }

    if (dx == 0 || dx == 1) && dy == 0 {
        match (&X.attr, &Y.attr) {
            (Attribute::P, Attribute::Hp) | (Attribute::C, Attribute::Hc) => return true,
            _ => false,
        };
    }

    if (dx == 0 || dx == -1) && dy == 0 {
        match (&X.attr, &Y.attr) {
            (Attribute::Hp, Attribute::P) | (Attribute::Hc, Attribute::C) => return true,
            _ => false,
        };
    }

    return false;
}

pub fn V(X: &Element, Y: &Element) -> bool {
    let dx = X.coor.1 - Y.coor.1;
    let dy = X.coor.0 - Y.coor.0;

    if dx == 0 && dy.abs() == 1 {
        match (&X.attr, &Y.attr) {
            (Attribute::P, Attribute::P)
            | (Attribute::C, Attribute::C)
            | (Attribute::Vp, Attribute::Vp)
            | (Attribute::Vc, Attribute::Vc) => return true,
            _ => false,
        };
    }

    if dx == 0 && (dy == 0 || dy == 1) {
        match (&X.attr, &Y.attr) {
            (Attribute::P, Attribute::Hp) | (Attribute::C, Attribute::Hc) => return true,
            _ => false,
        };
    }

    if dx == 0 && (dy == 0 || dy == 1) {
        match (&X.attr, &Y.attr) {
            (Attribute::Vp, Attribute::P) | (Attribute::Vc, Attribute::C) => return true,
            _ => false,
        };
    }

    return false;
}

pub fn D(X: &Element, Y: &Element) -> bool {
    let dx = X.coor.1 - Y.coor.1;
    let dy = X.coor.0 - Y.coor.0;

    if dx.abs() == 1 && dy.abs() == 1 {
        match (&X.attr, &Y.attr) {
            (Attribute::C, Attribute::C) | (Attribute::P, Attribute::P) => return true,
            _ => return false,
        };
    }

    if (dx == 0 || dx == 1) && (dy == 0 || dy == -1) {
        match (&X.attr, &Y.attr) {
            (Attribute::Hp, Attribute::Vp) | (Attribute::Hc, Attribute::Vc) => return true,
            _ => return false,
        };
    }

    if (dx == 0 || dx == -1) && (dy == 0 || dy == 1) {
        match (&X.attr, &Y.attr) {
            (Attribute::Vp, Attribute::Hp) | (Attribute::Vc, Attribute::Hc) => return true,
            _ => return false,
        };
    }
    return false;
}

pub fn M(X: &Element, Y: &Element) -> bool {
    let dx = X.coor.0 - Y.coor.0;
    let dy = X.coor.1 - Y.coor.1;

    if dx == 0 && dy == 0 {
        match (&X.attr, &Y.attr) {
            (Attribute::P, Attribute::P)
            | (Attribute::C, Attribute::C)
            | (Attribute::Hp, Attribute::Hp)
            | (Attribute::Vp, Attribute::Vp)
            | (Attribute::Hc, Attribute::Hc)
            | (Attribute::Vc, Attribute::Vc) => return true,
            _ => false,
        };
    }
    return false;
}
