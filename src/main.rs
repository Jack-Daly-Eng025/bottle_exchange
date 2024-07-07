pub struct Solution;


impl Solution {
    pub fn num_water_bottles(mut num_bottles: i32, num_exchange: i32) -> i32 {
        let (mut drink_tot, mut bot_empty) = (0,0);
        while num_bottles > 0{
            drink_tot += num_bottles;
            bot_empty += num_bottles;
            num_bottles = bot_empty / num_exchange;
            bot_empty %= num_exchange;
        }
        drink_tot
    }
}

fn main() {
    let bottle_exchange = Solution::num_water_bottles(25, 60);
    println!("Hello, world! Here's the bottle exchange! {}", bottle_exchange);
}
