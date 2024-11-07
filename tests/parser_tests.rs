use pest::Parser;
use chemistry_parser::{ChemParser, Rule};

#[test]
fn test_successful_element_parse() {
    assert!(ChemParser::parse(Rule::element, "H").is_ok());
    assert!(ChemParser::parse(Rule::element, "Li").is_ok());
}

#[test]
fn test_unsuccessful_element_parse() {
    assert!(ChemParser::parse(Rule::element, "2").is_err());

    assert!(ChemParser::parse(Rule::element, "h").is_err());
}

#[test]
fn test_correct_element_parse() {
    let successful_parse = ChemParser::parse(Rule::element, "Na");

    assert!(successful_parse.is_ok());

    let pair = successful_parse.unwrap().next().unwrap();

    assert_eq!(pair.as_rule(), Rule::element);
    assert_eq!(pair.as_str(), "Na");
}

#[test]
fn test_successful_formula_parse() {
    assert!(ChemParser::parse(Rule::formula, "H2O").is_ok());
    assert!(ChemParser::parse(Rule::formula, "NaCl").is_ok());
}

#[test]
fn test_correct_formula_parse() {
    let successful_parse = ChemParser::parse(Rule::formula, "CH2O");

    assert!(successful_parse.is_ok());

    let mut pairs = successful_parse.unwrap();
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
}


#[test]
fn test_invalid_formula() {
    assert!(ChemParser::parse(Rule::formula, "2O2").is_err());

    assert!(ChemParser::parse(Rule::formula, "h2o").is_err());
}

#[test]
fn test_successful_group_parse() {
    assert!(ChemParser::parse(Rule::group, "(H2O)").is_ok());
    assert!(ChemParser::parse(Rule::group, "(CO2)4").is_ok());
}

#[test]
fn test_unsuccessful_group_parse() {
    assert!(ChemParser::parse(Rule::group, "((H2O)").is_err());
    assert!(ChemParser::parse(Rule::group, "CO2 4").is_err());
}

#[test]
fn test_successful_formula_with_group_parse() {
    assert!(ChemParser::parse(Rule::formula, "Al2(Si2O5)(OH)4").is_ok());
    assert!(ChemParser::parse(Rule::formula, "Ca5(PO4)3(OH)").is_ok());
}

#[test]
fn test_successful_equation_parse() {
    assert!(ChemParser::parse(Rule::equation, "2H2 + O2 -> 2H2O").is_ok());
    assert!(ChemParser::parse(Rule::equation, "2HCl + 2Na -> 2NaCl + H2").is_ok());
}


#[test]
fn test_unsuccessful_equation_parse() {
    assert!(ChemParser::parse(Rule::equation, "2 + O2 -> 2H2O").is_err());
    assert!(ChemParser::parse(Rule::equation, "2HCl + 2Na = 2NaCl + H2").is_err());
}