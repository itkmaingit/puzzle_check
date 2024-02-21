use std::fmt;

// 元素ではない構造体
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Composition {
    pub val: Option<i32>,
    pub entity: Vec<Structure>,
}

// 元素
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Element {
    pub val: Option<i32>,
    pub attr: Attribute,
    pub coor: Coordinate,
}

// 構造体enum
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Structure {
    Composition(Composition),
    Element(Element),
}

// 元素の属性
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Attribute {
    P,
    C,
    Hp,
    Vp,
    Hc,
    Vc,
}

// 元素の座標
#[derive(Clone, PartialEq, Hash, Eq)]
pub struct Coordinate(pub i32, pub i32);

// 盤面のサイズ
pub struct BoardSize(pub i32, pub i32);

impl Element {
    // コンストラクタ, 初期解はNone
    pub fn new(attr: Attribute, coor: Coordinate) -> Self {
        Element {
            val: None,
            attr,
            coor,
        }
    }
}

impl Composition {
    // コンストラクタ, 初期解はNone
    pub fn new(entity: Vec<Structure>) -> Self {
        Composition { val: None, entity }
    }
}

//TODO: StructureにCompositon, Elementに対してそれぞれvalのsetter, getterの実装

// 座標を操るうえで便利なメソッド
impl Coordinate {
    // 差分だけ動かす
    fn change_coordinate(&mut self, coor: Coordinate) {
        self.0 += coor.0;
        self.1 += coor.1;
    }

    // 両隣の座標を返す, board_sizeを超える場合があることに注意
    pub fn horizon_points(&self) -> (Coordinate, Coordinate) {
        let left = self.clone();
        let mut right = self.clone();
        right.change_coordinate(Coordinate(0, 1));
        return (left, right);
    }

    // 上下の座標を返す, board_sizeを超える場合があることに注意
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

// ---------------------------------------------------------------------------------------------------------------------
// ↓ dataclassのDebug trait の実装

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

impl fmt::Debug for Element {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:?}{:?}, {:?})", self.attr, self.coor, self.val)
    }
}

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

impl fmt::Debug for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.0, self.1)
    }
}
