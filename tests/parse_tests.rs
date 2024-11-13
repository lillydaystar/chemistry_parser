use chemistry_parser::{ChemParseError, ChemParser};

#[test]
fn test_parse_element_valid() -> anyhow::Result<()> {
    let parser = ChemParser::new();
    let element = parser.parse_element("H")?;
    assert_eq!(element.symbol, "H");
    assert_eq!(element.atomic_mass, 1.008);
    Ok(())
}

#[test]
fn test_parse_element_invalid() -> anyhow::Result<()> {
    let parser = ChemParser::new();
    let result = parser.parse_element("Ha");
    assert!(matches!(result, Err(ChemParseError::InvalidElement(_))));
    Ok(())
}

#[test]
fn test_parse_formula() -> anyhow::Result<()> {
    let parser = ChemParser::new();
    let formula = parser.parse_formula("H2O")?;
    assert_eq!(formula.formula, "H2O");
    assert_eq!(formula.elements["H"], 2);
    assert_eq!(formula.elements["O"], 1);
    assert_eq!(formula.mass, 18.015);
    Ok(())
}

#[test]
fn test_parse_formula_with_groups() -> anyhow::Result<()> {
    let parser = ChemParser::new();
    let formula = parser.parse_formula("Cu2(OH)2CO3")?;
    assert_eq!(formula.elements["Cu"], 2);
    assert_eq!(formula.elements["O"], 5);
    assert_eq!(formula.elements["H"], 2);
    assert_eq!(formula.elements["C"], 1);
    Ok(())
}

#[test]
fn test_parse_formula_invalid_element() -> anyhow::Result<()> {
    let parser = ChemParser::new();
    let result = parser.parse_formula("Yx2");
    assert!(matches!(result, Err(ChemParseError::InvalidElement(_))));
    Ok(())
}

#[test]
fn test_parse_equation_balanced() -> anyhow::Result<()> {
    let parser = ChemParser::new();
    let equation = parser.parse_equation("2H2 + O2 -> 2H2O")?;

    assert_eq!(equation.reactants["H2"], 2);
    assert_eq!(equation.reactants["O2"], 1);
    assert_eq!(equation.products["H2O"], 2);

    assert!(equation.check_equation());

    Ok(())
}

#[test]
fn test_parse_equation_unbalanced() -> anyhow::Result<()> {
    let parser = ChemParser::new();
    let equation = parser.parse_equation("H2 + O2 -> H2O")?;

    assert!(!equation.check_equation());
    Ok(())
}
