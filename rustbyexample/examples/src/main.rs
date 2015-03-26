// comment
use std::fmt;

fn main() {
    //_types_casting();
    //_types();
    //_variable_bindings_declare_first();
    //_variable_bindings_scope_and_shadowing();
    //_variable_bindings_mutability();
    //_variable_bindings();
    //_literals_and_operators();
    //_formatted_print_display();
    //_formatted_print_debug();
    //_formatted_print();
    //_hello_world();
}

fn _types_casting() {
    let decimal = 65.9321_f32;

    //let integer: u8 = decimal;
    
    let integer = decimal as u8;
    let character = integer as char;

    println!("Casting {} -> {} -> {}", decimal, integer, character);
}

fn _types() {
    let _a_float: f64 = 1.0;
    let mut _an_integer = 5_i32;
    //an_integer = true;
}

fn _variable_bindings_declare_first() {
    let a_binding;

    {
        let x = 2;
        a_binding = x * x;
    }

    println!("a binding: {}", a_binding);

    let another_binding;

    //println!("another binding: {}", another_binding);

    another_binding = 1;
    println!("another binding: {}", another_binding);
}

fn _variable_bindings_scope_and_shadowing() {
    let long_lived_binding = 1;

    {
        let short_lived_binding = 2;

        println!("inner short: {}", short_lived_binding);

        let long_lived_binding = 5_f32;

        println!("inner long: {}", long_lived_binding);
    }
    
    //println!("outer short: {}", short_live_binding);

    println!("outer long: {}", long_lived_binding);
}

fn _variable_bindings_mutability() {
    let _immutable_binding = 1;
    let mut mutable_binding = 1;

    println!("Before mutation: {}", mutable_binding);

    mutable_binding += 1;

    println!("After mutation: {}", mutable_binding);

    //_immutable_binding += 1;
}

fn _variable_bindings() {
    let an_integer = 1u32;
    let a_boolean = true;
    let unit = {};

    let copied_integer = an_integer;

    println!("An integer: {:?}", copied_integer);
    println!("A boolean: {:?}", a_boolean);
    println!("Meet the unit value: {:?}", unit);

    let _unused_variable = 3u32;

    let _noisy_unused_variabled = 2u32;
}

fn _literals_and_operators() {
    println!("1 + 2 = {}", 1u32 + 2);

    println!("1 - 2 = {}", 1i32 - 2);
    println!("1u32 - 2 = {}", 1u32 - 2);

    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    println!("One million is written as {}", 1_000_000u32);
}

#[derive(Debug)]
struct _MinMax(i64, i64);

impl fmt::Display for _MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Debug)]
struct _Point2 {
    x: f64,
    y: f64,
}

impl fmt::Display for _Point2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

fn _formatted_print_display() {
    let minmax = _MinMax(0, 14);

    println!("Compare structures:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range = _MinMax(-300, 300);
    let small_range = _MinMax(-3, 3);

    println!("The big range is {big} and the small is {small}", small = small_range, big = big_range);

    let point = _Point2 {x: 3.3, y: 7.2};

    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);
    
    //println!("Wha does Point2D look like in binary: {:b}?", point);
}

#[derive(Debug)]
struct _Structure(i32);

#[derive(Debug)]
struct _Deep(_Structure);

fn _formatted_print_debug() {
    println!("{:?} months in a year.", 12);
    println!("{1:?} {0:?} is the {actor:?} name.", "Slater", "Christian", actor="actor's");

    println!("Now {:?} will print!", _Structure(3));

    println!("Now {:?} will print!", _Deep(_Structure(7)));
}

fn _formatted_print() {

    println!("{} days", 31);
    println!("{} days", 31);
    
    println!("{0}, this is {1}. {1}, this is {0}", "Slubis", "Besikeiciantis");

    println!("{subject} {verb} {predicate}",
            predicate="over the lazy dog",
            subject="the quick brown fox",
            verb="jumps");

    println!("{} of {:b} people know binary, the other half don't", 1, 2);

    println!("My name is {0}, {1} {0}", "Bond", "James");

    //struct Structure(i32);

    //println!("This struct '{}' won't print...", Structure(3));
}

fn _hello_world() {
    // comment
    println!("Hello, world!");
}
