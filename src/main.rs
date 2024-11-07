use chemistry_parser::element::PeriodicTable;
use chemistry_parser::{ChemParser, Rule};
use pest::Parser;

fn main() -> anyhow::Result<()> {
    let element_parse = ChemParser::parse(Rule::element, "Al")?;
    println!("{:?}", element_parse);

    let formula_parse = ChemParser::parse(Rule::formula, "H2O")?;
    println!("{:?}", formula_parse);

    let equation_parse = ChemParser::parse(Rule::equation, "2H + O2 -> 2H2O")?;
    println!("{:?}", equation_parse);

    let periodic_table =
        PeriodicTable::from_csv("./data/elements.csv").expect("Failed to load periodic table");

    println!("{:?}", periodic_table.get_element("H"));

    Ok(())
}
