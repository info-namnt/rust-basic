fn main() {
    let mut numbers = [1,2,3,4];

        let all = &numbers[..];
        println!("All of them: {:?}", all);

        let first_two: &mut [u8] = &mut numbers[0..2];
        first_two[0] = 100;
        first_two[1] = 99;
        println!("First two of them: {:?}", first_two);

    println!("Look ma! I can modify through slices: {:?}", numbers);
}