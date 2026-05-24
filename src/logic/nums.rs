
// ./src/logic/nums.rs


// Funkcja pomocnicza do obliczania Największego Wspólnego Dzielnika (NWD)
pub fn nwd_gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a.abs()
}

// Funkcja pomocnicza do obliczania Najmniejszej Wspólnej Wielokrotności (NWW/LCM)
pub fn nww_lcm(a: i32, b: i32) -> i32 {
    if a == 0 || b == 0 {
        return 0;
    }
    (a * b).abs() / nwd_gcd(a, b)
}

// Bezpieczne potęgowanie dla i32 //bezsens
pub fn checked_pow(base: i32, exp: i32) -> i32 {
    if exp <= 0 {
        return 1;
    }
    base.pow(exp as u32)
}

// Prywatna funkcja pomocnicza do zapisu liczby zmiennoprzecinkowej w formacie naukowym
fn format_scientific(val: f64) -> String {
    let s = format!("{:.9E}", val);
    if let Some((mantissa, exponent)) = s.split_once('E') {
        let exp_parsed: i32 = exponent.parse().unwrap_or(0);
        let exp_formatted = format!("{:+03}", exp_parsed); // Znak i 2 cyfry wykładnika (np. -05, +00)
        format!("{}E{}", mantissa, exp_formatted)
    } else {
        s
    }
}

#[derive(Debug, Clone)]
pub struct NumFrac {
    pub num: i32,
    pub den: i32,
    pub fract: String,
    pub float: String,
    pub value: f64,
}

impl NumFrac {
    pub fn from_fract(num: i32, den: i32) -> Self {
        assert!(den != 0, "Mianownik nie może być zerem!");
        let value = num as f64 / den as f64;
        let fract = format!("{}/{}", num, den);
        let float = format_scientific(value);

        Self {
            num,
            den,
            fract,
            float,
            value,
        }
    }

    pub fn from_value(val: f64) -> Self {
        if val == 0.0 {
            return Self::from_fract(0, 1);
        }

        let max_denominator = i32::MAX;
        let mut m00: i32 = 1;
        let mut m01: i32 = 0;
        let mut m10: i32 = 0;
        let mut m11: i32 = 1;

        let mut x = val;
        loop {
            let a = x.floor();
            let a_i = a as i32;

            let next_den = m10.checked_mul(a_i).and_then(|p| p.checked_add(m11));
            let next_num = m00.checked_mul(a_i).and_then(|p| p.checked_add(m01));

            let (n, d) = match (next_num, next_den) {
                (Some(n), Some(d)) if d <= max_denominator => {
                    m01 = m00;
                    m00 = n;
                    m11 = m10;
                    m10 = d;
                    (m00, m10)
                }
                _ => break,
            };

            if (val - (n as f64 / d as f64)).abs() < 1e-12 {
                break;
            }
            if (x - a).abs() < 1e-12 {
                break;
            }
            x = 1.0 / (x - a);
        }

        let g = nwd_gcd(m00, m10);
        Self::from_fract(m00 / g, m10 / g)
    }
}

use std::collections::BTreeMap;

pub fn sequence<F, T>(k: (i32, i32), f: F) -> BTreeMap<i32, T>
where
    F: Fn(i32) -> T, // lub FnMut, jeśli potrzebujesz mutowalnej funkcji, albo FnOnce, jeśli funkcja jest wywoływana tylko raz
{
    let mut map = BTreeMap::new();
    for i in k.0.min(k.1)..=k.0.max(k.1) {
        map.insert(i, f(i));
    }
    map
}

