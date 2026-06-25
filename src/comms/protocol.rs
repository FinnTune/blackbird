pub const NICK_PREFIX: &str = "NICK:";

pub fn nickname_handshake(name: &str) -> String {
    format!("{NICK_PREFIX}{name}")
}

pub fn parse_nickname_handshake(line: &str) -> Option<&str> {
    line.strip_prefix(NICK_PREFIX)
}

pub fn is_valid_nickname(name: &str) -> bool {
    let name = name.trim();
    !name.is_empty()
        && name.len() <= 32
        && !name.contains(char::is_whitespace)
        && !name.contains('[')
        && !name.contains(']')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_nickname_handshake_extracts_name() {
        assert_eq!(parse_nickname_handshake("NICK:alice"), Some("alice"));
        assert_eq!(parse_nickname_handshake("hello"), None);
    }

    #[test]
    fn is_valid_nickname_rejects_invalid_names() {
        assert!(is_valid_nickname("alice"));
        assert!(!is_valid_nickname(""));
        assert!(!is_valid_nickname(" "));
        assert!(!is_valid_nickname("alice bob"));
        assert!(!is_valid_nickname("[alice]"));
    }
}
