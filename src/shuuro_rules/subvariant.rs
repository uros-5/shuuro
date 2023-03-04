use crate::Variant;

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum SubVariant {
    Standard = 0,
    StandardFairy1,
    StandardFairy2,
    StandardPlacement,
}

impl SubVariant {
    pub fn starting_position(&self) -> &str {
        match self {
            SubVariant::Standard => {
                "RNBQKBNR/PPPPPPPP/8/8/8/8/pppppppp/rnbqkbnr w - 1"
            }
            SubVariant::StandardFairy1 => {
                "RNA1KCNR/PPPPPPPP/8/8/8/8/pppppppp/rna1kcnr w - 1"
            }
            SubVariant::StandardFairy2 => {
                "RGB1KBAR/PPPPPPPP/8/8/8/8/pppppppp/rgb1kbar w - 1"
            }
            SubVariant::StandardPlacement => {
                "8/PPPPPPPP/8/8/8/8/pppppppp/8 w 2R2B2NQK2r2b2nqk 1"
            }
        }
    }

    pub fn starting_stage(&self) -> u8 {
        match self {
            SubVariant::Standard => 2,
            SubVariant::StandardFairy1 => 2,
            SubVariant::StandardFairy2 => 2,
            SubVariant::StandardPlacement => 1,
        }
    }

    pub fn is_valid(&self, variant: Variant) -> bool {
        match self {
            SubVariant::Standard | SubVariant::StandardPlacement => {
                variant == Variant::Standard
            }
            SubVariant::StandardFairy1 | SubVariant::StandardFairy2 => {
                variant == Variant::StandardFairy
            }
        }
    }

    pub fn index(self) -> usize {
        self as usize
    }

    pub fn valid_index(index: u8) -> bool {
        index < 4
    }
}

impl TryFrom<u8> for SubVariant {
    type Error = Option<u8>;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(SubVariant::Standard),
            1 => Ok(SubVariant::StandardFairy1),
            2 => Ok(SubVariant::StandardFairy2),
            3 => Ok(SubVariant::StandardPlacement),
            _ => Err(None),
        }
    }
}
