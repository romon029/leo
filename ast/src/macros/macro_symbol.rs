use crate::ast::Rule;

use pest_ast::FromPest;
use serde::Serialize;
use std::fmt;

#[derive(Clone, Debug, FromPest, PartialEq, Serialize)]
#[pest_ast(rule(Rule::macro_symbol))]
pub struct MacroSymbol {}

impl fmt::Display for MacroSymbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "!")
    }
}