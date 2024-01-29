// if3.rs
//
// Execute `rustlings hint if3` or use the `hint` watch subcommand for a hint.

// I AM DONE

pub fn animal_habitat(animal: &str) -> &'static str {
    let identifier = if animal == "crab" {
        "Beach"
    } else if animal == "gopher" {
        "Burrow"
    } else if animal == "snake" {
        "Desert"
    } else {
        "Unknown"
    };

    // Adjust the comparisons to match strings
    let habitat = if identifier == "Beach" {
        "Beach"
    } else if identifier == "Burrow" {
        "Burrow"
    } else if identifier == "Desert" {
        "Desert"
    } else {
        "Unknown"
    };

    habitat
}


// No test changes needed.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gopher_lives_in_burrow() {
        assert_eq!(animal_habitat("gopher"), "Burrow")
    }

    #[test]
    fn snake_lives_in_desert() {
        assert_eq!(animal_habitat("snake"), "Desert")
    }

    #[test]
    fn crab_lives_on_beach() {
        assert_eq!(animal_habitat("crab"), "Beach")
    }

    #[test]
    fn unknown_animal() {
        assert_eq!(animal_habitat("dinosaur"), "Unknown")
    }
}
