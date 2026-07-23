fn main() {
    let s1 = String::from("Ankit");
    let s2 = String::from("Ankit Lingwal");
    let s3 = longest(&s1, &s2);
    print!("Longest string is {}", s3);
}

fn longest<'a>(x: &'a str, h: &'a str) -> &'a str {
    if x.len() > h.len() { x } else { h }
}

// the function get smaller or overlapping lifetime than the input lifetime, so we need to specify the lifetime of the output as well. In this case, we are saying that the returned reference will live at least as long as both input references.

//Rules

// 1. Each parameter that is a reference gets its own lifetime parameter.
// fn foo<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {}

// 2. If there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters.
// fn foo<'a>(x: &'a str) -> &'a str {}

// 3. If there are multiple input lifetime parameters, but one of them is &self or &mut self, the lifetime of self is assigned to all output lifetime parameters.
// impl Foo {
//     fn foo<'a>(&self, x: &'a str) -> &'a str {
//         x
//     }
// }
