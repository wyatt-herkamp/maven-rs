use std::{
    any::type_name,
    fmt::{Debug, Display},
};

use winnow::{
    combinator::eof,
    error::ParserError,
    stream::{Stream, StreamIsPartial},
    Parser,
};

/// Extends [Parser] with helper methods.
pub(crate) trait ParserExt<I, O, E>: Parser<I, O, E> {
    /// Parse all of `input`, generating `O` from it.
    ///
    /// Returns [ParseErrorExt] instead of [winnow::error::ParseError],
    /// functionally equivalent to [winnow::Parser::parse].
    #[inline]
    fn parse_ext(&mut self, mut input: I) -> Result<O, ParseErrorExt<I, E>>
    where
        Self: core::marker::Sized,
        I: Stream + StreamIsPartial,
        E: ParserError<I>,
    {
        debug_assert!(
            !I::is_partial_supported(),
            "partial streams need to handle `ErrMode::Incomplete`"
        );

        let start = input.checkpoint();
        let (o, _) = (self.by_ref(), eof).parse_next(&mut input).map_err(|e| {
            let inner = e
                .into_inner()
                .expect("complete parsers should not report `ErrMode::Incomplete(_)`");
            ParseErrorExt::new_rewind(input, start, inner)
        })?;
        Ok(o)
    }
}

impl<P, I, O, E> ParserExt<I, O, E> for P where P: Parser<I, O, E> {}

/// [`winnow::error::ParseError`] but with fields public for better ergonomics.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParseErrorExt<I, E> {
    /// The [`Stream`] at the initial location when parsing started.
    pub input: I,

    /// The location in [`ParseErrorExt::input`] where parsing failed.
    ///
    /// **Note:** This is an offset, not an index, and may point to the end of input
    /// (`input.len()`).
    pub offset: usize,

    /// The original [`ParserError`].
    pub inner: E,
}

impl<I, E> ParseErrorExt<I, E> {
    pub fn new(input: I, offset: usize, inner: E) -> Self {
        Self {
            input,
            offset,
            inner,
        }
    }

    pub fn map<U, F: FnOnce(&I) -> U>(self, op: F) -> ParseErrorExt<U, E> {
        ParseErrorExt::new(op(&self.input), self.offset, self.inner)
    }
}

impl<I: Stream, E> ParseErrorExt<I, E> {
    pub fn new_rewind(mut input: I, start: I::Checkpoint, inner: E) -> Self {
        let offset = input.offset_from(&start);
        input.reset(&start);
        Self::new(input, offset, inner)
    }
}

impl<I, E: Display> Display for ParseErrorExt<I, E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Parsing of `{}` stopped at offset {}, context: {}",
            type_name::<I>(),
            self.offset,
            self.inner
        )
    }
}

impl<I, E: std::error::Error> std::error::Error for ParseErrorExt<I, E>
where
    I: Debug,
{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.inner.source()
    }
}
