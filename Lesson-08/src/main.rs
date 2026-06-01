trait Animal {
    fn speak(&self) {
        println!("The animal makes a sound.");
    }
}

struct Dog;
struct Cat;

impl Animal for Dog {
    fn speak(&self) {
        println!("Woof!");
    }
}

trait Pet : Animal {
    fn play(&self) {
        println!("The pet is playing.");
    }
}

impl Pet for Dog {}
impl Animal for Cat {}

fn interact_with_animal(animal: Box<dyn Animal>) {
    animal.speak();
}

fn create_dog() -> impl Animal {
    Dog
}

fn create_cat() -> impl Animal {
    Cat
}
fn paly_with_pet(pet: Box<dyn Pet>) {
    pet.play();
}

fn main() {
    let dog = create_dog();
    let cat = create_cat();
    let animals: Vec<Box<dyn Animal>> = vec![Box::new(dog), Box::new(cat)];

    for animal in animals {
        interact_with_animal(animal);
    }

    paly_with_pet(Box::new(Dog)); // This will work because `Dog` implements `Pet`.
}
