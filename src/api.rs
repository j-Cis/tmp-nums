// ./src/api.rs
//
// [lib]
// name = "x"
// path = "src/api.rs"



#[path = "logic/nums.rs"]
mod nums;
pub use nums::{NumFrac, sequence, checked_pow, nwd_gcd, nww_lcm};


#[path = "data/formulas_data.rs"]
mod formulas_data;
pub use formulas_data::{Formula, Formulas, Record, Records};

