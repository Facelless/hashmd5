use std::collections::HashSet;

const HASH_VERIFY: &str = "098f6bcd4621d373cade4e832627b4f6";
const LETTERS: &str = "abcdefghijklmnopqrstuvwxyz0123456789";

fn md5(text: &str) -> String {
    format!("{:x}", md5::compute(text))
}

fn find_matching_hash() {
    for size in 1..10 {
        let p = LETTERS.len().pow(size as u32);
        println!("Gerando hashs para {} caracteres ({} possibilidades)", size, p);

        let mut visited_combinations = HashSet::new();

        fn generate_combinations(
            current_combination: String,
            size: usize,
            visited_combinations: &mut HashSet<String>,
        ) -> Option<String> {
            if current_combination.len() == size {
                let current_hash = md5(&current_combination);

                if current_hash == HASH_VERIFY {
                    return Some(current_combination);
                }
            } else {
                for letter in LETTERS.chars() {
                    let new_combination = format!("{}{}", current_combination, letter);
                    if !visited_combinations.contains(&new_combination) {
                        visited_combinations.insert(new_combination.clone());

                        if let Some(matching_combination) =
                            generate_combinations(new_combination, size, visited_combinations)
                        {
                            return Some(matching_combination);
                        }
                    }
                }
            }
            None
        }

        if let Some(matching_combination) =
            generate_combinations(String::new(), size, &mut visited_combinations)
        {
            println!("Hash encontrada: {}", matching_combination);
            return;
        }
    }
    println!("Nenhuma correspondência encontrada.");
}

fn main() {
    find_matching_hash();
}
