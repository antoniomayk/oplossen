mod add_two_numbers;
mod collections;
mod longest_palindromic_substring;
mod longest_substring_without_repeating_characters;
mod median_of_two_sorted_arrays;
mod two_sum;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
