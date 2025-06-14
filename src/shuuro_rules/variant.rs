use crate::shuuro_rules::PieceType;

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum Variant {
    Shuuro,
    ShuuroFairy,
    Standard,
    StandardFairy,
    ShuuroMini,
    ShuuroMiniFairy,
}

impl From<u8> for Variant {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Shuuro,
            1 => Self::ShuuroFairy,
            2 => Self::Standard,
            3 => Self::StandardFairy,
            4 => Self::ShuuroMini,
            5 => Self::ShuuroMiniFairy,
            _ => Self::Shuuro,
        }
    }
}

impl From<&String> for Variant {
    fn from(value: &String) -> Self {
        match value.as_str() {
            "shuuro" => Self::Shuuro,
            "shuuroFairy" => Self::ShuuroFairy,
            "shuuroMini" => Self::ShuuroMini,
            "shuuroMiniFairy" => Self::ShuuroMiniFairy,
            "standard" => Self::Standard,
            "standardFairy" => Self::StandardFairy,
            _ => Self::Shuuro,
        }
    }
}

impl Variant {
    pub fn change_variant(&self, variant: &String) -> Self {
        Variant::from(variant)
    }

    pub fn can_select(&self, piece: &PieceType) -> bool {
        if piece == &PieceType::Plinth {
            return false;
        }
        match &self {
            Self::Shuuro | Self::Standard | Self::ShuuroMini => {
                !piece.is_fairy_piece()
            }
            _ => true,
        }
    }

    pub fn start_credit(&self) -> i32 {
        match &self {
            Self::Shuuro => 800,
            Self::ShuuroFairy => 870,
            Self::ShuuroMini => 200,
            Self::ShuuroMiniFairy => 250,
            Self::Standard => 350,
            Self::StandardFairy => 400,
        }
    }

    pub fn min_credit(&self) -> i32 {
        match &self {
            Self::Shuuro | Self::ShuuroFairy => 700,
            Self::ShuuroMini | Self::ShuuroMiniFairy => 150,
            Self::Standard | Self::StandardFairy => 270,
        }
    }
}

impl ToString for Variant {
    fn to_string(&self) -> String {
        match &self {
            Self::Shuuro => String::from("shuuro"),
            Self::ShuuroFairy => String::from("shuuroFairy"),
            Self::ShuuroMini => String::from("shuuroMini"),
            Self::Standard => String::from("standard"),
            Self::StandardFairy => String::from("standardFairy"),
            Variant::ShuuroMiniFairy => String::from("shuuroMiniFairy"),
        }
    }
}
