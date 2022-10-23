use regex::Regex;

fn main() {
    println!("expression: ");
    let mut expression = String::new();
    std::io::stdin().read_line(&mut expression).unwrap();

    println!("Result: {}", reduce_expression(expression))
}

fn reduce_expression(mut expression: String) -> String {
    let re_add = Regex::new(r"(?P<left>\d+)\s?\+\s?(?P<right>\d+)").unwrap();
    let re_sub = Regex::new(r"(?P<left>\d+)\s?\-\s?(?P<right>\d+)").unwrap();
    let re_mult = Regex::new(r"(?P<left>\d+)\s?\*\s?(?P<right>\d+)").unwrap();
    let re_div = Regex::new(r"(?P<left>\d+)\s?/\s?(?P<right>\d+)").unwrap();

    let precedence = [("/", re_div), ("*", re_mult), ("+", re_add), ("-", re_sub)];

    expression = precedence
        .into_iter()
        .fold(expression, |acc, (operator, re)| apply(re, acc, operator));

    expression
}

// apply the given operation
fn apply(re: Regex, mut expression: String, operator: &str) -> String {
    loop {
        let caps = re.captures(expression.as_str());

        if caps.is_none() {
            break;
        }

        let caps = caps.unwrap();
        let cap_expression = caps.get(0).unwrap().as_str();
        let left_value: i32 = caps.name("left").unwrap().as_str().parse().unwrap();
        let right_value: i32 = caps.name("right").unwrap().as_str().parse().unwrap();
        let result = match operator {
            "+" => left_value + right_value,
            "-" => left_value - right_value,
            "*" => left_value * right_value,
            "/" => left_value / right_value,
            _ => 0,
        };

        expression = expression.replace(cap_expression, &result.to_string());
    }

    expression
}
