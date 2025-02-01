use std::fmt::Display;

#[derive(Clone, Debug, Copy, Eq, PartialEq)]
pub enum Tile {
    Concealed(usize),
    ConcealedMine,
    RevealedMine,
    Revealed(usize),
}

impl Tile {
    pub fn reveal(self) -> Self {
        match self {
            Tile::Concealed(v) => Tile::Revealed(v),
            Tile::ConcealedMine | Tile::RevealedMine => Tile::RevealedMine,
            Tile::Revealed(v) => Tile::Revealed(v),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let symbol = match self {
            Tile::Concealed(_) | Tile::ConcealedMine => "#".to_string(),
            Tile::RevealedMine => "X".to_string(),
            Tile::Revealed(v) => v.to_string(),
        };
        write!(f, "{}", symbol)
    }
}
