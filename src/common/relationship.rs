use crate::common::dataclass::{Attribute, Coordinate, Element, Structure};

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

    match (&X.attr, &Y.attr) {
        (Attribute::P, Attribute::P)
        | (Attribute::C, Attribute::C)
        | (Attribute::Hp, Attribute::Hp)
        | (Attribute::Hc, Attribute::Hc) => {
            if dx.abs() == 1 && dy == 0 {
                return true;
            }
        }
        (Attribute::P, Attribute::Hp) | (Attribute::C, Attribute::Hc) => {
            if (dx == 0 || dx == 1) && dy == 0 {
                return true;
            }
        }
        (Attribute::Hp, Attribute::P) | (Attribute::Hc, Attribute::C) => {
            if (dx == 0 || dx == -1) && dy == 0 {
                return true;
            }
        }
        _ => return false,
    };

    return false;
}

pub fn V(X: &Element, Y: &Element) -> bool {
    let dx = X.coor.1 - Y.coor.1;
    let dy = X.coor.0 - Y.coor.0;

    match (&X.attr, &Y.attr) {
        (Attribute::P, Attribute::P)
        | (Attribute::C, Attribute::C)
        | (Attribute::Vp, Attribute::Vp)
        | (Attribute::Vc, Attribute::Vc) => {
            if dx == 0 && dy.abs() == 1 {
                return true;
            }
        }
        (Attribute::P, Attribute::Vp) | (Attribute::C, Attribute::Vc) => {
            if dx == 0 && (dy == 0 || dy == 1) {
                return true;
            }
        }
        (Attribute::Vp, Attribute::P) | (Attribute::Vc, Attribute::C) => {
            if dx == 0 && (dy == 0 || dy == -1) {
                return true;
            }
        }
        _ => return false,
    };

    return false;
}

pub fn D(X: &Element, Y: &Element) -> bool {
    let dx = X.coor.1 - Y.coor.1;
    let dy = X.coor.0 - Y.coor.0;

    match (&X.attr, &Y.attr) {
        (Attribute::C, Attribute::C) | (Attribute::P, Attribute::P) => {
            if dx.abs() == 1 && dy.abs() == 1 {
                return true;
            }
        }
        (Attribute::Hp, Attribute::Vp) | (Attribute::Hc, Attribute::Vc) => {
            if (dx == 0 || dx == -1) && (dy == 0 || dy == 1) {
                return true;
            }
        }
        (Attribute::Vp, Attribute::Hp) | (Attribute::Vc, Attribute::Hc) => {
            if (dx == 0 || dx == 1) && (dy == 0 || dy == -1) {
                return true;
            }
        }
        _ => return false,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_H() {
        let p21 = Element::new(Attribute::P, Coordinate(2, 1));
        let p22 = Element::new(Attribute::P, Coordinate(2, 2));
        let p23 = Element::new(Attribute::P, Coordinate(2, 3));
        let hp22 = Element::new(Attribute::Hp, Coordinate(2, 2));
        let hp23 = Element::new(Attribute::Hp, Coordinate(2, 3));
        assert_eq!(H(&p21, &p22), true);
        assert_eq!(H(&p22, &p21), true);
        assert_eq!(H(&hp22, &hp23), true);
        assert_eq!(H(&hp23, &hp22), true);
        assert_eq!(H(&p22, &hp22), true);
        assert_eq!(H(&hp22, &p22), true);
        assert_eq!(H(&p22, &hp23), false);
        assert_eq!(H(&hp23, &p22), false);
        assert_eq!(H(&p21, &hp22), false);
        assert_eq!(H(&hp22, &p21), false);
        assert_eq!(H(&p21, &hp23), false);
        assert_eq!(H(&p21, &p23), false);
    }

    #[test]
    fn test_V() {
        let p23 = Element::new(Attribute::P, Coordinate(2, 3));
        let p33 = Element::new(Attribute::P, Coordinate(3, 3));
        let p43 = Element::new(Attribute::P, Coordinate(4, 3));
        let vp23 = Element::new(Attribute::Vp, Coordinate(2, 3));
        let vp33 = Element::new(Attribute::Vp, Coordinate(3, 3));
        let vp43 = Element::new(Attribute::Vp, Coordinate(4, 3));
        assert_eq!(V(&p23, &p33), true);
        assert_eq!(V(&p33, &p23), true);
        assert_eq!(V(&p23, &vp23), true);
        assert_eq!(V(&vp23, &p23), true);
        assert_eq!(V(&p33, &vp23), true);
        assert_eq!(V(&vp23, &p33), true);
        assert_eq!(V(&vp33, &vp23), true);
        assert_eq!(V(&vp23, &vp33), true);
        assert_eq!(V(&p43, &p23), false);
        assert_eq!(V(&p23, &p43), false);
        assert_eq!(V(&vp23, &vp43), false);
        assert_eq!(V(&vp43, &vp23), false);
        assert_eq!(V(&p23, &vp33), false);
        assert_eq!(V(&vp33, &p23), false);
    }

    #[test]
    fn test_D() {
        let hp31 = Element::new(Attribute::Hp, Coordinate(3, 1));
        let hp32 = Element::new(Attribute::Hp, Coordinate(3, 2));
        let hp33 = Element::new(Attribute::Hp, Coordinate(3, 3));
        let hp41 = Element::new(Attribute::Hp, Coordinate(4, 1));
        let hp42 = Element::new(Attribute::Hp, Coordinate(4, 2));
        let hp43 = Element::new(Attribute::Hp, Coordinate(4, 3));
        let vp22 = Element::new(Attribute::Vp, Coordinate(2, 2));
        let vp23 = Element::new(Attribute::Vp, Coordinate(2, 3));
        let vp32 = Element::new(Attribute::Vp, Coordinate(3, 2));
        let vp33 = Element::new(Attribute::Vp, Coordinate(3, 3));
        let vp42 = Element::new(Attribute::Vp, Coordinate(4, 2));
        let vp43 = Element::new(Attribute::Vp, Coordinate(4, 3));
        assert_eq!(D(&vp22, &hp32), true);
        assert_eq!(D(&hp32, &vp22), true);
        assert_eq!(D(&vp22, &hp31), true);
        assert_eq!(D(&hp31, &vp22), true);
        assert_eq!(D(&hp32, &vp23), true);
        assert_eq!(D(&vp23, &hp32), true);
        assert_eq!(D(&vp33, &hp33), true);
        assert_eq!(D(&hp33, &vp23), true);
        assert_eq!(D(&hp43, &vp32), false);
        assert_eq!(D(&vp32, &hp43), false);
        assert_eq!(D(&vp23, &vp43), false);
        assert_eq!(D(&hp41, &vp33), false);
        assert_eq!(D(&vp33, &hp41), false);
        assert_eq!(D(&vp42, &vp33), false);
        assert_eq!(D(&vp33, &vp42), false);
    }
}
