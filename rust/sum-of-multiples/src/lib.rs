pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    
    let mut multiples = Vec::new();

    for num in 1..limit+1 {
        for factor in factors {
            if num % factor == 0{
                multiples.push(num);
            }
        }
    }

    let mut sum = 0u32;
    for multiple in multiples.iter() {
        sum += multiple;
    }
    sum
}

