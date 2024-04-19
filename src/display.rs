use gtk::{gdk::Screen, gdk_pixbuf::Pixbuf, gio::ApplicationFlags, prelude::*, *};
use gtk_layer_shell::{Edge, Layer, LayerShell};
use serde::{Deserialize, Serialize};
use std::{
    io::{Read, Write},
    os::unix::net::{UnixListener, UnixStream},
    path::PathBuf,
    sync::mpsc,
    thread,
    time::Duration,
};

use crate::assets;

#[cfg(debug_assertions)]
const SOCKET: &str = "/tmp/ind.nanashi.augment.vol-brt-man-debug.sock";

#[cfg(not(debug_assertions))]
const SOCKET: &str = "/tmp/ind.nanashi.augment.vol-brt-man.sock";

const LEVEL_WIDTH: i32 = 272;

#[derive(Serialize, Deserialize)]
struct SocketData {
    display_type: DisplayType,
    level_percentage: u32,
}

#[derive(Serialize, Deserialize)]
pub enum DisplayType {
    Volume { is_muted: bool, is_extended: bool },
    Brightness,
}

struct IconLocations {
    volume_max: PathBuf,
    volume_not_max: PathBuf,
    brightness_max: PathBuf,
    brightness_not_max: PathBuf,
    brightness_min: PathBuf,
    volume_muted: PathBuf,
    volume_extended: PathBuf,
}

impl IconLocations {
    fn get_icon_path(icon: &str) -> PathBuf {
        xdg::BaseDirectories::new()
            .unwrap()
            .get_config_file(format!("augment/icons/{icon}"))
    }

    fn initialize_files(&self) {
        xdg::BaseDirectories::new()
            .unwrap()
            .create_config_directory("augment/icons")
            .unwrap();

        if !self.volume_max.exists() {
            self.init_volume_max();
        }

        if !self.volume_not_max.exists() {
            self.init_volume_not_max();
        }

        if !self.brightness_max.exists() {
            self.init_brightness_max();
        }

        if !self.brightness_not_max.exists() {
            self.init_brightness_not_max();
        }

        if !self.brightness_min.exists() {
            self.init_brightness_min();
        }

        if !self.volume_muted.exists() {
            self.init_volume_muted();
        }

        if !self.volume_extended.exists() {
            self.init_volume_extended();
        }
    }

    fn init_volume_max(&self) {
        std::fs::write(&self.volume_max, assets::VOLUME_MAX_ICON).unwrap();
    }

    fn init_volume_not_max(&self) {
        std::fs::write(&self.volume_not_max, assets::VOLUME_NOT_MAX_ICON).unwrap();
    }

    fn init_brightness_max(&self) {
        std::fs::write(&self.brightness_max, assets::BRIGHTNESS_MAX_ICON).unwrap();
    }

    fn init_brightness_not_max(&self) {
        std::fs::write(&self.brightness_not_max, assets::BRIGHTNESS_NOT_MAX_ICON).unwrap();
    }

    fn init_brightness_min(&self) {
        std::fs::write(&self.brightness_min, assets::BRIGHTNESS_MIN_ICON).unwrap();
    }

    fn init_volume_muted(&self) {
        std::fs::write(&self.volume_muted, assets::VOLUME_MUTED_ICON).unwrap();
    }

    fn init_volume_extended(&self) {
        std::fs::write(&self.volume_extended, assets::VOLUME_EXTENDED_ICON).unwrap();
    }
}

impl Default for IconLocations {
    fn default() -> Self {
        Self {
            volume_max: Self::get_icon_path("volume_max_icon.png"),
            volume_not_max: Self::get_icon_path("volume_not_max_icon.png"),
            brightness_max: Self::get_icon_path("brightness_max_icon.png"),
            brightness_not_max: Self::get_icon_path("brightness_not_max_icon.png"),
            brightness_min: Self::get_icon_path("brightness_min_icon.png"),
            volume_muted: Self::get_icon_path("volume_muted_icon.png"),
            volume_extended: Self::get_icon_path("volume_extended_icon.png"),
        }
    }
}

