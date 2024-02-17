use std::collections::HashMap;

#[derive(Debug)]
struct Rule {
    category: String,
    comparison_symbol: String,
    value: u32,
    passing_workflow: String,
}

#[derive(Debug, Clone)]
struct RatingRanges {
    ranges: HashMap<String, (u32, u32)>,
    current_rule: String,
}

fn get_combination_count(ranges: &HashMap<String, (u32, u32)>) -> u64 {
    let mut count: u64 = 1;
    for (lower, upper) in ranges.values() {
        count *= ((upper - lower) as u64) + 1;
    }

    return count;
}

fn main() {
    // let input: &str = include_str!("./exemplo_1.txt");
    let input: &str = include_str!("./input.txt");

    let input_workflows: &str = input.split_once("\n\n").unwrap().0;

    let mut workflows: HashMap<String, Vec<Rule>> = HashMap::new();
    for workflow in input_workflows.lines() {
        let (name, input_rules): (&str, &str) = workflow.split_once("{").unwrap();
        let str_rules: Vec<&str> = input_rules.split(",").collect();
        let mut rules: Vec<Rule> = Vec::new();

        for i in 0..(str_rules.len() - 1) {
            let (comparison, passing_workflow): (&str, &str) =
                str_rules[i].split_once(":").unwrap();

            let symbol: String = String::from(comparison.chars().nth(1).unwrap());
            let (category, value): (&str, &str) = comparison.split_once(&symbol).unwrap();

            rules.push(Rule {
                category: category.to_string(),
                comparison_symbol: symbol,
                value: value.parse::<u32>().unwrap(),
                passing_workflow: passing_workflow.to_string(),
            })
        }

        rules.push(Rule {
            category: "".to_string(),
            comparison_symbol: "".to_string(),
            value: 0,
            passing_workflow: str_rules[str_rules.len() - 1].replace("}", ""),
        });

        workflows.insert(name.to_string(), rules);
    }

    workflows.insert("A".to_string(), Vec::new());
    workflows.insert("R".to_string(), Vec::new());

    let mut accepted_ranges: Vec<RatingRanges> = vec![RatingRanges {
        ranges: HashMap::from([
            ("x".to_string(), (1, 4000)),
            ("m".to_string(), (1, 4000)),
            ("a".to_string(), (1, 4000)),
            ("s".to_string(), (1, 4000)),
        ]),
        current_rule: "in".to_string(),
    }];

    let mut combinations: u64 = 0;
    while !accepted_ranges.is_empty() {
        let mut ranges: RatingRanges = accepted_ranges.pop().unwrap();

        for rule in workflows.get(&ranges.current_rule).unwrap() {
            if rule.category == "" {
                ranges.current_rule = rule.passing_workflow.to_string();
            } else {
                if (rule.comparison_symbol == "<"
                    && ranges.ranges.get(&rule.category).unwrap().0 >= rule.value)
                    || (rule.comparison_symbol == ">"
                        && ranges.ranges.get(&rule.category).unwrap().1 <= rule.value)
                {
                    continue;
                }

                if (rule.comparison_symbol == "<"
                    && ranges.ranges.get(&rule.category).unwrap().1 < rule.value)
                    || (rule.comparison_symbol == ">"
                        && ranges.ranges.get(&rule.category).unwrap().0 > rule.value)
                {
                    ranges.current_rule = rule.passing_workflow.to_string();
                    break;
                }

                let mut passing_ranges: RatingRanges = ranges.clone();
                let range: (u32, u32) = *ranges.ranges.get(&rule.category).unwrap();

                if rule.comparison_symbol == ">" {
                    ranges.ranges.insert(rule.category.to_string(), (range.0, rule.value));
                    passing_ranges.ranges.insert(rule.category.to_string(), (rule.value + 1, range.1));
                } else {
                    ranges.ranges.insert(rule.category.to_string(), (rule.value, range.1));
                    passing_ranges.ranges.insert(rule.category.to_string(), (range.0, rule.value - 1));
                }

                accepted_ranges.push(passing_ranges);
            }
        }

        if ranges.current_rule == "A" {
            combinations += get_combination_count(&ranges.ranges);
        } else if ranges.current_rule != "R" {
            accepted_ranges.push(ranges);
        }
    }

    println!("{}", combinations);
}
