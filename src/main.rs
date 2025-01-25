use std::collections::HashMap;
pub struct Block {
    pub id: u64,
    pub hash: String,
    pub previous_hash: String,
    pub timestamp: i64,
    pub txn_date: String,
    pub nonce: u64,
}

pub struct Blockchain {
    pub blocks: Vec<Block>,
}
// const HOURS_IN_DAYS: i64 = 24;

fn tuple() {
    println!("------------------------running tuple-------------------------------------\n",);
    //Tuple declaration type one
    let tup = (10, 13.0, "a");
    let (x, y, z) = tup;

    println!("the value of x: {x}");
    println!("the value of y: {y}");
    println!("the value of z: {z}");

    // xplicit tuple declaration
    let x: (i32, u64, char) = (10, 67, 'w');
    let ten = x.0;
    let sixty_seven = x.1;
    println!("the value of ten be: {ten}\n",);
    println!("the value of sixty seven be: {sixty_seven}\n",);
}

fn array() {
    println!("------------------------running arrays-------------------------------------\n");
    let x = [2, 4, 8, 9];
    let y: [f64; 3] = [67.0, 8.9, 45.0];
    let months: [&str; 3] = ["jan", "feb", "mar"];

    let first = x[0];
    println!("the first value: {first}\n",);
    println!("the first month of the year: :{}\n", { months[0] });
}

fn heap() {
    let mut s = String::from("hello");
    s.push_str(", world");
    println!("say hello to the world obinna : :{}\n", { s });
}

trait Animal {
    fn speak(&self);
}

struct Dog;
struct Cat;

impl Animal for Dog {
    fn speak(&self) {
        print!("Dog can say bark!\n")
    }
}

impl Animal for Cat {
    fn speak(&self) {
        print!("Cat says meow!\n")
    }
}

fn make_sound(animal: &dyn Animal) {
    animal.speak();
}

fn execute_traits() {
    println!("------------------------running traits-------------------------------------\n");

    let cat = Cat;
    let dog = Dog;

    make_sound(&cat);
    make_sound(&dog);
}
fn slice() {
    println!("------------------------running slices-------------------------------------\n");
    let my_array = [1, 3, 6, 9, 10, 0];
    // sleice from index[2] to index[5]
    let my_slice = &my_array[2..6];

    print!("my slice is :{:?}\n", my_slice);
}

fn hashmap() {
    println!("------------------------running hashmap-------------------------------------\n");
    // declare a mutable hashmap
    let mut rgb = HashMap::new();
    rgb.insert(String::from("Blue"), 10);
    rgb.insert(String::from("Green"), 50);
    rgb.insert(String::from("Red"), 100);

    // querying the hashmap
    match rgb.get("Blue") {
        Some(&number) => print!("Blue:{}\n", number),
        None => print!("No value for Blue\n"),
    }

    // iterating over the hashmap
    for (key, value) in &rgb {
        print!("{key}: {value}\n")
    }
}
fn main() {
    tuple();
    array();
    heap();
    execute_traits();
    slice();
    hashmap();
}
