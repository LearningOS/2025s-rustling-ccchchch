fn trim_me(input: &str) -> String {
    input.trim().to_string() // Remove whitespace from both ends and convert to String
}

fn compose_me(input: &str) -> String {
    input.to_string() + " world!" // Concatenate " world!" to the input string
}

fn replace_me(input: &str) -> String {
    input.replace("cars", "balloons") // Replace "cars" with "balloons"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(replace_me("I think cars are cool"), "I think balloons are cool");
        assert_eq!(replace_me("I love to look at cars"), "I love to look at balloons");
    }
}
