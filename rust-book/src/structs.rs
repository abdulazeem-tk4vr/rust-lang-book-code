use std::fmt::Debug;

#[derive(Debug, Clone)]
struct CollegeId {
    name: String,
    age: u8,
    id: String,
}

impl Default for CollegeId {
    fn default() -> Self {
        Self {
            name: (String::from("new member")),
            age: (99),
            id: (String::from("unavailable")),
        }
    }
}

pub fn start() {
    let mut student = CollegeId {
        name: (String::from("Abdul")),
        age: (21),
        id: (String::from("2019AAPS1234H")),
    };
    student.name = String::from("Omar");
    student.age = 22;
    student.id = String::from("2019XXPS1111H");

    let professor = CollegeId {
        name: String::from("Ahmed"),
        age: 43,
        ..student.clone()
    };

    println!("The student is {:#?}", student);
    println!("The professor is {:#?}", professor);

    let joinee = CollegeId::default();
    println!("The new joinee is {:#?}", joinee);
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
