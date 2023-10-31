pub fn start() {
    let string1 = String::from("abcd");
    let string2 = String::from("xyz");
    let result: &str = longest(string1.as_str(), string2.as_str());
    println!("The longest string is {}", result);



}


/*

This covers traits, trait bounds, generic types and lifetimes

Keep in mind :
- Any function that returns a reference, needs to be dealt with lifetime annotations either explicitly (manual) or implicitly (ellision)
- To take a reference of a struct, we would need to have its lifetime annotation defined

Ellision rules : 
    1. Each parameter that is a reference gets its own lifetime parameter

    2. If there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters;

    3. If there are multiple input lifetime parameters, but one of them is &self or &mut self ( a mutable refernce to self ) the lifetime of self is assigned to all output lifetime parameters.

*/

pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = 2 + 2;
        start();
        assert_eq!(result, 4);
    }
}
