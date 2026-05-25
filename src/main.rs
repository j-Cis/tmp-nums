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



println!("--- TESTY OPERACJI NA UŁAMKACH ---");

    // Definiujemy zmienne (klocek a, b, c, d)
    let a = Node::Var("a".to_string());
    let b = Node::Var("b".to_string());
    let c = Node::Var("c".to_string());
    let d = Node::Var("d".to_string());

    // Budujemy Ułamek 1: a/b oraz Ułamek 2: c/d
    let ulamek1 = Node::Fr(Box::new(a.clone()), Box::new(b.clone()));
    let ulamek2 = Node::Fr(Box::new(c.clone()), Box::new(d.clone()));

    println!("Ułamek 1: {}", ulamek1.to_latex());
    println!("Ułamek 2: {}", ulamek2.to_latex());
    println!("----------------------------------");

    // 1. MNOŻENIE: (a/b) * (c/d) -> (a * c) / (b * d)
    let gora_mnozenie = Node::Mul(Box::new(a.clone()), Box::new(c.clone()));
    let dol_mnozenie = Node::Mul(Box::new(b.clone()), Box::new(d.clone()));
    let mnozenie = Node::Fr(Box::new(gora_mnozenie), Box::new(dol_mnozenie));
    println!("Mnożenie:        {}", mnozenie.to_latex());

    // 2. DZIELENIE: (a/b) / (c/d) -> (a * d) / (b * c)
    let gora_dzielenie = Node::Mul(Box::new(a.clone()), Box::new(d.clone()));
    let dol_dzielenie = Node::Mul(Box::new(b.clone()), Box::new(c.clone()));
    let dzielenie = Node::Fr(Box::new(gora_dzielenie), Box::new(dol_dzielenie));
    println!("Dzielenie:       {}", dzielenie.to_latex());

    // 3. DODAWANIE: a/b + c/d -> (a*d + c*b) / (b*d)
    let ad = Node::Mul(Box::new(a.clone()), Box::new(d.clone()));
    let cb = Node::Mul(Box::new(c.clone()), Box::new(b.clone()));
    let gora_dodawanie = Node::Add(Box::new(ad), Box::new(cb));
    let dol_dodawanie = Node::Mul(Box::new(b.clone()), Box::new(d.clone()));
    let dodawanie = Node::Fr(Box::new(gora_dodawanie), Box::new(dol_dodawanie));
    println!("Dodawanie:       {}", dodawanie.to_latex());

    // 4. ODEJMOWANIE: a/b - c/d -> (a*d - c*b) / (b*d)
    // Realizujemy jako: a*d + (-1 * (c*b))
    let cb_negatywne = Node::Mul(Box::new(Node::Num(-1)), Box::new(Node::Mul(Box::new(c), Box::new(b))));
    let ad_odejmowanie = Node::Mul(Box::new(a), Box::new(d));
    let gora_odejmowanie = Node::Add(Box::new(ad_odejmowanie), Box::new(cb_negatywne));
    let dol_odejmowanie = Node::Mul(Box::new(b), Box::new(d));
    let odejmowanie = Node::Fr(Box::new(gora_odejmowanie), Box::new(dol_odejmowanie));
    println!("Odejmowanie:     {}", odejmowanie.to_latex());
}
