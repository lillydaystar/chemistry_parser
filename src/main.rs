use chemistry_parser::element::PeriodicTable;
use pest::Parser;
use chemistry_parser::{ChemParser, Rule};

fn main() -> anyhow::Result<()> {
    let parser = ChemParser::new();
    let form = parser.parse_formula("Ca5(PO4)3(OH)")?;
    dbg!(form);

    let form1 = parser.parse_formula("H2O")?;
    dbg!(form1);

    let form2 = parser.parse_formula("O2")?;
    dbg!(form2);

    let equation = parser.parse_equation("H + O2 -> H2O")?;
    dbg!(&equation);

    let solved = equation.clone().solve_equation();
    dbg!(&solved);

    let element_parse = ChemParser::parse(Rule::element, "Al")?;
    println!("{:?}", element_parse);

    let formula_parse = ChemParser::parse(Rule::formula, "H2O(SO4)2")?;
    println!("{:?}", formula_parse);

    let equation_parse = ChemParser::parse(Rule::equation, "2H + O2 -> 2H2O")?;
    println!("{:?}", equation_parse);

    let periodic_table =
        PeriodicTable::from_csv("./data/elements.csv").expect("Failed to load periodic table");

    println!("{:?}", periodic_table.get_element("H"));

    Ok(())
}
