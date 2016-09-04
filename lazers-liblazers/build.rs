extern crate tango;
extern crate cheddar;

fn main() {
    tango::process_root().unwrap();

    cheddar::Cheddar::new().expect("could not read manifest")
        .run_build("include/lazers.h");
}
