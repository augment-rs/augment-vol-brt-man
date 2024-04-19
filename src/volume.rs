use crate::utils::run_command;

const SET_VOLUME_COMMAND: &str = "wpctl set-volume @DEFAULT_AUDIO_SINK@ {}%";
const MUTE_COMMAND: &str = "wpctl set-mute @DEFAULT_AUDIO_SINK@ toggle";
const GET_VOLUME_COMMAND: &str = "wpctl get-volume @DEFAULT_AUDIO_SINK@ | sed 's/[^0-9]//g'";
const GET_IS_MUTED_COMMAND: &str = "wpctl get-volume @DEFAULT_AUDIO_SINK@ | sed 's/[^MUTED]//g'";

pub struct Volume {
    pub extended_volume: bool,
    pub current_volume: u32,
    pub is_muted: bool,
}

impl Volume {
    pub fn new(extended_volume: bool) -> Self {
        Self {
            extended_volume,
            current_volume: Self::get_current_volume(),
            is_muted: Self::is_muted(),
        }
    }

    pub fn is_muted() -> bool {
        String::from_utf8(run_command(GET_IS_MUTED_COMMAND).stdout)
            .unwrap()
            .trim()
            .eq("MUTED")
    }

    pub fn get_current_volume() -> u32 {
        let output = String::from_utf8(run_command(GET_VOLUME_COMMAND).stdout).unwrap();

        output.trim().parse::<u32>().unwrap()
    }

    pub fn increase_volume(&mut self) {
        self.current_volume += 5;

        self.current_volume = clamp_volume(self.current_volume as i32, self.extended_volume) as u32;

        run_command(&SET_VOLUME_COMMAND.replace("{}", &self.current_volume.to_string()));
    }

    pub fn decrease_volume(&mut self) {
        // Fix bug that causes the volume to change to 32768.
        let mut current_volume_i32 = self.current_volume as i32;
        current_volume_i32 -= 5;

        self.current_volume = clamp_volume(current_volume_i32, self.extended_volume) as u32;

        run_command(&SET_VOLUME_COMMAND.replace("{}", &current_volume_i32.to_string()));
    }

    pub fn set_volume(&mut self, volume: u32) {
        self.current_volume = volume;

        run_command(&SET_VOLUME_COMMAND.replace("{}", &volume.to_string()));
    }

    pub fn mute(&mut self) {
        self.is_muted = !self.is_muted;

        run_command(MUTE_COMMAND);
    }
}

impl Default for Volume {
    fn default() -> Self {
        Self::new(false)
    }
}

fn clamp_volume(volume: i32, extended_volume: bool) -> i32 {
    let max = if extended_volume { 150 } else { 100 };

    volume.clamp(0, max)
}
