#[derive(Debug)]
struct Animal {
    age: u8,
    animal_type: AnimalType,
}

#[derive(Debug)]
enum AnimalType {
    Cat,
    Dog,
}

impl Animal {
    fn new() -> Self {
        // THIS is Static/associated method
        // Self means Animal.
        //You can also write Animal instead of Self
        Self {
            // When we write Animal::new(), we always get a cat that is 10 years old
            age: 10,
            animal_type: AnimalType::Cat,
        }
    }

    fn change_to_dog(&mut self) {
        // THIS is regular method
        // because we are inside Animal, &mut self means &mut Animal
        // use .change_to_dog() to change the cat to a dog
        // with &mut self we can change it
        println!("Changing animal to dog!");
        self.animal_type = AnimalType::Dog;
    }
}

fn main() {
    let mut new_animal = Animal::new(); // Associated method to create a new animal
                                        // It is a cat, 10 years old
    new_animal.change_to_dog();
    println!("animal: {:?}", new_animal);

    // destructuring
    let Animal {
        age: a,
        animal_type: b
    } = new_animal;
        println!("animal age: {}, type: {:?}", a, b);

}
