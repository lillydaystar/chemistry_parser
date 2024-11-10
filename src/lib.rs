pub mod element;

use crate::element::{Element, Equation, Formula, PeriodicTable};
use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ChemParseError {
    #[error("Invalid element symbol: {0}")]
    InvalidElement(String),

    #[error("Invalid chemical formula: {0}")]
    InvalidFormula(String),

    #[error("Failed to parse {0}: {1}")]
    ParsingError(String, String),

    #[error("Invalid index format: {0}")]
    InvalidIndexFormat(String),

    #[error("Invalid coefficient format: {0}")]
    InvalidCoefficientFormat(String),
}

#[derive(Parser)]
#[grammar = "chem.pest"]
pub struct ChemParser {
    periodic_table: PeriodicTable,
}

impl ChemParser {
    pub fn new() -> Self {
        ChemParser {
            periodic_table: PeriodicTable::from_csv("./data/elements.csv")
                .expect("Failed to parse periodic table"),
        }
    }

    pub fn get_table(&self) -> &PeriodicTable {
        &self.periodic_table
    }

    pub fn parse_element(&self, element: &str) -> Result<&Element, ChemParseError> {
        let mut element_parse = ChemParser::parse(Rule::element, element).map_err(|_| {
            ChemParseError::ParsingError(String::from("element"), String::from(element))
        })?;

        let element_symbol = element_parse.next().unwrap().as_str();

        if !self.validate_element(element_symbol) {
            return Err(ChemParseError::InvalidElement(String::from(element_symbol)));
        }

        Ok(self.get_table().get_element(element_symbol).unwrap())
    }

    pub fn parse_formula(&self, formula: &str) -> Result<Formula, ChemParseError> {
        let mut formula_parse = ChemParser::parse(Rule::formula, formula).map_err(|_| {
            ChemParseError::ParsingError(String::from("formula"), String::from(formula))
        })?;

        let mut inside_pairs = formula_parse.next().unwrap();

        let mut formula_struct = Formula::new(inside_pairs.as_str());

        self.process_pairs(&mut formula_struct.elements, &mut inside_pairs, 1)?;

        formula_struct.mass = formula_struct
            .elements
            .iter()
            .fold(0.0, |acc, (symbol, count)| {
                let element = self.get_table().get_element(symbol).unwrap();
                acc + (element.atomic_mass * *count as f64)
            });

        Ok(formula_struct)
    }

    fn process_pairs(
        &self,
        elements: &mut HashMap<String, u8>,
        pairs: &mut Pair<Rule>,
        multiplier: u8,
    ) -> Result<(), ChemParseError> {
        let mut prev_elem: Option<String> = None;

        for (pair_id, pair) in pairs.clone().into_inner().enumerate() {
            match pair.as_rule() {
                Rule::element => {
                    let symbol = pair.as_str().to_string();

                    if !self.validate_element(&symbol) {
                        return Err(ChemParseError::InvalidElement(symbol));
                    }

                    if prev_elem.is_some() {
                        let prev_symbol = prev_elem.unwrap().clone();
                        *elements.entry(prev_symbol).or_insert(0) += multiplier;
                    }
                    prev_elem = Some(symbol);
                }
                Rule::group => {
                    if prev_elem.is_some() {
                        let prev_symbol = prev_elem.unwrap().clone();
                        *elements.entry(prev_symbol).or_insert(0) += multiplier;
                        prev_elem = None;
                    }
                    let mut inner_pairs = pair.clone().into_inner().next().unwrap();
                    let mut group_multiplier = 1;
                    let pairs_vec: Vec<Pair<Rule>> = pairs.clone().into_inner().collect();
                    if pairs_vec.len() > pair_id + 1 {
                        let next_pair = &pairs_vec[pair_id + 1];
                        if next_pair.as_rule() == Rule::index {
                            group_multiplier = next_pair.as_str().parse::<u8>().unwrap();
                        }
                    }
                    self.process_pairs(elements, &mut inner_pairs, multiplier * group_multiplier)?;
                }
                Rule::index => {
                    if prev_elem.is_some() {
                        let index = pair.as_str().parse::<u8>().map_err(|_| {
                            ChemParseError::InvalidIndexFormat(pair.as_str().to_string())
                        })?;
                        let symbol = prev_elem.unwrap().clone();
                        *elements.entry(symbol).or_insert(0) += index * multiplier;
                    }
                    prev_elem = None;
                }
                _ => {}
            }
        }
        if prev_elem.is_some() {
            let prev_symbol = prev_elem.unwrap().clone();
            *elements.entry(prev_symbol).or_insert(0) += multiplier;
        }

        Ok(())
    }

    pub fn parse_equation(&self, equation: &str) -> Result<Equation, ChemParseError> {
        let mut equation_parse = ChemParser::parse(Rule::equation, equation).map_err(|_| {
            ChemParseError::ParsingError(String::from("equation"), String::from(equation))
        })?;

        let mut reactants = HashMap::new();
        let mut products = HashMap::new();
        let mut reactants_formulas = HashMap::new();
        let mut products_formulas = HashMap::new();

        let mut parts = equation_parse.next().unwrap().into_inner();
        let reactant_part = parts.next().unwrap();
        let product_part = parts.next().unwrap();

        self.process_side(&mut reactants, &mut reactants_formulas, &reactant_part)?;
        self.process_side(&mut products, &mut products_formulas, &product_part)?;

        Ok(Equation::new(
            String::from(equation),
            reactants,
            products,
            reactants_formulas,
            products_formulas,
        ))
    }

    fn process_side(
        &self,
        side: &mut HashMap<String, u8>,
        formulas: &mut HashMap<String, Formula>,
        side_part: &Pair<Rule>,
    ) -> Result<(), ChemParseError> {
        let mut prev_coefficient = 1;
        for compound in side_part.clone().into_inner() {
            match compound.as_rule() {
                Rule::coefficient => {
                    let coefficient: u8 = compound.as_str().parse().map_err(|_| {
                        ChemParseError::InvalidCoefficientFormat(compound.as_str().to_string())
                    })?;
                    prev_coefficient = coefficient;
                }
                Rule::formula => {
                    let formula_part = compound.clone();
                    let formula = formula_part.as_str().to_string();
                    let formula_struct = self.parse_formula(formula_part.as_str())?;

                    side.insert(formula.clone(), prev_coefficient);
                    formulas.insert(formula, formula_struct);
                    prev_coefficient = 1;
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn validate_element(&self, element: &str) -> bool {
        self.periodic_table.get_element(element).is_some()
    }
}

impl Default for ChemParser {
    fn default() -> Self {
        Self::new()
    }
}

