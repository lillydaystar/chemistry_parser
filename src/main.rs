use pest::Parser;
use chemistry_parser::{ChemParser, Rule};
use chemistry_parser::element::PeriodicTable;

fn main() {
    let element_parse = ChemParser::parse(Rule::element, "Al");
    println!("{:?}", element_parse);

    let periodic_table = PeriodicTable::from_csv("./data/elements.csv")
        .expect("Failed to load periodic table");

    println!("{:?}", periodic_table.get_element("H"));
}
