use chemistry_parser::{ChemParser, Rule};
use pest::Parser;

#[test]
fn test_successful_element_parse() -> anyhow::Result<()> {
    assert!(ChemParser::parse(Rule::element, "H").is_ok());
    assert!(ChemParser::parse(Rule::element, "Li").is_ok());

    Ok(())
}

#[test]
fn test_unsuccessful_element_parse() -> anyhow::Result<()> {
    assert!(ChemParser::parse(Rule::element, "2").is_err());
    assert!(ChemParser::parse(Rule::element, "h").is_err());

    Ok(())
}

#[test]
fn test_correct_element_parse() -> anyhow::Result<()> {
    let successful_parse = ChemParser::parse(Rule::element, "Na");

    assert!(successful_parse.is_ok());

    let pair = successful_parse?.next().unwrap();

    assert_eq!(pair.as_rule(), Rule::element);
    assert_eq!(pair.as_str(), "Na");

    Ok(())
}

#[test]
fn test_successful_index_parse() -> anyhow::Result<()> {
    assert!(ChemParser::parse(Rule::index, "2").is_ok());
    assert!(ChemParser::parse(Rule::index, "12").is_ok());

    Ok(())
}

#[test]
fn test_unsuccessful_index_parse() -> anyhow::Result<()> {
    assert!(ChemParser::parse(Rule::index, "0").is_err());
    assert!(ChemParser::parse(Rule::index, "H").is_err());

    Ok(())
}

#[test]
fn test_successful_formula_parse() -> anyhow::Result<()> {
    assert!(ChemParser::parse(Rule::formula, "H2O").is_ok());
    assert!(ChemParser::parse(Rule::formula, "NaCl").is_ok());

    Ok(())
}

#[test]
fn test_correct_formula_parse() -> anyhow::Result<()> {
    let successful_parse = ChemParser::parse(Rule::formula, "CH2O");

    assert!(successful_parse.is_ok());

    let mut pairs = successful_parse?;
    let formula = pairs.next().unwrap();

    assert_eq!(formula.as_rule(), Rule::formula);
    assert_eq!(formula.as_str(), "CH2O");

    let mut inner = formula.into_inner();
    let elem1 = inner.next().unwrap();
    assert_eq!(elem1.as_rule(), Rule::element);
    assert_eq!(elem1.as_str(), "C");

    let elem2 = inner.next().unwrap();
    assert_eq!(elem2.as_rule(), Rule::element);
    assert_eq!(elem2.as_str(), "H");

    let index = inner.next().unwrap();
    assert_eq!(index.as_rule(), Rule::index);
    assert_eq!(index.as_str(), "2");

    let elem3 = inner.next().unwrap();
    assert_eq!(elem3.as_rule(), Rule::element);
    assert_eq!(elem3.as_str(), "O");

    Ok(())
}

#[test]
fn test_invalid_formula() -> anyhow::Result<()> {
    assert!(ChemParser::parse(Rule::formula, "2O2").is_err());
    assert!(ChemParser::parse(Rule::formula, "h2o").is_err());

    Ok(())
}

#[test]
fn test_successful_group_parse() -> anyhow::Result<()> {
    assert!(ChemParser::parse(Rule::group, "(H2O)").is_ok());
    assert!(ChemParser::parse(Rule::group, "(CO2)4").is_ok());

    Ok(())
}

#[test]
fn test_unsuccessful_group_parse() -> anyhow::Result<()> {
    assert!(ChemParser::parse(Rule::group, "((H2O)").is_err());
    assert!(ChemParser::parse(Rule::group, "CO2 4").is_err());

    Ok(())
}

#[test]
fn test_successful_formula_with_group_parse() -> anyhow::Result<()> {
    assert!(ChemParser::parse(Rule::formula, "Al2(Si2O5)(OH)4").is_ok());
    assert!(ChemParser::parse(Rule::formula, "Ca5(PO4)3(OH)").is_ok());

    Ok(())
}

#[test]
fn test_successful_equation_parse() -> anyhow::Result<()> {
    assert!(ChemParser::parse(Rule::equation, "2H2 + O2 -> 2H2O").is_ok());
    assert!(ChemParser::parse(Rule::equation, "2HCl+2Na->2NaCl+H2").is_ok());

    Ok(())
}

#[test]
fn test_unsuccessful_equation_parse() -> anyhow::Result<()> {
    assert!(ChemParser::parse(Rule::equation, "2 + O2 -> 2H2O").is_err());
    assert!(ChemParser::parse(Rule::equation, "2 HCl + 2Na -> 2NaCl + H2").is_err());

    Ok(())
}

#[test]
fn test_successful_reactants_and_products_parse() -> anyhow::Result<()> {
    assert!(ChemParser::parse(Rule::reactants, "2H2O").is_ok());
    assert!(ChemParser::parse(Rule::products, "2HCl+2Na").is_ok());
    assert!(ChemParser::parse(Rule::reactants, "2H2O").is_ok());
    assert!(ChemParser::parse(Rule::products, "2HCl+2Na").is_ok());

    Ok(())
}

#[test]
fn test_unsuccessful_reactants_and_products_parse() -> anyhow::Result<()> {
    assert!(ChemParser::parse(Rule::reactants, "2 + O2").is_err());
    assert!(ChemParser::parse(Rule::products, "2cl + 2h2").is_err());
    assert!(ChemParser::parse(Rule::reactants, "na2").is_err());
    assert!(ChemParser::parse(Rule::products, "2 HCl").is_err());

    Ok(())
}

#[test]
fn test_successful_coefficient_parse() -> anyhow::Result<()> {
    assert!(ChemParser::parse(Rule::coefficient, "4").is_ok());
    assert!(ChemParser::parse(Rule::coefficient, "10").is_ok());

    Ok(())
}

#[test]
fn test_unsuccessful_coefficient_parse() -> anyhow::Result<()> {
    assert!(ChemParser::parse(Rule::coefficient, "0").is_err());
    assert!(ChemParser::parse(Rule::coefficient, "02").is_err());

    Ok(())
}

#[test]
fn test_successful_whitespace_parse() -> anyhow::Result<()> {
    assert!(ChemParser::parse(Rule::WS, " ").is_ok());

    Ok(())
}

#[test]
fn test_unsuccessful_whitespace_parse() -> anyhow::Result<()> {
    assert!(ChemParser::parse(Rule::WS, "_").is_err());

    Ok(())
}
