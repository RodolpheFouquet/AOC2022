use std::cell::RefCell;
use std::collections::VecDeque;
use std::fmt::Display;
use std::rc::Rc;

use pest::Parser;
use pest_derive::*;

#[derive(Parser)]
#[grammar = "monkeys.pest"] // relative to src
struct MonkeyParser;

struct Monkey {
    id: u32,
    worry_levels: VecDeque<u32>,
    operation: Operation,
    test: Test,
    target_true: u32,
    target_false: u32,
    count: u32,
}

impl Display for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Monkey {}:", self.id)?;
        for i in 0..self.worry_levels.len() {
            write!(f, " {}", self.worry_levels[i]).unwrap();
            if i < self.worry_levels.len() - 1 {
                write!(f, ",").unwrap();
            }
        }
        writeln!(f)
    }
}

#[derive(Debug, Copy, Clone)]
enum Op {
    Add,
    Mul,
}

impl TryFrom<&str> for Op {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "+" => Ok(Op::Add),
            "*" => Ok(Op::Mul),
            _ => Err(()),
        }
    }
}
impl Op {
    fn run(self, a: u32, b: u32) -> u32 {
        match self {
            Op::Mul => a * b,
            Op::Add => a + b,
        }
    }
}

type Operation = Box<dyn Fn(u32) -> u32>;
type Test = Box<dyn Fn(u32) -> bool>;

fn main() {
    let file_to_parse =
        std::fs::read_to_string("input2.txt").expect("the file should be on the disk");

    let monkeys_rule = MonkeyParser::parse(Rule::monkeys, &file_to_parse)
        .expect("unsucessful parse")
        .next()
        .unwrap();

    let mut monkeys: Vec<Monkey> = Vec::new();

    for monkey in monkeys_rule.into_inner() {
        match monkey.as_rule() {
            Rule::monkey => {
                let mut monkey_rules = monkey.into_inner();
                let monkey_id = monkey_rules
                    .next()
                    .unwrap()
                    .into_inner()
                    .next()
                    .unwrap()
                    .as_str()
                    .parse::<u32>()
                    .unwrap();

                let mut starting_items: Vec<u32> = Vec::new();
                let starting_rule = monkey_rules.next().unwrap();
                for start in starting_rule.into_inner() {
                    starting_items.push(start.as_str().parse::<u32>().unwrap());
                }

                let mut operation_rule_inner = monkey_rules.next().unwrap().into_inner();
                operation_rule_inner.next().unwrap();
                let op = Op::try_from(operation_rule_inner.next().unwrap().as_str()).unwrap();
                let second_term = operation_rule_inner.next().unwrap();
                let second_term_str = String::from(second_term.clone().as_str());

                let operation: Operation = match second_term.as_rule() {
                    Rule::second_term => Box::new(move |old: u32| -> u32 {
                        let s = second_term_str.parse::<u32>().unwrap();
                        let ret = op.run(old, s);
                        // match op {
                        //     Op::Add => println!("    Worry level increases by {} to {}.", s, ret),
                        //     Op::Mul => println!("    Worry level multiplied by {} to {}.", s, ret),
                        // };
                        ret
                    }),
                    Rule::old => Box::new(move |old: u32| -> u32 {
                        let ret = op.run(old, old);
                        // match op {
                        //     Op::Add => println!("    Worry level doubled to {}.", ret),
                        //     Op::Mul => println!("    Worry level multiplied by itself to {}.", ret),
                        // };
                        ret
                    }),
                    _ => unreachable!(),
                };

                let test_rule = monkey_rules.next().unwrap();
                let second_test_term = test_rule
                    .into_inner()
                    .next()
                    .unwrap()
                    .as_str()
                    .parse::<u32>()
                    .unwrap();
                let test_operation: Test = Box::new(move |val: u32| -> bool {
                    let ret = val % second_test_term == 0;
                    if ret {
                        // println!(
                        //     "    Current worry level {} is divisible by {}.",
                        //     val, second_test_term
                        // );

                        //     "    Current worry level {} is not divisible by {}.",
                        //     val, second_test_term
                        // );
                    }
                    ret
                });

                let target_true = monkey_rules
                    .next()
                    .unwrap()
                    .into_inner()
                    .next()
                    .unwrap()
                    .as_str()
                    .parse::<u32>()
                    .unwrap();
                let target_false = monkey_rules
                    .next()
                    .unwrap()
                    .into_inner()
                    .next()
                    .unwrap()
                    .as_str()
                    .parse::<u32>()
                    .unwrap();

                monkeys.push(Monkey {
                    id: monkey_id,
                    worry_levels: starting_items.into(),
                    target_false,
                    target_true,
                    test: test_operation,
                    operation,
                    count: 0,
                })
            }
            _ => unreachable!(),
        }
    }

    monkeys.iter().for_each(|m| print!("{}", m));

    for round in 0..20 {
        for i in 0..monkeys.len() {
            // println!("Monkey {}:", monkeys[i].id);
            while let Some(front) = monkeys[i].worry_levels.pop_front() {
                monkeys[i].count += 1;
                // println!("  Monkey inspects an item with worry level of {}.", front);
                let new = (monkeys[i].operation)(front) / 3;
                // println!(
                //     "    Monkey gets bored with item. Worry level is divided by 3 to {}.",
                //     new
                // );
                let test_res = (monkeys[i].test)(new);
                let target = if test_res {
                    monkeys[i].target_true
                } else {
                    monkeys[i].target_false
                } as usize;
                // println!(
                //     "    Item with worry level {} is thrown to monkey {}.",
                //     new, target
                // );
                monkeys[target].worry_levels.push_back(new);
            }
        }
        println!("== After round {} ==", round + 1);
        monkeys
            .iter()
            .for_each(|m| println!("Monkey {}: inspected items {} times.", m.id, m.count));
        //monkeys.iter().for_each(|m| print!("{}", m));
    }

    monkeys
        .iter()
        .for_each(|m| println!("Monkey {}: inspected items {} times.", m.id, m.count));
    let mut counts: Vec<u32> = monkeys.iter().map(|m| m.count).collect();
    counts.sort_by(|a, b| a.partial_cmp(b).unwrap());
    println!(
        "{}*{} = {}",
        counts[counts.len() - 1],
        counts[counts.len() - 2],
        counts[counts.len() - 1] * counts[counts.len() - 2]
    )
}
