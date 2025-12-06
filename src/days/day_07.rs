use std::collections::HashMap;

use crate::utils::input::read_input;

#[derive(Debug, Clone)]
enum Gate {
    Not(Symbol),
    And(Symbol, Symbol),
    Or(Symbol, Symbol),
    Rshift(Symbol, u8),
    Lshift(Symbol, u8),
}

#[derive(Debug, Clone)]
enum Symbol {
    Constant(u16),
    Wire(String),
}

#[derive(Debug, Clone)]
enum Expression {
    Symbol(Symbol),
    Gate(Gate),
}

type Circuit = HashMap<String, Expression>;

fn parse_symbol(op: &str) -> Symbol {
    let num = op.parse::<u16>();
    if num.is_ok() {
        Symbol::Constant(num.unwrap())
    } else {
        Symbol::Wire(op.to_owned())
    }
}

fn parse_expression(op: &str) -> Expression {
    let mut symbols = op.split(' ');
    match symbols.clone().count() {
        1 => Expression::Symbol(parse_symbol(symbols.next().unwrap())),
        2 => {
            let operand = parse_symbol(symbols.nth(1).unwrap());
            Expression::Gate(Gate::Not(operand))
        }
        3 => {
            let operand1 = parse_symbol(symbols.next().unwrap());
            let operator = symbols.next().unwrap();
            match operator {
                "AND" => {
                    let operand2 = parse_symbol(symbols.next().unwrap());
                    Expression::Gate(Gate::And(operand1, operand2))
                }
                "OR" => {
                    let operand2 = parse_symbol(symbols.next().unwrap());
                    Expression::Gate(Gate::Or(operand1, operand2))
                }
                "RSHIFT" => {
                    let operand2 = symbols.next().unwrap().parse().unwrap();
                    Expression::Gate(Gate::Rshift(operand1, operand2))
                }
                "LSHIFT" => {
                    let operand2 = symbols.next().unwrap().parse().unwrap();
                    Expression::Gate(Gate::Lshift(operand1, operand2))
                }
                _ => panic!(),
            }
        }
        _ => panic!(),
    }
}

fn parse_input(input: &str) -> Circuit {
    input
        .lines()
        .map(|line| {
            let mut spl = line.split(" -> ");
            let signal = parse_expression(spl.next().unwrap());
            (spl.next().unwrap().to_owned(), signal)
        })
        // .inspect(|(w, s)| println!("{:?}: {:?}", w, s))
        .collect()
}

fn get_symbol_value(circuit: &mut Circuit, sym: &Symbol) -> u16 {
    match sym {
        Symbol::Constant(v) => *v,
        Symbol::Wire(w) => get_wire_value(circuit, &w),
    }
}

fn get_gate_value(circuit: &mut Circuit, g: &Gate) -> u16 {
    match g {
        Gate::Not(sym) => get_symbol_value(circuit, sym) ^ 0xffff,
        Gate::And(sym1, sym2) => get_symbol_value(circuit, sym1) & get_symbol_value(circuit, sym2),
        Gate::Or(sym1, sym2) => get_symbol_value(circuit, sym1) | get_symbol_value(circuit, sym2),
        Gate::Rshift(sym, n) => get_symbol_value(circuit, sym) >> n,
        Gate::Lshift(sym, n) => get_symbol_value(circuit, sym) << n,
    }
}

fn get_wire_value(circuit: &mut Circuit, wire: &str) -> u16 {
    let e = circuit.get(wire).unwrap().clone();
    let v = match e {
        Expression::Symbol(Symbol::Constant(v)) => return v, // return early to prevent redundant update
        Expression::Symbol(sym) => get_symbol_value(circuit, &sym),
        Expression::Gate(g) => get_gate_value(circuit, &g),
    };
    *circuit.get_mut(wire).unwrap() = Expression::Symbol(Symbol::Constant(v));
    v
}

fn solve_part_1(input: &str) {
    let mut circuit = parse_input(input);
    // println!("{:?}", circuit);
    let signal = get_wire_value(&mut circuit, "a");
    println!("{}", signal);
}

fn solve_part_2(input: &str) {
    let mut circuit = parse_input(input);
    let signal = get_wire_value(&mut circuit, "a");

    circuit = parse_input(input);
    *circuit.get_mut("b").unwrap() = Expression::Symbol(Symbol::Constant(signal));
    let signal = get_wire_value(&mut circuit, "a");
    println!("{}", signal);
}

pub fn part_1() {
    let input = read_input(module_path!());
    solve_part_1(input.as_str());
}

pub fn part_2() {
    let input = read_input(module_path!());
    solve_part_2(input.as_str());
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    const EXAMPLE_1: &str = indoc! {"
    "};

    #[test]
    fn test_part_1() {
        super::solve_part_1(EXAMPLE_1);
    }

    const EXAMPLE_2: &str = EXAMPLE_1;

    #[test]
    fn test_part_2() {
        super::solve_part_2(EXAMPLE_2);
    }
}
