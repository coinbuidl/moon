fn sanitize_value(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    let mut prev_sep = false;
    for ch in value.chars() {
        if ch.is_ascii_whitespace() {
            if !out.is_empty() && !prev_sep {
                out.push('_');
                prev_sep = true;
            }
        } else if ch.is_ascii_graphic() {
            out.push(ch);
            prev_sep = false;
        }
    }
    let trimmed = out.trim_matches('_');
    if trimmed.is_empty() {
        "na".to_string()
    } else {
        trimmed.to_string()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct WarnEvent<'a> {
    pub code: &'a str,
    pub stage: &'a str,
    pub action: &'a str,
    pub session: &'a str,
    pub archive: &'a str,
    pub source: &'a str,
    pub retry: &'a str,
    pub reason: &'a str,
    pub err: &'a str,
}

pub fn emit(event: WarnEvent<'_>) {
    eprintln!(
        "MOON_WARN code={} stage={} action={} session={} archive={} source={} retry={} reason={} err={}",
        sanitize_value(event.code),
        sanitize_value(event.stage),
        sanitize_value(event.action),
        sanitize_value(event.session),
        sanitize_value(event.archive),
        sanitize_value(event.source),
        sanitize_value(event.retry),
        sanitize_value(event.reason),
        sanitize_value(event.err),
    );
}

#[cfg(test)]
mod tests {
    use super::sanitize_value;

    #[test]
    fn sanitize_value_rewrites_whitespace() {
        assert_eq!(sanitize_value("a b\tc"), "a_b_c");
    }

    #[test]
    fn sanitize_value_falls_back_for_empty() {
        assert_eq!(sanitize_value("   "), "na");
    }
}
