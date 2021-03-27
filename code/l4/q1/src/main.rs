fn main() {
    let red_traffic_light = TrafficLights::RED(30);
    let yellow_traffic_light = TrafficLights::YELLOW(3);
    let green_traffic_light = TrafficLights::GREEN(45);
    // let red_traffic_light = TrafficLights::RED(Lights{color:String::from("red"), time: 30});
    // let yellow_traffic_light = TrafficLights::YELLOW(Lights{color:String::from("yellow"), time: 3});
    // let green_traffic_light = TrafficLights::GREEN(Lights{color:String::from("green"), time: 45});
    println!("RED LIGHT, wait {}", red_traffic_light.get_time());
    println!("YELLOW LIGHT, wait {}", yellow_traffic_light.get_time());
    println!("GREEN LIGHT, wait {}", green_traffic_light.get_time())
}

// const RED_TIME:i32 = 30;
// const YELLOW_TIME:i32 = 3;
// const GREEN: i32 = 45;

enum TrafficLights {
    RED(i32),
    YELLOW(i32),
    GREEN(i32)
}

trait LightTime {
    fn get_time(&self) -> &i32;
}

impl LightTime for TrafficLights {
    fn get_time(&self) -> &i32 {
        match self {
            TrafficLights::RED(t) => t,
            TrafficLights::YELLOW(t) => t,
            TrafficLights::GREEN(t) => t
            // TrafficLights::RED => {
            //     println!("[RED] time: {}", &RED_TIME);
            //     RED_TIME
            // },
            // TrafficLights::YELLOW => {
            //     println!("[YELLOW] time: {}",&YELLOW_TIME);
            //     YELLOW_TIME
            // },
            // TrafficLights::GREEN => {
            //     println!("[GREEN] time: {}", &GREEN);
            //     GREEN
            // }
        }
    }
}

// 1
// #[derive(Debug)]
// struct Lights {
//     color: String,
//     time: i32
// }
//
// #[derive(Debug)]
// enum TrafficLights {
//     RED(Lights),
//     YELLOW(Lights),
//     GREEN(Lights)
// }
//
// trait LightTime {
//     fn get_time(&self) -> i32;
// }
//
// impl LightTime for TrafficLights {
//     fn get_time(&self) -> i32 {
//         match self {
//            TrafficLights::RED(t) => {
//                println!("[RED] color: {}, time: {}", &t.color, &t.time);
//                t.time
//            },
//            TrafficLights::YELLOW(t) => {
//                println!("[YELLOW] color: {}, time: {}", &t.color, &t.time);
//                t.time
//            },
//            TrafficLights::GREEN(t) => {
//                println!("[GREEN] color: {}, time: {}", &t.color, &t.time);
//                t.time
//            }
//         }
//     }
// }