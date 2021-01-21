fn main() {
    let person = Person::new(1, String::from("java"), Gender::Male, 26);
    println!("{:?}", person);
    let person = BuilderPerson::new()
        .id(2)
        .name(String::from("rust"))
        .gender(Gender::FeMale)
        .age(18)
        .build();
    println!("{:?}", person);
    let person = BuilderPerson::new()
        .id(3)
        .name(String::from("python"))
        .gender(Gender::Male)
        .age(20)
        .build();
    println!("{:?}", person);
}

#[derive(Debug, Clone)]
enum Gender {
    Male,
    FeMale,
}

#[derive(Debug)]
struct Person {
    id: usize,
    name: String,
    gender: Gender,
    age: u8,
}

impl Person {
    fn new(id: usize, name: String, gender: Gender, age: u8) -> Self {
        Person {
            id,
            name,
            gender,
            age,
        }
    }
}

struct BuilderPerson {
    id: Option<usize>,
    name: Option<String>,
    gender: Option<Gender>,
    age: Option<u8>,
}
impl BuilderPerson {
    fn new() -> Self {
        BuilderPerson {
            id: None,
            name: None,
            gender: None,
            age: None,
        }
    }
    fn id(&mut self, id: usize) -> &mut Self {
        self.id = Some(id);
        self
    }
    fn name(&mut self, name: String) -> &mut Self {
        self.name = Some(name);
        self
    }
    fn gender(&mut self, gender: Gender) -> &mut Self {
        self.gender = Some(gender);
        self
    }
    fn age(&mut self, age: u8) -> &mut Self {
        self.age = Some(age);
        self
    }
    fn build(&self) -> Person {
        Person {
            id: self.id.unwrap(),
            name: self.name.clone().unwrap(),
            gender: self.gender.clone().unwrap(),
            age: self.age.unwrap(),
        }
    }
}
