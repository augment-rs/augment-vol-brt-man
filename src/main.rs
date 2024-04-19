use augment_vol_brt_man::{
    brightness::Brightness,
    config::Config,
    display::{display, init, DisplayType},
    volume::Volume,
};
use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    subcommand: Subcommands,
}

#[derive(Subcommand)]
enum Subcommands {
    Init,
    VolumeUp,
    VolumeDown,
    VolumeMute,
    BrightnessUp,
    BrightnessDown,
}

fn main() {
    let cli = Cli::parse();
    let config = Config::new();
    let mut volume = Volume::new(config.extended_volume);
    let mut brightness = Brightness::default();

    match cli.subcommand {
        Subcommands::Init => {
            init();
        }
        Subcommands::VolumeUp => {
            volume.increase_volume();

            display(
                DisplayType::Volume {
                    is_muted: volume.is_muted,
                    is_extended: volume.extended_volume,
                },
                volume.current_volume,
            );
        }
        Subcommands::VolumeDown => {
            volume.decrease_volume();

            display(
                DisplayType::Volume {
                    is_muted: volume.is_muted,
                    is_extended: volume.extended_volume,
                },
                volume.current_volume,
            );
        }
        Subcommands::VolumeMute => {
            volume.mute();

            display(
                DisplayType::Volume {
                    is_muted: volume.is_muted,
                    is_extended: volume.extended_volume,
                },
                volume.current_volume,
            )
        }
        Subcommands::BrightnessUp => {
            brightness.increase_brightness();

            display(DisplayType::Brightness, brightness.current_brightness)
        }
        Subcommands::BrightnessDown => {
            brightness.decrease_brightness();

            display(DisplayType::Brightness, brightness.current_brightness)
        }
    }
}
