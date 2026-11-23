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
    //integer signed: i32,i64 unsigned u32, u64
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

    //let arr = [1,2,3,4,5,6,7,8,9,10];
    //println!("arr is {:?}",arr)
}