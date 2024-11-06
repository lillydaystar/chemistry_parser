use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "chem.pest"]
pub struct ChemParser;

fn main() {
    let element_parse = ChemParser::parse(Rule::element, "Al");
    println!("{:?}", element_parse);
}
