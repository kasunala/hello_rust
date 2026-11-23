//my hello world program
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

}