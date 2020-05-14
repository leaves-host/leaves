use rand::{distributions::Alphanumeric, Rng};
use std::iter;

pub fn random_string(n: usize) -> String {
    let mut rng = rand::thread_rng();

    iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .take(n)
        .collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_random_string() {
        for x in 0..10 {
            assert_eq!(x, super::random_string(x).len());
        }
    }
}
