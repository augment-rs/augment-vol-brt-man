# augment-vol-brt-man

Manages volume and brightness in Hyprland.

## Installation

### Cargo

    # cargo install augment-vol-brt-man --version "0.1.0-alpha" --root /usr/local

> [!IMPORTANT]
> `augment-vol-brt-man` must be installed outside the user home directory due permission pupposes.

## Usage

You can just run `augment-vol-brt-man` right away but it won't start at boot. Add `exec-once = augment-vol-brt-man` to your `~/.config/hypr/hyprland.conf` file.

```
# ~/.config/hypr/hyprland.conf

...
exec-once = augment-vol-brt-man
...
```

You also have to add the following lines for it to work with your volume and brightness keys.

```
# ~/.config/hypr/hyprland.conf

...
# Volume and Brightness Control
bindel = , XF86AudioRaiseVolume, exec, augment-vol-brt-man volume-up
bindel = , XF86AudioLowerVolume, exec, augment-vol-brt-man volume-down
bindl = , XF86AudioMute, exec, augment-vol-brt-man volume-mute
bindl = , XF86MonBrightnessUp, exec, augment-vol-brt-man brightness-up
bindl = , XF86MonBrightnessDown, exec, augment-vol-brt-man brightness-down
...
```

### Extended Volume

Change the line `extended_volume = false` to `extended_volume = true` in `~/.config/augment/vol-brt-man.toml` to enable extended volume control.

## Requirements

Required commands:

- `wpctl`
- `brightnessctl`

## License

This is licensed under the GPL-3.0 License.
