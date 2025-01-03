use std::time::Duration;
use std::thread;

// 定义交通灯状态
#[derive(Debug, PartialEq, Eq)]
enum TrafficLightState {
    Red,
    Green,
    Yellow,
}


// 定义事件枚举（这里只使用一个定时器事件）
#[derive(Debug)]
enum Event {
    TimerElapsed,
}

// 定义交通灯结构体
struct TrafficLight {
    state: TrafficLightState,
}

// 实现TrafficLight结构体
impl TrafficLight {
    // 创建一个新的交通灯，初始状态为红灯
    fn new() -> Self {
        TrafficLight{state: TrafficLightState::Red}
    }

    // 状态转换函数
    // 状态转换函数
    fn transition(&mut self, event: Event) {
        match (&self.state, event) {
            (TrafficLightState::Red, Event::TimerElapsed) => {
                self.state = TrafficLightState::Green;
                println!("Traffic Light turned GREEN");
            }
            (TrafficLightState::Green, Event::TimerElapsed) => {
                self.state = TrafficLightState::Yellow;
                println!("Traffic Light turned YELLOW");
            }
            (TrafficLightState::Yellow, Event::TimerElapsed) => {
                self.state = TrafficLightState::Red;
                println!("Traffic Light turned RED");
            }
            // 处理其他情况（可选）
            _ => println!("Invalid transition"),
        }
    }

    // 获取当前状态
    fn get_current_state(&self) -> &TrafficLightState {
        &self.state
    }
}

fn main() {
    // 创建交通灯实例
    let mut traffic_light = TrafficLight::new();

    // 模拟时间流逝
    let durations = [5, 3, 2]; // 红灯 5 秒，绿灯 3 秒，黄灯 2 秒
    //let mut duration_index = 0;

    loop {
        // 获取当前状态
        let current_state = traffic_light.get_current_state();
        println!("Current state: {:?}", current_state);

        // 根据当前状态获取持续时间
        let duration = match current_state {
            TrafficLightState::Red => durations[0],
            TrafficLightState::Green => durations[1],
            TrafficLightState::Yellow => durations[2],
        };

        // 等待指定的时间
        thread::sleep(Duration::from_secs(duration));

        // 触发定时器到时事件
        traffic_light.transition(Event::TimerElapsed);
    }
}
