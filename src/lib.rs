use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "chem.pest"]
pub struct ChemParser;

pub mod element;
