use crate::shuuro_rules::PieceType;

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum Variant {
    Shuuro,
    ShuuroFairy,
    ShuuroMini,
    Standard,
    StandardFairy,
}

impl From<&String> for Variant {
    fn from(value: &String) -> Self {
        match value.as_str() {
            "shuuro" => Self::Shuuro,
            "shuuroFairy" => Self::ShuuroFairy,
            "shuuroMini" => Self::ShuuroMini,
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
            Self::Shuuro | Self::Standard => !piece.is_fairy_piece(),
            _ => true,
        }
    }

    pub fn start_credit(&self) -> i32 {
        match &self {
            Self::Shuuro => 800,
            Self::ShuuroFairy => 870,
            Self::ShuuroMini => 200,
            Self::Standard => 350,
            Self::StandardFairy => 400,
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
        }
    }
}
