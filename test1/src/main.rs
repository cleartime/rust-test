trait TrafficLights {
    fn get_time(&self) -> String {
        String::from("[Object]")
    }
}

struct Light {
    red: String,
    green: String,
    yellow: String,
}

impl TrafficLights for Light {
    fn get_time(&self) -> String {
        format!("red time is {}, green time is{}, yellow time is {}", self.red, self.green, self.yellow)
    }
}

fn main() {
    let traffic_lights = Light {
        red: String::from("10s"),
        green: String::from("20s"),
        yellow: String::from("30s"),
    };
    println!("{}", traffic_lights.get_time());
}