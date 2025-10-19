use cookietyper_core::FacilityKey;

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Event {
    EarnCookies(i32),
    ShowCookiesAmount,
    ShowCps,
    PurchaseFacility(FacilityKey),
    InvalidCommand,
}

impl From<String> for Event {
    fn from(s: String) -> Self {
        let s = s.trim();

        if !s.starts_with("/") {
            let s_num = s.len();
            return Event::EarnCookies(s_num as i32);
        }

        match s {
            "/cc" => Event::ShowCookiesAmount,
            "/cps" => Event::ShowCps,
            s if s.starts_with("/f buy") => Event::PurchaseFacility(FacilityKey::Cursor),
            _ => Event::InvalidCommand,
        }
    }
}

#[cfg(test)]
mod test {
    use super::Event;

    #[test]
    fn test_from_str() {
        assert_eq!(Event::from("12345".to_string()), Event::EarnCookies(5));
        assert_eq!(Event::from("123".to_string()), Event::EarnCookies(3));
        assert_ne!(Event::from("1".to_string()), Event::EarnCookies(3));
        assert_ne!(Event::from("123".to_string()), Event::EarnCookies(5));

        assert_eq!(Event::from("/cc".to_string()), Event::ShowCookiesAmount);
        assert_eq!(Event::from("/cps".to_string()), Event::ShowCps);
        assert_eq!(
            Event::from("/invalidcmd".to_string()),
            Event::InvalidCommand
        );
    }
}
