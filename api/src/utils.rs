use rand::{distributions::Alphanumeric, Rng};
use serde::Serialize;
use serde_json::json;
use std::iter;
use tide::Response;

pub fn random_string(n: usize) -> String {
    let mut rng = rand::thread_rng();

    iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .take(n)
        .collect()
}

pub fn response(status: u16, json: &impl Serialize) -> Response {
    Response::new(status).body_json(json).unwrap_or_else(|_| {
        Response::new(500)
            .body_string(
                json!({
                    "message": "Error making response, please try again",
                })
                .to_string(),
            )
            .set_header("Content-Type", "application/json")
    })
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
