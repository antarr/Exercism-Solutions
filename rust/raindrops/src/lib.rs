pub fn raindrops(n: usize) -> String {
    let mut result = String::new();
    if n % 3 == 0 {
       result.push_str("Pling");
    }
    if n % 5 == 0 {
       result.push_str("Plang");
    }
    if n % 7 == 0 {
       result.push_str("Plong");
    }
    if result.is_empty(){
        let digits = format!("{}", n);
        result.push_str(&digits);
    }
    result
}

