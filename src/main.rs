use std::fs::{File, OpenOptions, remove_file};
use std::io;
use std::io::{Read, Write};

fn main() {
    //let mut file = File::create("src/example.txt").expect("create failed");
    //file.write_all("Hello World!\n".as_bytes()).expect("write failed");
    /* let mut file = OpenOptions::new()
        .append(true)
        .open("src/example.txt")
        .expect("cannot open file");
    file.write_all("Adding content to the file. \n".as_bytes()).expect("failed");*/

   /* let mut file = File::open("src/example.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}", contents);

    remove_file("src/example.txt").expect("failed");
    */
    //let v =  vec![1, 2, 3]:
    //v[10];
    //panic!("Something went wrong, cannot proceed");
    //Recoverable errors

    //let f = File::open("main.jpg").unwrap();
    //let f = File::open("main.jpg").expect("Unable to open file"); // use this instead of match. Its basically the same
    /*match f {
        Ok(f) => {
          println!("file found {:?}", f);
        },
        Err(e) =>  {
            println!("File not found \n {:?}",e);
        }
    }
    println!("Continuing on with the execution");*/

  /*  divide(Some(1));
    divide(Some(10));
    divide(None);
    divide(Some(0));

    */let a = read_username_from_file();
    println!("{:?}",a);
}
fn read_username_from_file() -> Result<String,io::Error> {
    let mut f = File::open("src/usernames.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)


}
/*const ANSWER_TO_LIFE: i32 = 42;
fn divide(x: Option<i32>){
    match x {
        Some(0) => panic!("cannot divide by zero"),
        Some(x) => println!("result is {}", ANSWER_TO_LIFE / x),
        None => println!("No value introduced. The answer is {}", ANSWER_TO_LIFE)
    }*/

/*fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("src/usernames.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}*/