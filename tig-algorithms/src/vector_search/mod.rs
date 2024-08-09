
pub mod guy_vec_search; // c004_a001
use std::time::Instant;

#[cfg(test)]
mod tests {
    use super::*;
    use tig_challenges::{vector_search::*, *};

    #[test]
    fn test_guy_vec_search() {
    let start = Instant::now();
    let difficulty = Difficulty {
        // -- vector search --
        num_queries: 50,
        better_than_baseline: 100,
    };
    let seed = 2; // change this to generate different instances
    let challenge = Challenge::generate_instance(seed, &difficulty).unwrap();
    let results = guy_vec_search::solve_challenge(&challenge);
    print!("{:?}", results);
    match results {
        Ok(Some(solution)) => match challenge.verify_solution(&solution) {
            Ok(_) => println!("Valid solution"),
            Err(e) => println!("Invalid solution: {}", e),
        },
        Ok(None) => println!("No solution"),
            Err(e) => println!("Algorithm error: {}", e),
        };
        let duration = start.elapsed();
        println!("Time elapsed: {:?}", duration);
    }
}
