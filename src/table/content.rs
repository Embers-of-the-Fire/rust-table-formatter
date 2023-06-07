use super::Overflow;

#[derive(Debug, Clone, Default)]
pub enum Content {
    Text(String),
    Splitter,
    #[default]
    None,
}

impl<T: ToString> From<T> for Content {
    fn from(value: T) -> Self {
        Self::Text(value.to_string())
    }
}

/// usize: actual width \
/// String: string without overflow \
/// bool: overflow? \
fn render(width: usize, last: usize, s: &str) -> (usize, String, bool) {
    let mut chars = s.chars();
    let mut cache = String::new();
    let mut cache_b = String::new();
    let mut overflow = false;

    for ch in chars.by_ref().take(width - last) {
        cache.push(ch);
    }
    for ch in chars.by_ref().take(last) {
        cache_b.push(ch);
    }

    if matches!(chars.next(), Some(_)) {
        overflow = true;
    }

    if overflow {
        (width - last, cache, true)
    } else {
        (width, cache + &cache_b, false)
    }
}

impl Content {
    pub fn new(s: impl ToString) -> Self {
        Self::Text(s.to_string())
    }

    pub fn get_width(&self) -> Option<usize> {
        if let Self::Text(ref v) = self {
            Some(v.chars().count())
        } else {
            None
        }
    }

    pub fn render_with_width(&self, width: usize, overflow: Overflow) -> String {
        match self {
            Self::Text(ref v) => match overflow {
                Overflow::Ellipsis => {
                    let (w, s, flag) = render(width, if width < 3 { 1 } else { 3 }, v);
                    if flag {
                        s + if width < 3 { "." } else { "..." }
                    } else {
                        s + " ".repeat(width - w).as_str()
                    }
                }
                Overflow::Hidden => {
                    let (w, s, _) = render(width, 0, v);
                    s + " ".repeat(width - w).as_str()
                }
            },
            Self::Splitter => "─".repeat(width),
            Self::None => " ".repeat(width),
        }
    }
}

#[test]
fn test_render_cell() {
    let cell = Content::new("123123123");
    let t = cell.render_with_width(2, Overflow::Ellipsis);
    assert_eq!("1.", t.as_str());
    let t = cell.render_with_width(2, Overflow::Hidden);
    assert_eq!("12", t.as_str());
    let t = cell.render_with_width(5, Overflow::Ellipsis);
    assert_eq!("12...", t.as_str());
    let t = cell.render_with_width(5, Overflow::Hidden);
    assert_eq!("12312", t.as_str());
    let t = cell.render_with_width(9, Overflow::Ellipsis);
    assert_eq!("123123123", t.as_str());
    let t = cell.render_with_width(9, Overflow::Hidden);
    assert_eq!("123123123", t.as_str());

    let cell = Content::Splitter;
    let t = cell.render_with_width(9, Overflow::Hidden);
    assert_eq!("─────────", t.as_str());

    let cell = Content::None;
    let t = cell.render_with_width(9, Overflow::Hidden);
    assert_eq!("         ", t.as_str());
}
