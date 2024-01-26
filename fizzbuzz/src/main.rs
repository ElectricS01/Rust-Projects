fn main() {
    for i in 1..100 {
        let mut out =  String::new();

        if i % 3 == 0 {
            out = "Fizz".to_string();
        }
        if i % 7 == 0 {
            out = out + "Buzz";
        }
        if out == "" {
            println!("{}", i);
        } else {
            println!("{}", out);
        }
    }
}
