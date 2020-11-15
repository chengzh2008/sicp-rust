// main needs to include all mod files otherwise compiler won't be able pick it up
mod build_abstraction_with_data;
mod build_abstraction_with_function;

use crate::build_abstraction_with_function::elements_of_programming;

fn main() {
    let _result = elements_of_programming::hello();
}
