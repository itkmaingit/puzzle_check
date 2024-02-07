#[derive(Clone, PartialEq)]
pub struct Composition {
    pub val: Option<i32>,
    pub entity: Vec<Box<Structure>>,
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
    pub fn new(entity: Vec<Box<Structure>>) -> Self {
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
