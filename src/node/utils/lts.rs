use crate::node::types::LTS;

pub enum LtsAliases {
    ARGON,
    BORON,
    CARBON,
    DUBNIUM,
    ERBIUM,
    FERMIUM,
    GALLIUM,
    HYDROGEN,
    IRON,
}

impl LtsAliases {
    pub fn get_alias(&self) -> LTS {
        match self {
            LtsAliases::ARGON => LTS {
                alias: "Argon".to_string(),
                version: "4.9.1".to_string(),
            },
            LtsAliases::BORON => LTS {
                alias: "Boron".to_string(),
                version: "6.17.1".to_string(),
            },
            LtsAliases::CARBON => LTS {
                alias: "Carbon".to_string(),
                version: "8.17.0".to_string(),
            },
            LtsAliases::DUBNIUM => LTS {
                alias: "Dubnium".to_string(),
                version: "10.24.1".to_string(),
            },
            LtsAliases::ERBIUM => LTS {
                alias: "Erbium".to_string(),
                version: "12.22.12".to_string(),
            },
            LtsAliases::FERMIUM => LTS {
                alias: "Fermium".to_string(),
                version: "14.21.3".to_string(),
            },
            LtsAliases::GALLIUM => LTS {
                alias: "Gallium".to_string(),
                version: "16.20.2".to_string(),
            },
            LtsAliases::HYDROGEN => LTS {
                alias: "Hydrogen".to_string(),
                version: "18.20.3".to_string(),
            },
            LtsAliases::IRON => LTS {
                alias: "Iron".to_string(),
                version: "20.13.1".to_string(),
            },
        }
    }
}
