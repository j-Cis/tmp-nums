// ./src/main.rs

fn main() {
	let p = x::Formulas::new_from([
		x::Formula("a_ce_dir", "12^{k-2}", |k| x::NumFrac::from_value(12f64.powi(k - 2))),
		x::Formula("a_center", "12^{k}", |k| x::NumFrac::from_value(12f64.powi(k))),
		x::Formula("a_ce_inv", "12^{k+2}", |k| x::NumFrac::from_value(12f64.powi(k + 2))),
		x::Formula("b_direct", "12^{k}·2^{-1}", |k| x::NumFrac::from_value(12f64.powi(k) * 2f64.powi(-1))),
		x::Formula("b_invert", "12^{k}·2^{+1}", |k| x::NumFrac::from_value(12f64.powi(k) * 2f64.powi(1))),
		x::Formula("c_direct", "12^{k-2}·3^{+2}", |k| x::NumFrac::from_value(12f64.powi(k - 2) * 3f64.powi(2))),
		x::Formula("c_invert", "12^{k+2}·3^{-2}", |k| x::NumFrac::from_value(12f64.powi(k + 2) * 3f64.powi(-2))),
	]);

	p.make((-4, 4)).print_table();
}
