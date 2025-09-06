// Copyright © 2025 theplaceincan
// ***** Workout Calculator *****

use std::io;

const KG_PER_LB: f64 = 0.453592;
const MAINTENANCE_MULTIPLIER: f64 = 15.0;
const PROTEIN_LOW_MULTIPLIER: f64 = 1.6;
const PROTEIN_HIGH_MULTIPLIER: f64 = 2.2;
const FATS_LOW_PERCENT: f64 = 0.25;
const FATS_HIGH_PERCENT: f64 = 0.30;
// const CALORIES_PER_PROTEIN: f64 = 4.0;
// const CALORIES_PER_CARB: f64 = 4.0;
const CALORIES_PER_FAT: f64 = 9.0;

struct Stats {
  bmr: u32,
  maintenance: u32,
  surplus_low: u32,
  surplus_high: u32,
  protein_low: u32,
  protein_high: u32,
  fats_low: u32,
  fats_high: u32,
}

// --------- Function to get num input from the user
fn get_numeric_input(prompt: &str) -> f64 {
  loop {
    println!("{}", prompt);
    let mut input = String::new();
    
    io::stdin()
      .read_line(&mut input)
      .expect("Error while reading line");
      match input.trim().parse::<f64>() {
      Ok(num) => return num,
      
      Err(_) => {
        println!("Invalid input, please type a number");
        continue;
      }
    };
  }
}


// --------- Function to calculate BMR
fn calculate_bmr(weight_lbs: f64, height_cm: f64, age: f64, gender: &str) -> f64 {
  let weight_kg = weight_lbs * KG_PER_LB;
  let base_calories = 10.0 * weight_kg + 6.25 * height_cm - 5.0 * age;

  match gender {
    "female" => base_calories - 161.0,
    "male" => base_calories + 5.0,
    _ => base_calories,
  }
}

fn main() {
  println!("======== Welcome to the Workout Calculator ========");

  loop {
    println!("\nEnter your gender (male/female):");
    let mut gender = String::new();
    io::stdin().read_line(&mut gender).expect("Error reading gender");
    let gender = gender.trim().to_lowercase();
      
    let weight_lbs = get_numeric_input("Enter your weight (lbs):");
    let height_cm = get_numeric_input("Enter your height (cm):");
    let age = get_numeric_input("Enter your age:");
    let bmr = calculate_bmr(weight_lbs, height_cm, age, &gender);
    let maintenance = weight_lbs * MAINTENANCE_MULTIPLIER;

    let stats = Stats {
      bmr: bmr as u32,
      maintenance: maintenance as u32,
      surplus_low: (maintenance + 300.0) as u32,
      surplus_high: (maintenance + 500.0) as u32,
      protein_low: (weight_lbs * PROTEIN_LOW_MULTIPLIER) as u32,
      protein_high: (weight_lbs * PROTEIN_HIGH_MULTIPLIER) as u32,
      fats_low: ((maintenance * FATS_LOW_PERCENT) / CALORIES_PER_FAT) as u32,
      fats_high: ((maintenance * FATS_HIGH_PERCENT) / CALORIES_PER_FAT) as u32,
    };

    println!("\n======== Workout Results ========");
    println!("BMR---------: {} calories/day", stats.bmr);
    println!("Maintenance---------: ~{} calories/day", stats.maintenance);
    println!("Calorie Surplus (for muscle gain)---------:  {} - {} calories/day", stats.surplus_low, stats.surplus_high);
    println!("Recommended Daily Protein---------:  {} - {} grams", stats.protein_low, stats.protein_high);
    println!("Recommended Daily Fats---------:  {} - {} grams", stats.fats_low, stats.fats_high);
    println!("Fill the rest of your calories with Carbohydrates.\n");
    println!("Would you like to start over? (y/n)");
    
    let mut again = String::new();
    io::stdin().read_line(&mut again).expect("Error");
    
    if again.trim().to_lowercase() != "y" {
        break;
    }
  }
  
  println!("\nThank you for using the Workout Calculator!");
}