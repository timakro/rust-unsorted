use std::fs;
use std::collections::{HashMap,HashSet};

fn main() {
    let mut save_ingredients: HashMap<&str, u32> = HashMap::new();
    let mut allergens: HashMap<&str,HashSet<&str>> = HashMap::new();
    let input = fs::read_to_string("input").unwrap();
    for line in input.lines() {
        let mut it = line.strip_suffix(")").unwrap().split(" (contains ");
        let mut ingredients = HashSet::new();
        for ingredient in it.next().unwrap().split(" ") {
            ingredients.insert(ingredient);
            *save_ingredients.entry(ingredient).or_default() += 1;
        }
        for allergen in it.next().unwrap().split(", ") {
            if let Some(i) = allergens.get_mut(allergen) {
                i.retain(|x| ingredients.contains(x));
            } else {
                allergens.insert(allergen, ingredients.clone());
            }
        }
    }

    let mut to_remove: Vec<&str> = allergens.values().filter(|x| x.len() == 1)
                                  .map(|x| *x.iter().nth(0).unwrap()).collect();
    while let Some(rem) = to_remove.pop() {
        for options in allergens.values_mut().filter(|x| x.len() > 1) {
            options.remove(rem);
            if options.len() == 1 {
                to_remove.push(options.iter().nth(0).unwrap());
            }
        }
    }

    let mut canonical: Vec<(&str, &str)> = Vec::new();
    for (allergent, ingredients) in allergens {
        let ingredient = ingredients.iter().nth(0).unwrap();
        save_ingredients.remove(ingredient);
        canonical.push((ingredient, allergent));
    }
    let save_count: u32 = save_ingredients.values().sum();
    println!("Save ingredients appear {} times", save_count);

    canonical.sort_by_key(|x| x.1);
    let canonical = canonical.iter().map(|x| x.0).collect::<Vec<&str>>().join(",");
    println!("{}", canonical);
}
