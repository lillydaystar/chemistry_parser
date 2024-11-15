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
    /// Full name of the chemical element
    pub name: String,
    /// Symbol of the element
    pub symbol: String,
    /// Atomic number of the element from periodic table
    pub atomic_number: u8,
    /// Atomic mass of the element from periodic table
    pub atomic_mass: f64,
    /// Density of the element.
    pub density: f64,
    /// Optional group number of the element in the periodic table.
    pub group: Option<u8>,
    /// Optional melting point of the element.
    pub melting_point: Option<Value>,
    /// Optional boiling point of the element.
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
    /// String representation of the formula.
    pub formula: String,
    /// Map of element symbols to their counts.
    pub elements: HashMap<String, u8>,
    /// Molecular mass of the formula.
    pub mass: f64,
}

impl Formula {
    /// Creates a new Formula instance with the specified formula string.
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
    /// String representation of the equation.
    pub equation: String,
    /// Map of reactants and their coefficients.
    pub reactants: HashMap<String, u8>,
    /// Map of products and their coefficients.
    pub products: HashMap<String, u8>,
    /// Map of reactant formulas and their Formula structures.
    reactants_formulas: HashMap<String, Formula>,
    /// Map of product formulas and their Formula structures.
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
    /// Creates a new Equation instance with reactants, products, and their formulas.
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
