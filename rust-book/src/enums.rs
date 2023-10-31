#[derive(Debug)]
pub enum Structure {
    Square,
    Rect,
    Circle,
    Polygon(i32),
}

// Define your custom Err enum
#[derive(Debug, Clone)]
pub enum Erraneous {
    TooManySides,
    // Add other error types as needed
}

pub fn return_area(x: &Structure) -> Result<String, &'static str> {
    match x {
        Structure::Circle => Ok("pi r square".to_string()),
        Structure::Polygon(n) => {
            if *n > 10 {
                Err("The sides are too much for me")
            } else {
                Ok(format!("The polygon has {} sides", n))
            }
        }
        Structure::Rect => Ok("l*b".to_string()),
        Structure::Square => Ok("a square".to_string()),
    }
}

pub fn return_err_enum(x: &Structure) -> Result<String, Erraneous> {
    match x {
        Structure::Circle => Ok("pi r square".to_string()),
        Structure::Polygon(n) => {
            if *n > 10 {
                Err(Erraneous::TooManySides)
            } else {
                Ok(format!("The polygon has {} sides", n))
            }
        }
        Structure::Rect => Ok("l*b".to_string()),
        Structure::Square => Ok("a square".to_string()),
    }
}

pub fn return_opt_enum(x: &Structure) -> Option<String> {
    match x {
        Structure::Circle => Some("pi r square".to_string()),
        Structure::Polygon(n) => {
            if *n > 10 {
                None
            } else {
                Some(format!("The polygon has {} sides", n))
            }
        }
        Structure::Rect => Some("l*b".to_string()),
        Structure::Square => Some("a square".to_string()),
    }
}

/*
This file covers the following :
- enum : standard
- pattern matching : expect, unwrap_or and match 
- Return enums : Result and Option
- Formatting strings
- Using custom enum for dealing with Err

*/

pub fn start() {
    let shape = Structure::Circle;
    let bad = Structure::Polygon(17);
    let express = return_area(&shape).expect("Failed to get area expression");
    let me = return_err_enum(&shape).unwrap_or(String::from("This was something else, quite the connundrum"));
    println!("The expression of {:?} is {}", shape, express);
    println!("The expression of {:?} is {}", shape, me);

    let word = return_opt_enum(&bad).unwrap_or(String::from("bad result"));
    println!("The expression of {:?} is {}",bad, word);
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