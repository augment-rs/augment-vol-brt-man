use crate::utils::run_command;

const SET_BRIGHTNESS: &str = "brightnessctl set {}%";
const GET_BRIGHTNESS: &str = r"brightnessctl set +0 | sed -En 's/.*\(([0-9]+)%\).*/\1/p'";

pub struct Brightness {
    pub current_brightness: u32,
}

impl Brightness {
    pub fn new() -> Self {
        Self {
            current_brightness: Self::get_current_brightness(),
        }
    }

    pub fn get_current_brightness() -> u32 {
        let output = String::from_utf8(run_command(GET_BRIGHTNESS).stdout);

        output.unwrap().trim().parse::<u32>().unwrap()
    }

    pub fn increase_brightness(&mut self) {
        self.current_brightness = clamp_brightness(self.current_brightness as i32 + 5) as u32;

        run_command(&SET_BRIGHTNESS.replace("{}", &self.current_brightness.to_string()));
    }

    pub fn decrease_brightness(&mut self) {
        self.current_brightness = clamp_brightness(self.current_brightness as i32 - 5) as u32;

        run_command(&SET_BRIGHTNESS.replace("{}", &self.current_brightness.to_string()));
    }
}

fn clamp_brightness(brightness: i32) -> i32 {
    brightness.clamp(1, 100)
}

impl Default for Brightness {
    fn default() -> Self {
        Self::new()
    }
}
