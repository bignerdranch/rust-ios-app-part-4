extern crate cheddar;

fn main() {
    cheddar::Cheddar::new()
        .expect("could not read manifest")
        .module("for_c")
        .expect("malformed module path")
        .run_build("copying_view_model.h");
}
