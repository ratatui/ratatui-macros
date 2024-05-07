/// A macro for creating a [`Line`] using vec! syntax.
///
/// `line!` is similar to the [`vec!`] macro, but it returns a [`Line`] instead of a `Vec`.
///
/// # Examples
///
/// ```rust
/// # use ratatui::prelude::*;
/// use ratatui_macros::{line, raw};
///
/// let line = line!["hello", "world"];
/// let line = line![raw!("hello {}", "world"), raw!("goodbye {}", "world")];
/// let line = line!["hello".red(), "world".red().bold()];
/// let line = line!["hello"; 2];
/// ```
///
/// [`Line`]: crate::text::Line
#[macro_export]
macro_rules! line {
    () => {
        ratatui::text::Line::default()
    };
    ($span:expr; $n:expr) => {
      ratatui::text::Line::from(vec![$span.into(); $n])
    };
    ($($span:expr),+ $(,)?) => {{
        ratatui::text::Line::from(vec![
        $(
            $span.into(),
        )+
        ])
    }};
}

#[cfg(test)]
mod tests {
    use ratatui::prelude::*;

    #[test]
    fn line() {
        // literal
        let line = line!["hello", "world"];
        assert_eq!(line, Line::from(vec!["hello".into(), "world".into()]));

        // raw instead line
        let line = line![crate::raw!("hello"), "world"];
        assert_eq!(line, Line::from(vec!["hello".into(), "world".into()]));

        // vec count syntax
        let line = line!["hello"; 2];
        assert_eq!(line, Line::from(vec!["hello".into(), "hello".into()]));

        // vec count syntax with span
        let line = line![Span::raw("hello"); 2];
        assert_eq!(line, Line::from(vec!["hello".into(), "hello".into()]));
    }
}
