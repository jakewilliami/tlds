#[derive(Debug, Clone, Copy)]
pub struct Tld {
    pub domain: &'static str,
    pub r#type: Type,
    pub manager: &'static str,
    pub country: &'static str,
}

include!(concat!(env!("OUT_DIR"), "/map.rs"));

pub fn lookup(tld: &str) -> Option<&'static Tld> {
    let mut key = tld.to_ascii_lowercase();
    if !key.starts_with('.') {
        key.insert(0, '.');
    }
    TLD_MAP.get(key.as_str())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn looks_up_with_and_without_dot() {
        let with_dot = lookup(".nr").expect(".nr should be present");
        let without_dot = lookup("nr").expect("nr should be present");

        assert_eq!(with_dot.domain, without_dot.domain);
        assert_eq!(with_dot.country, "Nauru");
        assert_eq!(with_dot.r#type, Type::CountryCode);
    }

    #[test]
    fn is_case_insensitive() {
        let upper = lookup("NR").expect("NR should be present");
        assert_eq!(upper.country, "Nauru");
    }

    #[test]
    fn unknown_tld_returns_none() {
        assert!(lookup(".doesnotexist123456789").is_none());
        assert!(lookup("doesnotexist123456789").is_none());
    }
}
