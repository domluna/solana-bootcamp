// This shopping list program isn't compiling!
// Use your knowledge of generics to fix it.

// Execute `rustlings hint generics1` for hints!

fn main() {
    let mut shopping_list: Vec<&'static str> = Vec::new();
    shopping_list.push("milk");
    shopping_list.push("eggs");
}