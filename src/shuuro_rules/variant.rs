use crate::shuuro_rules::PieceType;

#[derive(Clone, Copy, Debug)]
pub enum Variant {
    Shuuro,
    ShuuroFairy,
    ShuuroMini,
    Standard,
}

impl From<&String> for Variant {
    fn from(value: &String) -> Self {
        match value.as_str() {
            "shuuro" => Self::Shuuro,
            "shuuroFairy" => Self::ShuuroFairy,
            "shuuroMini" => Self::ShuuroMini,
            _ => Self::Standard,
        }
    }
}

impl Variant {
    pub fn change_variant(&self, variant: &String) -> Self {
        Variant::from(variant)
    }

    pub fn can_buy(&self, piece: &PieceType) -> bool {
        if piece == &PieceType::Plinth {
            return false;
        } else if let Self::Shuuro = &self {
            if piece.is_fairy_piece() {
                return false;
            }
        }
        true
    }

    pub fn start_credit(&self) -> i32 {
        match &self {
            Self::Shuuro => 800,
            Self::ShuuroFairy => 870,
            Self::ShuuroMini => 200,
            Self::Standard => 0,
        }
    }
}

impl ToString for Variant {
    fn to_string(&self) -> String {
        match &self {
            Self::Shuuro => String::from("shuuro"),
            Self::ShuuroFairy => String::from("shuuroFairy"),
            Self::ShuuroMini => String::from("shuuroMini"),
            Self::Standard => String::from("shuuroStandard"),
        }
    }
}
