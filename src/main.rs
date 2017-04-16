extern crate turtle;
extern crate itertools;

use turtle::{Canvas, Turtle};
use itertools::join;


fn main() {

    let iterations = 6;

    let a = "-BF+AFA+FB-";
    let b = "+AF-BFB-FA+";

    let mut dir = a.to_string();
    for _ in 0..iterations {
        dir = join(dir.split("")
                      .map(|x| match x {
                          "A" => a,
                          "B" => b,
                          _ => x,
                      }),
                   "");
    }

    let input = dir.replace("A", "").replace("B", "");

    let mut t = Canvas::new();
    for c in input.split("") {
        match c {
            "-" => t.left(90.0),
            "+" => t.right(90.0),
            "F" => t.forward(100.0),
            _ => (),
        }
    }

    t.save_svg(&mut std::io::stdout()).unwrap();
}
