//! # Chemical units Module
//!
//! This module defines structs that represent chemical elements, formulas, and equations

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::Display;

/// Represents a chemical element with its properties from periodic table.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Element {
    pub name: String,
    pub symbol: String,
    pub atomic_number: u8,
    pub atomic_mass: f64,
    pub density: f64,
    pub group: Option<u8>,
    pub melting_point: Option<Value>,
    pub boiling_point: Option<Value>,
}

impl Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{} ({})\nAtomic number: {}\nAtomic mass: {}",
            self.symbol, self.name, self.atomic_number, self.atomic_mass
        )
    }
}

/// Represents a chemical formula with its elements with corresponding indices, and molecular mass.
#[derive(Debug, Clone)]
pub struct Formula {
    pub formula: String,
    pub elements: HashMap<String, u8>,
    pub mass: f64,
}

impl Formula {
    pub fn new(formula_str: &str) -> Self {
        Formula {
            formula: formula_str.to_string(),
            elements: HashMap::new(),
            mass: 0.0,
        }
    }
}

impl Display for Formula {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{} \nMass: {}\nElements: {:?}",
            self.formula, self.mass, self.elements
        )
    }
}

/// Represents a chemical equation with its reactants and products.
#[derive(Debug, Clone)]
pub struct Equation {
    pub equation: String,
    pub reactants: HashMap<String, u8>,
    pub products: HashMap<String, u8>,
    reactants_formulas: HashMap<String, Formula>,
    products_formulas: HashMap<String, Formula>,
}

impl Display for Equation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{} \nReactants: {:?}\nProducts: {:?}",
            self.equation, self.reactants, self.products
        )
    }
}

impl Equation {
    pub fn new(
        equation: String,
        reactants: HashMap<String, u8>,
        products: HashMap<String, u8>,
        reactants_formulas: HashMap<String, Formula>,
        products_formulas: HashMap<String, Formula>,
    ) -> Self {
        Equation {
            equation,
            reactants,
            products,
            reactants_formulas,
            products_formulas,
        }
    }

    /// Checks if the equation is balanced by comparing the total mass of reactants and products.
    pub fn check_equation(&self) -> bool {
        let reactant_mass: f64 = self
            .reactants
            .iter()
            .map(|(form, coefficient)| (*coefficient as f64) * self.reactants_formulas[form].mass)
            .sum();
        let product_mass: f64 = self
            .products
            .iter()
            .map(|(form, coefficient)| (*coefficient as f64) * self.products_formulas[form].mass)
            .sum();

        (reactant_mass - product_mass).abs() < 0.000001
    }
}

/// Represents a collection of chemical elements from periodic table.
pub struct PeriodicTable {
    elements: HashMap<String, Element>,
}

impl PeriodicTable {
    /// Loads elements from a CSV file and creates a `PeriodicTable` instance.
    pub fn from_csv(path: &str) -> Result<Self, Box<dyn Error>> {
        let mut elements = HashMap::new();
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(true)
            .from_path(path)?;

        for result in rdr.deserialize() {
            let element: Element = result?;
            elements.insert(element.symbol.clone(), element);
        }

        Ok(PeriodicTable { elements })
    }

    /// Retrieves an element by its symbol.
    pub fn get_element(&self, symbol: &str) -> Option<&Element> {
        self.elements.get(symbol)
    }
}
