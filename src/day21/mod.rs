use std::collections::{HashMap, HashSet};
use std::str::FromStr;

use crate::AdventError;

type Allergen = String;
type Ingredient = String;

pub fn part1() -> Result<(), AdventError> {
    let input = include_str!("input");
    let recipes = input.parse::<RecipeList>()?;
    solve(&recipes);
    Ok(())
}

pub fn test() -> Result<(), AdventError> {
    let input = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
    trh fvjkl sbzzf mxmxvkd (contains dairy)
    sqjhc fvjkl (contains soy)
    sqjhc mxmxvkd sbzzf (contains fish)";
    let recipes = input.parse::<RecipeList>()?;
    solve(&recipes);
    Ok(())
}
struct RecipeList {
    recipes: Vec<Recipe>,
}

#[derive(Debug)]
struct Recipe {
    ingredients: Vec<Ingredient>,
    allergens: Vec<Allergen>,
}

fn solve(input: &RecipeList) {
    let allergens_with_recipe = find_recipes_for_allergens(input);
    let allergen_sources = compute_allergens(allergens_with_recipe);
    println!("There are {} other ingredients", count_unknown_ingredients(input, &allergen_sources));

    let mut allergens = allergen_sources.keys().collect::<Vec<&String>>();
    allergens.sort();

    let mut list = String::new();
    for allergen in allergens {
        if list.len() > 0 {
            list.push(',');
        }
        list.push_str(&allergen_sources[allergen]);
    }
    println!("Danger list: {}", list);
}

fn count_unknown_ingredients(input: &RecipeList, allergen_sources: &HashMap<Allergen, Ingredient>) -> usize {
    let allergen_sources = allergen_sources.values().collect::<HashSet<&String>>();
    let mut result = 0;
    for recipe in &input.recipes {
        result += recipe.ingredients.iter()
                                    .filter(|&i| !allergen_sources.contains(i))
                                    .count();
    }
    result
}
fn compute_allergens(input: HashMap<Allergen, Vec<&Recipe>>) -> HashMap<Allergen, Ingredient> {
    let mut ingredient_candidates: HashMap<Allergen, HashSet<Ingredient>> = HashMap::new();
    for (allergen, recipes) in input.iter() {
        let mut candidate_ingredients: HashSet<Ingredient> = HashSet::new();

        for (i, recipe) in recipes.iter().enumerate() {
            if i == 0 {
                for ingredient in &recipe.ingredients {
                    candidate_ingredients.insert(ingredient.clone());
                }
            } else {
                candidate_ingredients = candidate_ingredients
                                            .iter()
                                            .filter(|&i| recipe.ingredients.contains(i))
                                            .map(|s| s.clone())
                                            .collect::<HashSet<Ingredient>>();
            }
        }

        ingredient_candidates.insert(allergen.clone(), candidate_ingredients);
    }

    let mut allergen_source: HashMap<Allergen, Ingredient> = HashMap::new();
    while ingredient_candidates.len() > 0 {
        if let Some((allergen, ingredient)) = find_candidate_allergen(&ingredient_candidates) {
            ingredient_candidates.remove(&allergen);
            for possible_ingredients in ingredient_candidates.values_mut() {
                possible_ingredients.remove(&ingredient);
            }
            allergen_source.insert(allergen, ingredient);
        } else {
            panic!("Cannot continue at {:?}", ingredient_candidates);
        }
    }
    allergen_source
}

fn find_candidate_allergen(options: &HashMap<Allergen, HashSet<Ingredient>>) -> Option<(Allergen, Ingredient)> {
    options.iter()
           .filter(|&(_,v)| v.len() == 1)
           .map(|(k, v)| (k.to_string(), v.iter().nth(0).unwrap().to_string()) )
           .nth(0)
}
fn find_recipes_for_allergens(input: &RecipeList) -> HashMap<Allergen, Vec<&Recipe>> {
    let mut result: HashMap<Allergen, Vec<&Recipe>> = HashMap::new();
    for recipe in &input.recipes {
        for allergen in &recipe.allergens {
            if !result.contains_key(allergen) {
                result.insert(allergen.clone(), Vec::new());
            }
            result.get_mut(allergen).unwrap().push(recipe);
        }
    }
    result
} 
impl FromStr for RecipeList {
    type Err = AdventError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut recipes = Vec::new();
        for line in s.lines() {
            let main_chunks: Vec<&str> = line.split("(contains").collect();
            if main_chunks.len() != 2 {
                return Err(AdventError{cause: format!("Cannot understand {}", line)});
            }
            let ingredients = main_chunks[0].trim()
                                                       .split(" ")
                                                       .map(|s| s.to_string())
                                                       .collect();
            let allergens = main_chunks[1].trim_end_matches(')')
                                                     .split(",")
                                                     .map(|s| s.trim().to_string())
                                                     .collect();
            recipes.push(Recipe{ingredients, allergens});
        }
        Ok(RecipeList{recipes})
    }
}