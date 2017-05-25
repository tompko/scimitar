use std::fmt::{Display, Formatter, Result};

pub static DEFAULT_MODEL_PRIORITY: [Model; 5] = [Model::Dmg, Model::Dmg0, Model::Mgb, Model::Sgb2, Model::Sgb];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Model {
    Dmg0,
    Dmg,
    Mgb,
    Sgb,
    Sgb2,
}

impl Display for Model {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", match *self {
            Model::Dmg0 => "DMG0",
            Model::Dmg => "DMG",
            Model::Mgb => "MGB",
            Model::Sgb => "SGB",
            Model::Sgb2 => "SGB2",
        })
    }
}

impl From<&'static str> for Model {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_ref() {
            "dmg0"=> Model::Dmg0,
            "dmg"=> Model::Dmg,
            "mgb"=> Model::Mgb,
            "sgb"=> Model::Sgb,
            "sgb2"=> Model::Sgb2,
            _ => panic!("Unrecognised model type {}", s),
        }
    }
}

impl Model {
    pub fn bootrom_name(&self) -> &'static str {
        match *self {
            Model::Dmg0 => "dmg0_rom.bin",
            Model::Dmg => "dmg_boot.bin",
            Model::Mgb => "mgb_boot.bin",
            Model::Sgb => "sgb_boot.bin",
            Model::Sgb2 => "sgb2_boot.bin",
        }
    }
}
