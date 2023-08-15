fn main() {
    println!("Hello, world!");

    some_other_fn(10);
    value_w_units(10.0, "kg");
    statements_and_expressions();

    let x = five(); // use return value to initialize var x
    println!("{}",x);

    //let five_fact = factorial(3);
    //println!("{}",five_fact);

    let twenty_four_h_in_s = h_to_s(24);
    println!("{twenty_four_h_in_s}");
}

fn some_other_fn(x: i32){
    println!("Value is {x}");
}

fn value_w_units (value: f64, unit: &str) {
    println!("{} {}", value, unit);
}

fn statements_and_expressions() {

    let y = 6; // this doesnt evaluate to a value, this is a statement
    //fn is a statement too, entire preceding block is a stament

    //let x = let y = 6;
    // not supported, let y = 6 is a statement with no value, nothing for x to bind to

    //calling fn is an expression
    let a = some_other_fn(10);

    //calling a macro is an expression
    let b = println!("abc");

    //calling a curly brace scope is an expressoin
    let c = {
        let x = 3;
        x+4 // this expression evaluates to 7, notice no semicolon as its an expression
    };
    println!("{}", c);



}

fn five() -> i64 {
    5// returns this expression as val
}

/* 
fn factorial(val : u128) -> u128{
    let factorized: u128 = val;
    let one_less: u128 = val;
    loop{
        let one_less: u128 = one_less - 1; //loop starts at initial value again, thats the mistakte im doing
        if one_less <= 2 {
            return factorized;
        };
        println!("{one_less}");
        let factorized: u128 = factorized*one_less;
        println!("{factorized}");
    } 
} */

//idk how to implement this yet

fn h_to_s(h : u64) -> u64 {
    h * 3600 //adding semicolon here would give an error
}