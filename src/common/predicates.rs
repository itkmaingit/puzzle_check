use crate::common::dataclass::{Attribute, BoardSize, Composition, Element, Structure};

pub fn is_rectangle(area: &Structure) -> bool {
    let mut minX = std::i32::MAX;
    let mut minY = std::i32::MAX;
    let mut maxX = -std::i32::MAX;
    let mut maxY = -std::i32::MAX;

    if let Structure::Composition(ref area_content) = area {
        for cell in area_content.entity.iter() {
            if let Structure::Element(ref cell_content) = cell {
                let x = cell_content.coor.1;
                let y = cell_content.coor.0;
                minX = std::cmp::min(minX, x);
                minY = std::cmp::min(minY, y);
                maxX = std::cmp::max(maxX, x);
                maxY = std::cmp::max(maxY, y);
            }
        }
        let width = maxX - minX + 1;
        let height = maxY - minY + 1;
        let size = width * height;

        return area_content.entity.len() as i32 == size;
    }

    unreachable!();
}
