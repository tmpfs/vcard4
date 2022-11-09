//! Iterator for parsing vCards.
use crate::{
    parser::{Token, VcardParser},
    Error, Result, Vcard,
};
use std::ops::Range;

/// Iterator for parsing vCards.
pub struct VcardIterator<'s> {
    parser: VcardParser<'s>,
    offset: usize,
}

impl<'s> VcardIterator<'s> {
    /// Create a new iterator.
    pub fn new(source: &'s str, strict: bool) -> Self {
        Self {
            parser: VcardParser::new(source, strict),
            offset: 0,
        }
    }

    /// Parse the next vCard.
    fn parse_next(&self, offset: usize) -> Result<(Vcard, Range<usize>)> {
        let mut lex = self.parser.lexer();
        lex.bump(offset);
        while let Some(first) = lex.next() {
            if first == Token::NewLine {
                continue;
            } else {
                return self.parser.parse_one(&mut lex, Some(first));
            }
        }
        Err(Error::TokenExpected)
    }
}

impl<'s> Iterator for VcardIterator<'s> {
    type Item = Result<Vcard>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.offset >= self.parser.source.len() {
            return None;
        }
        match self.parse_next(self.offset) {
            Ok((card, span)) => {
                self.offset = span.end;
                Some(Ok(card))
            }
            Err(e) => Some(Err(e)),
        }
    }
}
