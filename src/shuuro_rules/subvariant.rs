use crate::Variant;

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum SubVariant {
    Standard = 0,
    StandardFairy1,
    StandardFairy2,
    StandardPlacement,
    NoSubVariant,
}

impl SubVariant {
    pub fn starting_position(&self) -> &str {
        match self {
            SubVariant::Standard => {
                "RNBQKBNR/PPPPPPPP/8/8/8/8/pppppppp/rnbkqbnr w - 1"
            }
            SubVariant::StandardFairy1 => {
                "RNA1KCNR/PPPPPPPP/8/8/8/8/pppppppp/rnck1anr w - 1"
            }
            SubVariant::StandardFairy2 => {
                "RGB1KBAR/PPPPPPPP/8/8/8/8/pppppppp/rabk1bgr w - 1"
            }
            SubVariant::StandardPlacement => {
                "8/PPPPPPPP/8/8/8/8/pppppppp/8 w 2R2BQK2r2bqk 1"
            }
            _ => "",
        }
    }

    pub fn starting_stage(&self) -> u8 {
        match self {
            SubVariant::Standard => 2,
            SubVariant::StandardFairy1 => 2,
            SubVariant::StandardFairy2 => 2,
            SubVariant::StandardPlacement => 1,
            _ => 0,
        }
    }

    pub fn parse(&self, variant: Variant, s: &str) -> Option<SubVariant> {
        match variant {
            Variant::Standard => {
                if s == "standard" {
                    return Some(SubVariant::Standard);
                } else if s == "standardPlacement" {
                    return Some(SubVariant::StandardPlacement);
                }
                None
            }
            Variant::StandardFairy => {
                if s == "standardFairy1" {
                    return Some(SubVariant::StandardFairy1);
                } else if s == "standardFairy2" {
                    return Some(SubVariant::StandardFairy2);
                }
                None
            }
            _ => None,
        }
    }

    pub fn index(self) -> usize {
        self as usize
    }

    pub fn valid_index(index: u8) -> bool {
        index < 4
    }
}

impl From<u8> for SubVariant {
    fn from(value: u8) -> Self {
        match value {
            0 => SubVariant::Standard,
            1 => SubVariant::StandardFairy1,
            2 => SubVariant::StandardFairy2,
            3 => SubVariant::StandardPlacement,
            _ => SubVariant::NoSubVariant,
        }
    }
}
