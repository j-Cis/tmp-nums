// ./src/main.rs
/* 
 * fn main() {
	* let p = x::Formulas::new_from([
	* 	x::Formula("a_ce_dir", "12^{k-2}", |k|
 * x::NumFrac::from_value(12f64.powi(k - 2))),
 *	 x::Formula("a_center", "12^{k}", |k|
 * x::NumFrac::from_value(12f64.powi(k))),
	* x::Formula("a_ce_inv", "12^{k+2}", |k|
 * x::NumFrac::from_value(12f64.powi(k + 2))),
	* x::Formula("b_direct", "12^{k}·2^{-1}", |k|
 * x::NumFrac::from_value(12f64.powi(k) * 2f64.powi(-1))),
	* x::Formula("b_invert", "12^{k}·2^{+1}", |k|
 * x::NumFrac::from_value(12f64.powi(k) * 2f64.powi(1))),
	* 	x::Formula("c_direct", "12^{k-2}·3^{+2}", |k| 
 * x::NumFrac::from_value(12f64.powi(k - 2) * 3f64.powi(2))),
 * x::Formula("c_invert", "12^{k+2}·3^{-2}", |k| 
 * x::NumFrac::from_value(12f64.powi(k + 2) * 3f64.powi(-2))),
 * 	]);
 *
 * p.make((-4, 4)).print_table();
 * }
 */

mod api;
use api::Node;

fn main() {
    println!("--- URUCHAMIANIE SYSTEMU CAS ---");

    // PRZYKŁAD 1: log_e(e)
    let wyrazenie1 = Node::Log(Box::new(Node::E), Box::new(Node::E));
    
    println!("--- Przykład 1 ---");
    println!("Przed uproszczeniem: {}", wyrazenie1.to_latex());
    println!("Po uproszczeniu:     {}", api::simplify_equation(&wyrazenie1).to_latex());
    println!();

    // PRZYKŁAD 2: log_e(k^n) z Twojego wzoru
    let k = Node::Var("k".to_string());
    let n = Node::Var("n".to_string());
    
    let potega = Node::Pow(Box::new(k), Box::new(n));
    let wyrazenie2 = Node::Log(Box::new(Node::E), Box::new(potega));

    println!("--- Przykład 2 ---");
    println!("Przed uproszczeniem: {}", wyrazenie2.to_latex());
    println!("Po uproszczeniu:     {}", api::simplify_equation(&wyrazenie2).to_latex());
}
