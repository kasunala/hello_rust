//my hello world program

const Y:i32 = 10;
fn main() {
    /*multi 
    line comments 
    */
    //println!("Hello, world!");//this line prints
    
    //Declaring variables
    let mut x = 10; //use mut to mutate variable
    println!("x is {}",x);
    x = 20;
    println!("x is {}",x);

    tut_run()
}

fn tut_run(){

    //scalar data types
    //integer signed: i32,i64 unsigned u32,u64
    let x:i32 = -10;
    let y:u64 =  20;

    println!("x is signed {}",x);
    println!("y is un-signed {}",y);

    //float
    //f32, f64
    let pi:f32 = 3.1415926;
    println!("value of pi is {}",pi);

    //boolean values
    let is_snowing: bool = true;
    println!("is it snowing? {}",is_snowing);

    //charachter type , char

    let letter:char = 'a';
    println!("first letter of the alphabet is: {}",letter);

    //compound data types
    //arrays,tuples,slices and strings (slice string)

    let numbers: [i32; 5] = [1,2,3,4,5];
    println!("arr is {:?}",numbers); //debug format {!?}

    let fruits:[&str;3] = ["Apple", "Banana", "Orange"];
    println!("fruits array: {:?}",fruits);
    println!("fruit 1: {}",fruits[0]);
    println!("fruit 2: {}",fruits[1]);
    println!("fruit 3: {}",fruits[2]);

    //tuples
    //let human: (&str, i32, bool) = ("Kasun", 37, false);
    let human: (String, i32, bool) = ("Kasun".to_string(), 37, false);
    println!("human {:?}",human);

    let mix_tuple = ("Dawn",23,[1,2,3,4,5]);
    println!("mix tuple {:?}",mix_tuple);

    //Slices: [1,2,3,4], memory efficient
    let number_slices: &[i32] = &[1,2,3,4,5];
    println!("number slice {:?}",number_slices);

    //String slices, stack - quicker, memory efficient but immutable
    let animal_slices: &[&str] = &["Lion","Cat","Dog"];
    println!("animal slice {:?}",animal_slices);

    //String reference
    let book_slices: &[&String] = &[&"Fiction".to_string(),&"Novel".to_string(),&"Romance".to_string()];
    println!("book slice {:?}",book_slices);

    //Strings Vs String slices (&str)

    //Strings [growable, mutable, owned string type] - Heap
    let mut stone_cold: String = String::from("Hell, ");
    println!("Stone clod says {}", stone_cold);
    stone_cold.push_str("Yea!");
    println!("Stone clod says {}", stone_cold);

    //&str (String slice) - Stack
    let string = String::from("hello world");
    let slice: &str = &string[0..5];
    println!("slice value {}",slice);

    //Functions
    //snake case my_function

    hello_world();
    tell_height(160);
    human_id("Kasun",37,180.52);

    //code block

    let _b: i32 = {
      let price:i32 = 10;
        let units:i32 = 50;
        price * units
    };

    println!("code block results: {}",_b);

    let y = add(1,3);
    println!("y is {}",y);

    let weight = 70.0;
    let height:f64 = 1.5;

    let bmi:f64 = calculate_bmi(weight,height);
    println!("bmi is {}",bmi);


    //1- Each value of rust has a variable that's its owner
    let s1 =String::from("RUST");
    let len = calculate_length(&s1); //passing reference of s1

    println!("length is {}",len);

    //2- There can be only one owner at a time
    //trasfering ownership, now you cannot print s1 becasue it is owned by s2
    let s2 = s1;
    println!("s2 {}",s2);

    //3- When owner goes out of scope, the value will be dropped
    //s2 cannot be accessed ot it is out of scope outside the main function

    //Creating references
    let mut _x = 5;
    let _r = &mut _x; //referencing to original owner(_x) of value(5)

    //Increment value

    *_r += 1;


    println!("_x is {}",_x); //x is mutable if reference as mutable, 1 mutable reference you can change to _r here without using another println!
    //println!("_r is {}",_r); // you can only have 1 mutable reference and many immutable referances

    //Constants
    //are immutable and cannot be mutable
    //Should be captital, screaming snake case
    //can be defined in outside main, in gloabal scope

    //const Y:i32 = 10;
    println!("Y is {}",Y);

    //Shadowing
    //Second variable defined will overshadow the first variable and compiler will see only the second variable
    let x1 = 5;
    let x1 = x1+2; //second x1 overshadowing the first variable

    {
        let x1 = x1 * 2;
        println!("x1 {}",x1);
    }

    //Control flows in Rust
    //1-Conditions [if...Else]
    //2-Repeating actions [Loops]

    //if expression
    //
    // let age: u16 = 18;
    //
    // if age >= 18 {
    //     println!("You are old enough to drive!");
    // } else {
    //     println!("You are not enough to drive!");
    // }
    //

    //else if
    let number = 6;

    if number % 4 == 0 {
      println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else {
        println!("number is not divisible by 4 or 3"); //arms
    }

    //using if in let statement
    let condition = true;
    let number = if condition {5} else {6};
    println!("number is {number}");
    //data types in each arms should be same to be compatible with number variable

    //Loops
    //Loop
    //while
    //for

    //Loop
    let mut counter = 0;

   let results = loop {
        println!("in loop");

        counter += 1;

        if counter == 10 {
            break counter*2; //value of expression after break will return
        }
    };
     println!("results is {results}");

    //Loop labels
    //break and continue statement applies to innermost loop by default

        let mut count = 0;
        'counting_up: loop {   //'counting_up: label
            println!("count = {count}");
            let mut remaining = 10;

            loop {
                println!("remaining = {remaining}");
                if remaining == 9 {
                    break;
                }
                if count == 2 {
                    break 'counting_up;
                }
                remaining -= 1;
            }

            count += 1;
        }
        println!("End count = {count}");

    //while loops
    let mut number = 3;
    while number !=0{
        println!("number is {}",number);
        number -= 1;
        //break;
    }

    //for loop
    let a = [10, 20, 30, 40, 50];

    for elements in a {
        println!("elements are {}",elements);
    }





}

//hoisting means the function can be defined anywhere regardless of the main function
fn hello_world() {
    println!("Hello, world!");
}

fn tell_height(height:u32){
    println!("my height is {} cm",height);
}

fn human_id(name:&str, age:u32, height: f32){
    println!("my name is {} and age {}, height {} cm",name,age,height);
}

//Expression and Statement
//Expression: Anything that returns a value
//Statement: Anything that does not return a value

//Expressions
//5
//true for fals
//add(1,2)
//if conditions
//code block

//function with exp.
fn add(a:i32, b:i32) -> i32 {
    a+b
}

//Statements
//Always ends with ; like variable declarations , functional definitions, control flow statements

fn calculate_bmi(weight_kg: f64, height_m: f64)->f64{
    weight_kg/(weight_kg + height_m)

}

fn calculate_length(s: &String) -> usize{//output unassigned size
    s.len()
}

//Ownership, borrowing and references

//Ownership
//c and c++ -> memory management control issues
//garbage collector solve this but with cost of slow performance , it pauses the program

//3 rules of ownership
//1- Each value of rust has a variable that's its owner
//2- There can be only one owner at a time
//3- When owner goes out of scope, the value will be dropped

//Reference and borrowing
//Safety and performance

//Understanding references
//References: enable you to borrow value without taking ownership
// there are immutable and mutable references
//Create reference by adding "&"

//Immutable and Mutable borrow scenario
//Bank account structure, to check balance(immutable) and to withdraw balance(mutable)
//Simultanious imutable and mutable borrowing not allowed in same scope.

//Constants
