pub fn power_set<T: Clone>(set: &[T]) -> Vec<Vec<T>> {
    let mut result = vec![Vec::new()];
    for item in set {
        for subset in result.clone() {
            let mut new_subset = subset.clone();
            new_subset.push(item.clone());
            result.push(new_subset);
        }
    }
    result
}

pub fn progress_size(domain_size: usize, set_size: usize) -> u64 {
    return (domain_size as u64).pow(set_size as u32) as u64;
}
