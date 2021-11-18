use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args[1]=="brightness" {
        let mut br_max = fs::read_to_string("/sys/class/backlight/intel_backlight/max_brightness").expect("Could not read file.");
        let mut br_actual = fs::read_to_string("/sys/class/backlight/intel_backlight/brightness").expect("Could not read file.");
        
        br_max.pop(); br_actual.pop();
        
        let br_max = br_max.parse::<u32>().unwrap();
        let br_actual = br_actual.parse::<u32>().unwrap();
        println!("Brightness is {}%", br_actual*100/br_max);
    
    } else if args[1]=="battery" {
        let mut bat_max = fs::read_to_string("/sys/class/power_supply/BAT1/charge_full").expect("Could not read file.");
        let mut bat_actual = fs::read_to_string("/sys/class/power_supply/BAT1/charge_now").expect("Could not read file.");
        
        bat_max.pop(); bat_actual.pop();
        
        let bat_max = bat_max.parse::<u32>().unwrap();
        let bat_actual = bat_actual.parse::<u32>().unwrap();
        println!("Battery is {}%", 100*bat_actual/bat_max);
    }
}
