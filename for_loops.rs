fn main() {
	// does not include 10	
    print!("Normal ranges: ");
    for i in 0..10 {
        print!("{} ",i);
    }

    println!();

    print!("Inclusive ranges: ");
    	
	for i in 0..=10 {
       // counts till 10
       print!("{} ",i);
    }
}
