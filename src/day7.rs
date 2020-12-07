use crate::regex;
use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

/// Parses the input into an hashmap with the bag as key and another hashmap as element
/// This hashmap contains the information how many of which kind of bag can be contained
#[aoc_generator(day7)]
fn parse_input_day7(input: &str) -> HashMap<String, HashMap<String, i32>> {
    let mut hashmap = HashMap::new();
    for line in input.lines() {
        let (key, value) = convert_line_to_hashmap_entry(line);
        hashmap.insert(key, value);
    }
    hashmap
}

fn convert_line_to_hashmap_entry(line: &str) -> (String, HashMap<String, i32>) {
    let split_line: Vec<&str> = line.trim_end_matches("s.").split(" contain ").collect();
    let bag_key = split_line[0].trim_end_matches(" bags").to_owned();

    let mut bag_amount = HashMap::new();

    for bag_info in split_line[1].split(", ") {
        if bag_info == "no other bag" {
            continue;
        }
        let pattern = regex!(r"(\d+) (.*) bag");
        let captures = pattern.captures(bag_info).unwrap();
        let amount: i32 = str::parse(captures.get(1).unwrap().as_str()).unwrap();
        let bag: String = captures.get(2).unwrap().as_str().to_owned();
        bag_amount.insert(bag, amount);
    }

    (bag_key, bag_amount)
}

#[aoc(day7, part1)]
fn how_many_bags_can_contain_shiny_gold(input: &HashMap<String, HashMap<String, i32>>) -> i32 {
    let mut amount_bags = 0;

    for contained_bags in input.values() {
        amount_bags += contains_shiny_gold(input, Some(contained_bags));
    }

    amount_bags
}

fn contains_shiny_gold(
    input: &HashMap<String, HashMap<String, i32>>,
    contained_bags: Option<&HashMap<String, i32>>,
) -> i32 {
    if let Some(contained_bags) = contained_bags {
        for bag in contained_bags.keys() {
            if bag == "shiny gold" {
                return 1;
            }
            let contains = contains_shiny_gold(input, input.get(bag));
            if contains == 1 {
                return 1;
            }
        }
    }
    return 0;
}

#[test]
fn test_1() {
    let input = "\
light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

    let hashmap = parse_input_day7(input);
    assert!(hashmap["bright white"].contains_key("shiny gold"));
    assert!(hashmap["muted yellow"].contains_key("shiny gold"));
    assert!(hashmap["dark orange"].contains_key("bright white"));
    assert!(hashmap["light red"].contains_key("muted yellow"));

    assert_eq!(4, how_many_bags_can_contain_shiny_gold(&hashmap));
}