fn setup_css(_: &Application) {
    let provider = CssProvider::new();

    provider.load_from_data(assets::STYLES).unwrap();

    StyleContext::add_provider_for_screen(
        &Screen::default().unwrap(),
        &provider,
        STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn decide_icon(
    display_type: &DisplayType,
    level_percentage: u32,
    icon_locations: &IconLocations,
) -> PathBuf {
    match display_type {
        DisplayType::Volume {
            is_muted: false, ..
        } => match level_percentage {
            0 => icon_locations.volume_muted.clone(),
            100 => icon_locations.volume_max.clone(),
            101..=150 => icon_locations.volume_extended.clone(),
            _ => icon_locations.volume_not_max.clone(),
        },
        DisplayType::Volume { is_muted: true, .. } => icon_locations.volume_muted.clone(),
        DisplayType::Brightness => match level_percentage {
            1 => icon_locations.brightness_min.clone(),
            100 => icon_locations.brightness_max.clone(),
            _ => icon_locations.brightness_not_max.clone(),
        },
    }
}

fn percentage_level_compute(level_percentage: u32, min: u32, max: u32) -> i32 {
    (((level_percentage - min) as f64 / max as f64) * LEVEL_WIDTH as f64).ceil() as i32
}

pub fn activate(application: &Application) {
    let window = ApplicationWindow::new(application);

    window.init_layer_shell();
    window.set_layer(Layer::Top);
    window.set_app_paintable(true);

    window.set_anchor(Edge::Bottom, true);
    window.set_layer_shell_margin(Edge::Bottom, 64);

    let item_locations = IconLocations::default();
    item_locations.initialize_files();

    let icon_pixbuf = Pixbuf::from_file(decide_icon(
        &DisplayType::Volume {
            is_muted: false,
            is_extended: false,
        },
        0,
        &item_locations,
    ))
    .unwrap();
    let icon = Image::from_pixbuf(Some(&icon_pixbuf));

    let level_box = Box::new(Orientation::Horizontal, 0);
    level_box.set_widget_name("volume-box");
    level_box.set_width_request(LEVEL_WIDTH);

    let level = Box::new(Orientation::Vertical, 0);
    level.set_widget_name("level");

    level_box.add(&level);

    let root = Box::new(Orientation::Horizontal, 12);
    root.set_widget_name("root");

    root.add(&icon);
    root.add(&level_box);

    window.add(&root);

    let (tx, rx) = mpsc::channel();
    let window_clone = window.clone();

    glib::timeout_add_local(Duration::from_secs(2), move || {
        if let Ok(1) = rx.try_recv() {
            // We only want to do this once.
            while rx.try_recv().is_ok() {}

            return glib::ControlFlow::Continue;
        }

        window.hide();

        glib::ControlFlow::Continue
    });

    let window = window_clone;

    listen(move |display_type, level_percentage| {
        let icon_pixbuf =
            Pixbuf::from_file(decide_icon(display_type, level_percentage, &item_locations))
                .unwrap();
        icon.set_from_pixbuf(Some(&icon_pixbuf));

        match display_type {
            DisplayType::Volume { is_extended, .. } => {
                const MIN: u32 = 0;
                let max: u32 = match is_extended {
                    true => 150,
                    false => 100,
                };

                if level_percentage == MIN {
                    level.set_widget_name("level-min");
                } else if level_percentage == max {
                    level.set_widget_name("level");
                    level.set_width_request(LEVEL_WIDTH);
                } else {
                    level.set_widget_name("level");
                    level.set_width_request(percentage_level_compute(level_percentage, MIN, max));
                }

                if level_percentage > 100 {
                    level.set_widget_name("level-extended");
                }
            }

            DisplayType::Brightness => {
                const MIN: u32 = 1;
                const MAX: u32 = 100;

                if level_percentage == MIN {
                    level.set_widget_name("level-min");
                } else if level_percentage == MAX {
                    level.set_widget_name("level");
                    level.set_width_request(LEVEL_WIDTH);
                } else {
                    level.set_widget_name("level");
                    level.set_width_request(percentage_level_compute(level_percentage, MIN, MAX));
                }
            }
        }

        window.show_all();

        tx.send(1).unwrap();
    });
}

fn listen<F>(f: F)
where
    F: Fn(&DisplayType, u32) + 'static,
{
    let server = UnixListener::bind(SOCKET)
        .or_else(|e| match e.kind() {
            std::io::ErrorKind::AddrInUse => {
                std::fs::remove_file(SOCKET).unwrap();

                UnixListener::bind(SOCKET)
            }
            _ => Err(e),
        })
        .unwrap();

    let (tx, rx) = mpsc::channel::<SocketData>();

    thread::spawn(move || {
        for stream in server.incoming() {
            let mut stream = stream.unwrap();
            let mut buf = vec![];

            stream.read_to_end(&mut buf).unwrap();
            let socket_data = bincode::deserialize::<SocketData>(&buf).unwrap();

            tx.send(socket_data).unwrap();
        }
    });

    glib::timeout_add_local(Duration::from_millis(1), move || {
        if let Ok(socket_data) = rx.try_recv() {
            f(&socket_data.display_type, socket_data.level_percentage);
        }

        glib::ControlFlow::Continue
    });
}

pub fn init() {
    #[cfg(not(debug_assertions))]
    let id = "ind.nanashi.augment.vol-brt-man";

    #[cfg(debug_assertions)]
    let id = "ind.nanashi.augment.vol-brt-man-debug";

    let application = Application::new(Some(id), ApplicationFlags::default());

    application.connect_startup(setup_css);

    application.connect_activate(activate);

    application.run_with_args::<&str>(&[]);
}

pub fn display(display_type: DisplayType, level_percentage: u32) {
    let socket_data = SocketData {
        display_type,
        level_percentage,
    };

    let mut client = UnixStream::connect(SOCKET).unwrap();
    let buf = &bincode::serialize(&socket_data).unwrap();
    client.write_all(buf).unwrap();
}
