use super::Overflow;

/// Any possible content in a cell.
/// 
/// - Text(String): pure text.
/// - Splitter: horizontal splitter. Looks like "━".
/// - None: empty content.
/// 
/// ```rust
/// # use table_formatter::table::Content;
/// // You can use a string to create one.
/// assert_eq!(<&str as Into<Content>>::into("123").get_content(), Content::Text("123".to_string()).get_content());
/// assert_eq!(Content::new("123").get_content(), Content::Text("123".to_string()).get_content());
/// ```
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

    let mut len = 0;
    for ch in chars.by_ref().take(width - last) {
        cache.push(ch);
        len += 1;
    }
    for ch in chars.by_ref().take(last) {
        cache_b.push(ch);
        len += 1;
    }

    if matches!(chars.next(), Some(_)) {
        overflow = true;
    }

    if overflow {
        (width - last, cache, true)
    } else {
        (len, cache + &cache_b, false)
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

    pub fn have_default_padding(&self) -> bool {
        matches!(self, Content::Text(_))
    }

    pub(crate) fn render_with_width(&self, width: usize, overflow: Overflow) -> (String, usize) {
        match self {
            Self::Text(ref v) => match overflow {
                Overflow::Ellipsis => {
                    let (w, s, flag) = render(width, if width < 3 { 1 } else { 3 }, v);
                    if flag {
                        (s + if width < 3 { "." } else { "..." }, width)
                    } else {
                        (s, w)
                    }
                }
                Overflow::Hidden => {
                    let (w, s, _) = render(width, 0, v);
                    (s, w)
                }
            },
            Self::Splitter => ("━".repeat(width), width),
            Self::None => (" ".repeat(width), width),
        }
    }

    pub fn get_content(&self) -> Option<&String> {
        match self {
            Self::Text(ref t) => Some(t),
            _ => None
        }
    }
}

#[test]
fn test_render_cell() {
    let cell = Content::new("123123123");
    let (t, w) = cell.render_with_width(2, Overflow::Ellipsis);
    assert_eq!("1.", t.as_str());
    assert_eq!(2, w);
    let (t, w) = cell.render_with_width(2, Overflow::Hidden);
    assert_eq!("12", t.as_str());
    assert_eq!(2, w);
    let (t, w) = cell.render_with_width(5, Overflow::Ellipsis);
    assert_eq!("12...", t.as_str());
    assert_eq!(5, w);
    let (t, w) = cell.render_with_width(5, Overflow::Hidden);
    assert_eq!("12312", t.as_str());
    assert_eq!(5, w);
    let (t, w) = cell.render_with_width(9, Overflow::Ellipsis);
    assert_eq!("123123123", t.as_str());
    assert_eq!(9, w);
    let (t, w) = cell.render_with_width(9, Overflow::Hidden);
    assert_eq!("123123123", t.as_str());
    assert_eq!(9, w);
    let (t, w) = cell.render_with_width(19, Overflow::Ellipsis);
    assert_eq!("123123123", t.as_str());
    assert_eq!(9, w);
    let (t, w) = cell.render_with_width(19, Overflow::Hidden);
    assert_eq!("123123123", t.as_str());
    assert_eq!(9, w);

    let cell = Content::Splitter;
    let (t, w) = cell.render_with_width(9, Overflow::Hidden);
    assert_eq!("━━━━━━━━━", t.as_str());
    assert_eq!(9, w);

    let cell = Content::None;
    let (t, w) = cell.render_with_width(9, Overflow::Hidden);
    assert_eq!("         ", t.as_str());
    assert_eq!(9, w);
}
