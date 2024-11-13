use chemistry_parser::ChemParser;
use std::path::Path;
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let parser = ChemParser::new();

    if args.len() < 2 {
        print_help();
        return;
    }

    if args.len() == 2 {
        match args[1].as_str() {
            "credits" => {
                println!("This program was created by Liliia Parashchak, @lillydaystar")
            }
            "help" => {
                print_help();
            }
            _ => {
                eprintln!("Error: Unknown command '{}'", args[1]);
                print_help();
            }
        }
    } else if args.len() == 3 {
        match args[1].as_str() {
            "symbol" => {
                let element = &args[2];
                match parser.parse_element(element) {
                    Ok(parsed_element) => println!("Element: {}", parsed_element),
                    Err(e) => eprintln!("Error: {}, try again", e),
                }
            }
            "formula" => {
                let formula = &args[2];
                match parser.parse_formula(formula) {
                    Ok(parsed_formula) => println!("Formula: {}", parsed_formula),
                    Err(e) => eprintln!("Error: {}, try again", e),
                }
            }
            "equation" => {
                let equation = &args[2];
                match parser.parse_equation(equation) {
                    Ok(parsed_equation) => println!("Equation: {}", parsed_equation),
                    Err(e) => eprintln!("Error: {}, try again", e),
                }
            }
            "check" => {
                let equation = &args[2];
                match parser.parse_equation(equation) {
                    Ok(parsed_equation) => {
                        println!("Equation: \n{}", parsed_equation);
                        if parsed_equation.check_equation() {
                            println!("Equation is balanced.")
                        } else {
                            println!("Equation is not balanced.");
                        }
                    }
                    Err(e) => eprintln!("Error: {}, try again", e),
                }
            }
            "file" => {
                let file_path = &args[2];
                if let Err(e) = parse_file_equations(&parser, file_path) {
                    eprintln!("Error: {}", e);
                }
            }
            _ => {
                eprintln!("Error: Unknown command '{}'", args[1]);
                print_help();
            }
        }
    } else {
        eprintln!("Error: Unknown command '{}'", args[1]);
        print_help();
    }
}

fn print_help() {
    println!("Use following commands:");
    println!("  help                            Show all commands");
    println!("  credits                         Show credits");
    println!("  symbol <element-symbol>         Parse the element and print information about it");
    println!("  formula <chemical-formula>      Parse the formula and print information about it");
    println!(
        "  equation <chemical-equation>    Parse the chemical equation and print its formulas"
    );
    println!("  check <chemical-equation>       Check if the chemical equation is balanced");
    println!(
        "  file <file-path>                Parse the file with chemical equations and solve them"
    );
}

fn parse_file_equations(parser: &ChemParser, file_path: &str) -> anyhow::Result<(), String> {
    let path = Path::new(file_path);

    let content =
        fs::read_to_string(path).map_err(|_| format!("Failed to read file: {}", file_path))?;

    for (i, line) in content.lines().enumerate() {
        match parser.parse_equation(line) {
            Ok(parsed_equation) => {
                println!("{}. {}", i + 1, parsed_equation);
                if parsed_equation.check_equation() {
                    println!("Equation is balanced.")
                } else {
                    println!("Equation is not balanced.");
                }
            }
            Err(e) => eprintln!("Error on line {}: {}", i + 1, e),
        }
    }

    Ok(())
}
