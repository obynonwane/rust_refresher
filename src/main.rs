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

struct Employee {
    name: String,
    assigned_id: u64,
    email: String,
    active: bool,
}

fn vectors() {
    println!("------------------------running vectors-------------------------------------\n");

    // create a new instance of Employee struct
    let emp1 = Employee {
        name: String::from("John Doe"),
        assigned_id: 1,
        email: String::from("biostechnologyng@gmail.com"),
        active: true,
    };

    // declare and initialize a vector
    let mut employees = Vec::new();
    // push item to vector
    employees.push(emp1);

    //changing an element
    if let Some(emp) = employees.get_mut(0) {
        emp.email = String::from("obinn@gmail.com");
    }

    // reading an element
    if let Some(emp) = employees.get(0) {
        print!("Employee Name: {}\n", emp.name);
        print!("Employee Email: {}\n", emp.email);
    }
}

enum catchType {
    LRU,
    MRU,
}

struct Cache {
    level: String,
    cacheType: catchType,
}

// c-type enum
enum StatusCode {
    Ok = 200,
    Badrequest = 400,
    NotFound = 404,
}

// enums with value
enum CacheStrategy {
    LRU(String),
    MRU(i32),
}
fn enums() {
    let lru = catchType::LRU;
    let mru = catchType::MRU;
}

fn while_loop() {
    println!("------------------------running while loops-------------------------------------\n");

    let mut num = 1;

    while num < 5 {
        num += 1;
        print!("current num is {num}\n")
    }

    print!("while executed and exited current number is:  {num}\n")
}

enum Web3 {
    Defi,
    NFT,
    Game,
    Metaverse,
}
fn number_assign(web3: Web3) -> u8 {
    match web3 {
        Web3::Defi => 1,
        Web3::NFT => 2,
        Web3::Game => 3,
        Web3::Metaverse => 4,
    }
}

fn main() {
    tuple();
    array();
    heap();
    execute_traits();
    slice();
    hashmap();
    vectors();
    while_loop();

    let defi = number_assign(Web3::Defi);
    assert_eq!(1, defi);
}
