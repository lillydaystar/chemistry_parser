# Chemistry parser

A parser for the analysis of chemical elements, formulas and equations. This parser is designed to automate the processing of chemical formulas and equations, providing structured information about each element in the formula, as well as the ability to recognize and check the correctness of chemical equations.

## Features

The parser will parse the following structures that are defined as string:
- **Chemical elements recognition**. For example: `H`, `O`, `Na`.
- **Chemical formulas recognition and tokenization**. For example: `H2O`, `H2SO4`, `CH3(CH2)4CH3`
- **Chemical equations recognition and analysis**. For example: `2H2 + O2 -> 2H2O`
- **Solving chemical equation** (planning): It is planned to add the ability to automatically balance chemical equations and check their correctness.

## Technical description

The parser processes chemical elements, formulas and equations in the following steps:

1. **Chemical elements recognition** (`element`): An element is identified by its symbol(s) in the periodic table.
2. **Chemical formulas recognition** (`formula`): Each element in the formula is identified by its symbol and number of atoms. For example, in the formula `C6H12O6`, the parser will determine the symbols `C`, `H`, and `O` with the corresponding indices. This can be used to calculate atomic mass and structure chemical formulas.
3. **Groups recognition** (`group`): The parser processes groups with brackets, for example `Al2(SO4)3`.
4. **Chemical equations recognition** (`equation`): The parser separates reactants and products, which makes it possible to determine the structure of the reaction. This can be used for solving chemical equations.
5. **Returning the result structure**: The parsing result is provided as a Rust structure for further analysis.

## Additional information
**[The Hydrogen to Oganesson: Periodic Insights](https://www.kaggle.com/datasets/kanchana1990/hydrogen-to-oganesson-periodic-insights)** dataset was used to define the symbols of chemical elements