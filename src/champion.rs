mod list;

pub(crate) fn get_champion_name_from_id(id: u32) -> Option<&'static str> {
    for pair in list::LIST {
        if pair.0 == id {
            return Some(pair.1);
        }
    }
    None
}

pub(crate) fn get_champion_id_from_name(name: &str) -> Option<u32> {
    for pair in list::LIST {
        if pair.1 == name {
            return Some(pair.0);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_id() {
        let name = get_champion_name_from_id(523).unwrap();
        assert_eq!(name, "Aphelios");
    }

    #[test]
    fn from_name() {
        let id = get_champion_id_from_name("Aphelios").unwrap();
        assert_eq!(id, 523);
    }
}
