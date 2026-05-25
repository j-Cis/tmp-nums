// ./src/api.rs
//
// [lib]
// name = "x"
// path = "src/api.rs"

/*
 * #[path = "logic/nums.rs"]
 * mod nums;
 * pub use nums::{NumFrac, checked_pow, nwd_gcd, nww_lcm, sequence};
 *
 * #[path = "data/formulas_data.rs"]
 * mod formulas_data;
 * pub use formulas_data::{Formula, Formulas, Record, Records};
 */

// 1. Klocki (Modele)
#[path = "logic/cas.rs"]
mod cas;
pub use cas::{Node};

// 2. Operacje pośrednie (Serwisy)
#[path = "logic/operations.rs"]
mod operations;
pub use operations::{simplify_equation};

// 3. Parsery (Mapowanie)
#[path = "mapping/text_parse.rs"]
mod text_parse;
pub use text_parse::{parse_latex};
