use std::collections::HashMap;

#[derive(Debug)]
struct Rule {
    category: String,
    comparison_symbol: String,
    value: u32,
    passing_workflow: String,
}

#[derive(Debug)]
struct PartRating {
    ratings: HashMap<String, u32>,
    ratings_sum: u32,
}

fn accept_part(ratings: &HashMap<String, u32>, workflows: &HashMap<String, Vec<Rule>>) -> bool {
    let mut current_rules: &Vec<Rule> = workflows.get("in").unwrap();
    loop {
        for rule in current_rules {
            let passed: bool;
            if rule.category.eq("") {
                passed = true;
            } else {
                let value: u32 = *ratings.get(&rule.category).unwrap();
                passed = (rule.comparison_symbol.eq(">") && value > rule.value)
                || (rule.comparison_symbol.eq("<") && value < rule.value);
            }

            if !passed {
                continue;
            }

            if rule.passing_workflow.eq("A") {
                return true;
            } else if rule.passing_workflow.eq("R") {
                return false;
            } else {
                current_rules = workflows.get(&rule.passing_workflow).unwrap();
                break;
            }
        }
    }
}

fn main() {
    // let input: &str = include_str!("./exemplo_1.txt");
    let input: &str = include_str!("./input.txt");

    let (input_workflows, input_ratings): (&str, &str) = input.split_once("\n\n").unwrap();

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

    let mut part_ratings: Vec<PartRating> = Vec::new();
    for ratings in input_ratings.lines() {
        let ratings_list: String = ratings.replace(|r: char| r.eq(&'{') || r.eq(&'}'), "");

        let mut part_rating: PartRating = PartRating {
            ratings: HashMap::new(),
            ratings_sum: 0,
        };

        for rating in ratings_list.split(",") {
            let (category, str_value): (&str, &str) = rating.split_once("=").unwrap();
            let value: u32 = str_value.parse::<u32>().unwrap();

            part_rating.ratings.insert(category.to_string(), value);

            part_rating.ratings_sum += value;
        }

        part_ratings.push(part_rating);
    }

    let mut accepted_parts_ratings: u32 = 0;
    for part in part_ratings {
        if accept_part(&part.ratings, &workflows) {
            accepted_parts_ratings += part.ratings_sum as u32;
        }
    }

    println!("{}", accepted_parts_ratings);
}
