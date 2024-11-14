# Chemistry parser

Link: https://crates.io/crates/chemistry_parser
Docs: https://docs.rs/chemistry_parser/latest/chemistry_parser/

A parser for the analysis of chemical elements, formulas and equations. This parser is designed to automate the processing of chemical formulas and equations, providing structured information about each element in the formula, as well as the ability to recognize and check the correctness of chemical equations.

## Features

- The parser can parse the following structures that are defined as string:
  - **Chemical elements**. For example: `H`, `O`, `Na`.
  - **Chemical formulas**. For example: `H2O`, `H2SO4`, `CH3(CH2)4CH3`
  - **Chemical equations**. For example: `2H2 + O2 -> 2H2O`
- The parser can check whether a given chemical equation is balanced.
  - **Solving chemical equation** (planning): It is planned to add the ability to automatically balance chemical equations and check their correctness.

## Technical description

The parser processes chemical elements, formulas and equations in the following steps:

1. **Chemical elements recognition** (`element`): An element is identified by its symbol(s) in the periodic table.
2. **Chemical formulas recognition** (`formula`): Each element in the formula is identified by its symbol and number of atoms (`index`). For example, in the formula `C6H12O6`, the parser will determine the symbols `C`, `H`, and `O` with the corresponding indices. The molecular mass is calculated based on the elements and their indices in the chemical formula.
3. **Groups recognition** (`group`): The parser processes groups enclosed in brackets, for example `Al2(SO4)3`. The parser multiplies the count of elements inside the group by the number outside the brackets.
4. **Chemical equations recognition** (`equation`): The parser separates reactants and products, which makes it possible to determine the structure of the reaction. It ensures the correct format, parsing both the reactants and the products. This makes it possible to determine if the equation is balanced.
5. **Returning the result structure**: The parsing result is provided as a Rust structure, which includes:
   - `Element`: parsed element with its information from the periodic table;
   - `Formula`: parsed formula with its elements and molecular mass;
   - `Equation`: parsed equation with its reactants and products.

The parsing process is described in the diagram below:
![parsing process diagram](https://github.com/lillydaystar/chemistry_parser/blob/main/data/diagram.png?raw=true)

### Example

The CLI interface allows interaction with the parser using different commands. 

```shell
cargo run help 
```
Output:
```
Use following commands:
  help                            Show all commands
  credits                         Show credits
  symbol <element-symbol>         Parse the element and print information about it
  formula <chemical-formula>      Parse the formula and print information about it
  equation <chemical-equation>    Parse the chemical equation and print its formulas
  check <chemical-equation>       Check if the chemical equation is balanced
  file <file-path>                Parse the file with chemical equations and solve them
```

## Additional information
**The parser uses the [Hydrogen to Oganesson: Periodic Insights](https://www.kaggle.com/datasets/kanchana1990/hydrogen-to-oganesson-periodic-insights)** dataset to define and validate the symbols of chemical elements