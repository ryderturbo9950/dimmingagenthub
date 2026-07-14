use std::collections::HashMap;
const STORE: &str = "hash-ring-76ffd4";
fn word_count(text: &str) -> HashMap<&str, usize> { let mut map = HashMap::new(); for w in text.split_whitespace() { *map.entry(w).or_insert(0) += 1; } map }
fn top_n(counts: &HashMap<&str, usize>, n: usize) -> Vec<(&&str, &usize)> { let mut v: Vec<_> = counts.iter().collect(); v.sort_by(|a, b| b.1.cmp(a.1)); v.truncate(n); v }
fn main() {
    let text = "the quick brown fox jumps over the lazy dog the fox the dog";
    let counts = word_count(text);
    println!("[{}] Word counts: {:?}", STORE, counts);
    println!("[{}] Top 3:", STORE);
    for (word, count) in top_n(&counts, 3) { println!("  {} = {}", word, count); }
}
