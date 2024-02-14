use std::fmt;

#[derive(Clone, PartialEq)]
pub struct Composition {
    pub val: Option<i32>,
    pub entity: Vec<Structure>,
}

#[derive(Clone, PartialEq)]
pub struct Element {
    pub val: Option<i32>,
    pub attr: Attribute,
    pub coor: Coordinate,
}

#[derive(Clone, PartialEq)]
pub enum Structure {
    Composition(Composition),
    Element(Element),
}

#[derive(Clone, PartialEq)]
pub enum Attribute {
    P,
    C,
    Hp,
    Vp,
    Hc,
    Vc,
}

#[derive(Clone, PartialEq)]
pub struct Coordinate(pub i32, pub i32);

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
        write!(f, "{:?}, {{", self.val)?;
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
