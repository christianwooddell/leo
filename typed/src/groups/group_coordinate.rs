use crate::common::span::Span;
use leo_ast::values::{
    GroupCoordinate as AstGroupCoordinate,
    Inferred as AstInferred,
    NumberValue as AstNumberValue,
    SignHigh as AstSignHigh,
    SignLow as AstSignLow,
};

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GroupCoordinate {
    Number(String, Span),
    SignHigh(Span),
    SignLow(Span),
    Inferred(Span),
}

impl<'ast> From<AstGroupCoordinate<'ast>> for GroupCoordinate {
    fn from(coordinate: AstGroupCoordinate<'ast>) -> Self {
        match coordinate {
            AstGroupCoordinate::Number(number) => GroupCoordinate::from(number),
            AstGroupCoordinate::SignHigh(sign_high) => GroupCoordinate::from(sign_high),
            AstGroupCoordinate::SignLow(sign_low) => GroupCoordinate::from(sign_low),
            AstGroupCoordinate::Inferred(inferred) => GroupCoordinate::from(inferred),
        }
    }
}

impl fmt::Display for GroupCoordinate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GroupCoordinate::Number(number, _) => write!(f, "{}", number),
            GroupCoordinate::SignHigh(_) => write!(f, "+"),
            GroupCoordinate::SignLow(_) => write!(f, "-"),
            GroupCoordinate::Inferred(_) => write!(f, "_"),
        }
    }
}

impl<'ast> From<AstNumberValue<'ast>> for GroupCoordinate {
    fn from(number: AstNumberValue<'ast>) -> Self {
        let value = number.to_string();
        let span = Span::from(number.span().clone());

        GroupCoordinate::Number(value, span)
    }
}

impl<'ast> From<AstSignHigh<'ast>> for GroupCoordinate {
    fn from(sign: AstSignHigh<'ast>) -> Self {
        GroupCoordinate::SignHigh(Span::from(sign.span))
    }
}

impl<'ast> From<AstSignLow<'ast>> for GroupCoordinate {
    fn from(sign: AstSignLow<'ast>) -> Self {
        GroupCoordinate::SignLow(Span::from(sign.span))
    }
}

impl<'ast> From<AstInferred<'ast>> for GroupCoordinate {
    fn from(sign: AstInferred<'ast>) -> Self {
        GroupCoordinate::Inferred(Span::from(sign.span))
    }
}
