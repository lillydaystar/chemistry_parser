use std::collections::HashMap;
use std::error::Error;
use std::fmt::Display;
use std::fs::File;

#[derive(Debug, Clone)]
pub struct Element {
    pub name: String,
    pub symbol: String,
    pub atomic_number: u8,
    pub atomic_mass: f64,
    pub density: f64,
    pub group: Option<u8>,
    pub melting_point: Option<f64>,
    pub boiling_point: Option<f64>,
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

#[derive(Debug, Clone)]
pub struct Equation {
    equation: String,
    reactants: HashMap<String, u8>,
    products: HashMap<String, u8>,
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

    pub fn solve_equation(self) -> Equation {
        let result = self.clone();

        if self.check_equation() {
            return result;
        }
        // TODO: Define algorithm for solving chemical equations

        result
    }
}

pub struct PeriodicTable {
    elements: HashMap<String, Element>,
}

impl PeriodicTable {
    pub fn from_csv(path: &str) -> Result<Self, Box<dyn Error>> {
        let mut elements = HashMap::new();
        let file = File::open(path)?;
        let mut rdr = csv::Reader::from_reader(file);

        for result in rdr.records() {
            let record = result?;
            let element = Element {
                name: record[0].to_string(),
                symbol: record[1].to_string(),
                atomic_number: record[2].parse()?,
                atomic_mass: record[3].parse()?,
                density: record[4].parse()?,
                group: record[5].parse().ok(),
                melting_point: record[6].parse().ok(),
                boiling_point: record[7].parse().ok(),
            };
            elements.insert(element.symbol.clone(), element);
        }

        Ok(PeriodicTable { elements })
    }

    pub fn get_element(&self, symbol: &str) -> Option<&Element> {
        self.elements.get(symbol)
    }
}
