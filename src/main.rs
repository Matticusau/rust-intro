fn main() {
    // let arguments = std::env::args().skip(1);
    //println!("Hello, world!");
    // cargo run -- hello world matt
    // for arg in arguments {
    //     println!("Got arg: {}", arg);
    // }
    let mut arguments = std::env::args().skip(1);
    let key = arguments.next().unwrap();
    let value = arguments.next().unwrap();
    // println!("Key: {:?} Value: {:?}", key, value);
    println!("Key: {} Value: {}", key, value);

    // find std library functions => https://doc.rust-lang.org/stable/std/
    // let write_result = std::fs::write("kv.db", "Key: {} Value: {}", key, value);
    let write_result = write_database(key, value);
    match write_result {
        Ok(()) => {
            println!("It worked!");
        }
        Err(the_error) => {
            println!("We got an error {}", the_error);
        }
    }
}

fn write_database(key: String, value: String) -> Result<(), std::io::Error>{
    let contents = format!("{} {}", key, value);
    return std::fs::write("kv.db", contents);
}