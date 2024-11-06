use std::collections::HashMap;
use std::error::Error;
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
