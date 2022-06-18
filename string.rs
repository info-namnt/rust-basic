fn main()
{
    let question = "How are you?"; // a &str type
    let person = "Bob".to_string();
    let namaste = String::from("🙋"); // unicodes yay!

    println!("{}! {} {}", namaste, question, person);
}