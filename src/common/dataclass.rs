use std::fmt;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Composition {
    pub val: Option<i32>,
    pub entity: Vec<Structure>,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Element {
    pub val: Option<i32>,
    pub attr: Attribute,
    pub coor: Coordinate,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Structure {
    Composition(Composition),
    Element(Element),
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Attribute {
    P,
    C,
    Hp,
    Vp,
    Hc,
    Vc,
}

#[derive(Clone, PartialEq, Hash, Eq)]
pub struct Coordinate(pub i32, pub i32);

pub struct BoardSize(pub i32, pub i32);

impl Element {
    pub fn new(attr: Attribute, coor: Coordinate) -> Self {
        Element {
            val: None,
            attr,
            coor,
        }
    }
}

impl Composition {
    pub fn new(entity: Vec<Structure>) -> Self {
        Composition { val: None, entity }
    }
}

impl Element {
    pub fn solution(&self) -> i32 {
        return self.val.unwrap();
    }
}

impl Composition {
    pub fn solution(&self) -> i32 {
        return self.val.unwrap();
    }
}

impl Coordinate {
    fn change_coordinate(&mut self, coor: Coordinate) {
        self.0 += coor.0;
        self.1 += coor.1;
    }
    pub fn horizon_points(&self) -> (Coordinate, Coordinate) {
        let left = self.clone();
        let mut right = self.clone();
        right.change_coordinate(Coordinate(0, 1));
        return (left, right);
    }

    pub fn vertical_points(&self) -> (Coordinate, Coordinate) {
        let top = self.clone();
        let mut bottom = self.clone();
        bottom.change_coordinate(Coordinate(1, 0));
        return (top, bottom);
    }
}

// StructureのDebugトレイトを実装
impl fmt::Debug for Structure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Structure::Composition(comp) => writeln!(f, "<{:?}>", comp),
            Structure::Element(elem) => write!(f, "{:?}", elem),
        }
    }
}

// CompositionのDebugトレイトを実装
impl fmt::Debug for Composition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "val: {:?}, length: {:?} {{", self.val, self.entity.len())?;
        for (i, entity) in self.entity.iter().enumerate() {
            if i != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{:?}", entity)?;
        }
        write!(f, "}}")
    }
}

// ElementのDebugトレイトを実装
impl fmt::Debug for Element {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:?}{:?}, {:?})", self.attr, self.coor, self.val)
    }
}

// AttributeのDebugトレイトを実装
impl fmt::Debug for Attribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Attribute::P => write!(f, "p"),
            Attribute::C => write!(f, "c"),
            Attribute::Hp => write!(f, "hp"),
            Attribute::Vp => write!(f, "vp"),
            Attribute::Hc => write!(f, "hc"),
            Attribute::Vc => write!(f, "vc"),
        }
    }
}

// CoordinateのDebugトレイトを実装
impl fmt::Debug for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.0, self.1)
    }
}
