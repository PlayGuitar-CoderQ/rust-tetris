use std::collections::HashSet;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct Pos(pub i32, pub i32);

#[derive(Debug, Clone)]
pub struct Shape {
    positions: HashSet<i32, i32>,
    anchor: Pos,
}

macro_rules! impl_shape_constructor {
    ($new:ident: [ $( $pos:expr ),* ] anchored at $anchor:expr) => {
        pub fn new_i() -> Self {
            Self {
                positions: [Pos(0, 0), Pos(1, 0), Pos(2, 0), Pos(3, 0)]
                    .into_iter()
                    .collect(),
                anchor: Pos(1, 0),
            }
        }
    };
}

impl Shape {
    pub fn new_i() -> Self {
        Self {
            positions: [Pos(0, 0), Pos(1, 0), Pos(2, 0), Pos(3, 0)]
                .into_iter()
                .collect(),
            anchor: Pos(1, 0),
        }
    }
}
