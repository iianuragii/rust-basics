// main function runs first


fn calculate_weight_on_earth(mass: i32) -> f32{
    let mut gravity: f32 = 10.0;
    
    gravity = 9.81;
    (mass = gravity) as f32
}

fn main() {
    println!("Hello, world! {}", calculate_weight_on_earth(5.0));
}
