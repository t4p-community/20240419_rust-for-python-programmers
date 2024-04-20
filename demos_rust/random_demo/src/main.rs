fn main() {
    let s1 = String::from("hello");
    let s2 = String::from("cool");
    let s3 = String::from("hello");

    if s1 == s2 {
        println!("s1 and s2 are equal");
    } else {
        println!("s1 and s2 are not equal");
    }

    if s1 == s3 {
        println!("s1 and s3 are equal");
    } else {
        println!("s1 and s3 are not equal");
    }
}
