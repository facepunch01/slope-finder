fn main() {
    fn get_input() -> String {
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).expect("Failed");
        buffer
    }
    println!("put in top x");
    let top_x = get_input().trim().parse::<f32>().unwrap();
    println!("put in bottom x");
    let bottom_x = get_input().trim().parse::<f32>().unwrap();
    println!("put in top y");
    let top_y = get_input().trim().parse::<f32>().unwrap();
    println!("put in bottom y");
    let bottom_y = get_input().trim().parse::<f32>().unwrap();
    let x = top_x - bottom_x;
    let y = top_y - bottom_y;
    let divide = y / x;
    println!("the slope is {}", &divide.to_string());
}
