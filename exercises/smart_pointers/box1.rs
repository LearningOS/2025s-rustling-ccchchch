#[derive(PartialEq, Debug)]
pub enum List {
    Cons(i32, Box<List>),  // Use Box to store the next element in the list on the heap
    Nil,
}

fn main() {
    println!("This is an empty cons list: {:?}", create_empty_list());
    println!(
        "This is a non-empty cons list: {:?}",
        create_non_empty_list()
    );
}

// Creates an empty list (Nil)
pub fn create_empty_list() -> List {
    List::Nil
}

// Creates a non-empty list with a single element
pub fn create_non_empty_list() -> List {
    List::Cons(1, Box::new(List::Nil))  // Box the Nil value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_empty_list() {
        assert_eq!(List::Nil, create_empty_list())
    }

    #[test]
    fn test_create_non_empty_list() {
        assert_ne!(create_empty_list(), create_non_empty_list())
    }
}
