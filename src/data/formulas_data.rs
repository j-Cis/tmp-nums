// ./src/data/formulas_data.rs


use super::{NumFrac,sequence};
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct Formula(pub &'static str, pub &'static str, pub fn(i32) -> NumFrac);

#[derive(Debug, Clone)]
pub struct Formulas(pub Vec<Formula>);

#[derive(Debug, Clone)]
pub struct Record(pub String,pub String,pub BTreeMap<i32, NumFrac>);

#[derive(Debug, Clone)]
pub struct Records(pub Vec<Record>);

impl Formulas {
    pub fn new() -> Self {
        Formulas(Vec::new())
    }

    pub fn new_from<const N: usize>(ff: [Formula; N]) -> Self {
        Formulas(ff.to_vec())
    }

    pub fn add_formula(&mut self, label: &'static str, formula_math: &'static str, formula_func: fn(i32) -> NumFrac) {
        self.0.push(Formula(label, formula_math, formula_func));
    }

    pub fn make(self, kk: (i32, i32)) -> Records {
        let items = self.0
            .iter()
            .map(|f| {
                let set = sequence(kk, f.2);
                Record(f.0.to_string(), f.1.to_string(), set)
            })
            .collect();
        Records(items)
    }
}

impl Records {
pub fn print_table(&self) {
        if self.0.is_empty() {
            return;
        }

        // 1. Nagłówek: Etykiety kolumn
        print!("   |");
        for item in &self.0 {
            print!(" {:>17} |", item.0);
        }
        println!();

        // 2. Nagłówek: Wzory matematyczne
        print!(" k |");
        for item in &self.0 {
            print!(" {:>17} |", item.1);
        }
        println!();

        // Linia rozdzielająca dopasowana do szerokości kolumn
        let total_width = 4 + self.0.len() * 20;
        println!("{}", "-".repeat(total_width));

        // 3. Wiersze danych dla każdego k pobranego z pierwszej mapy
        if let Some(first_item) = self.0.first() {
            for &k in first_item.2.keys() {
                // Linia ułamków (fract)
                print!("{:>2} |", k);
                for item in &self.0 {
                    if let Some(num_frac) = item.2.get(&k) {
                        print!(" {:>17} |", num_frac.fract);
                    }
                }
                println!();

                // Linia wartości zmiennoprzecinkowych (float)
                print!("   |");
                for item in &self.0 {
                    if let Some(num_frac) = item.2.get(&k) {
                        print!(" {:>17} |", num_frac.float);
                    }
                }
                println!();
                println!();
            }
        }
    }
}

