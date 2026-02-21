<!-- # Tenki(天気) -->

<!-- tty-clock with weather effect written in Rust and powerd by [ratatui](https://github.com/ratatui-org/ratatui) and tenki means weather in japanese -->


# Jikan(時間)

_[**unofficial** fork from [Tenki(天気)](https://github.com/ckaznable/tenki)]_

tty-clock with customizable configuration written in Rust and powered by [ratatui](https://github.com/ratatui-org/ratatui)

> [!NOTE]
> Current project doesn't support all features available in Tenki.

<!-- ![demo](./doc/demo.gif) -->

## Installation


<!-- [![Packaging status](https://repology.org/badge/vertical-allrepos/tenki.svg)](https://repology.org/project/tenki/versions) -->

### Install from Cargo

```shell
cargo install --git https://github.com/maachue/jikan.git
```

## Authors

- [Maachue](https://github.com/maachue) - Config, font.
- [ckaznable](https://github.com/ckaznable) - Core.

<!-- ### Install from Source Code

tenki is written in Rust, so you'll need to grab a [Rust installation](https://www.rust-lang.org/) in order to compile it.

```shell
git clone https://github.com/ckaznable/tenki
cd tenki
make build
make install
```

If you want to uninstall

```shell
make uninsall
``` -->

<!-- ### Install from the AUR

If you are using Arch Linux, you can install tenki using an [AUR helper](https://wiki.archlinux.org/title/AUR_helpers). For example:

```shell
paru -S tenki
``` -->

## Usage

### CLI

CLI will override the configuration.

```
sorry, current in WIP
```

<!-- ```
Usage: tenki [OPTIONS]

Options:
      --mode <MODE>                [default: rain] [possible values: rain, snow, meteor, disable]
      --timer-mode <TIMER_MODE>    [possible values: dvd]
      --timer-color <TIMER_COLOR>  color of the effect. [red, green, blue] [default: white]
  -f, --fps <FPS>                  frame per second [default: 60]
  -t, --tps <TPS>                  tick per second [default: 60]
  -l, --level <LEVEL>              effect level, The lower, the stronger [4-1000]
      --wind <WIND>                wind mode. [random, disable, only-right, only-left, right, left] [default: random]
      --show-fps                   show fps at right-top in screen
      --blink-colon                blinking colon of timer
  -h, --help                       Print help
  -V, --version                    Print version
``` -->

### Config

```toml
[general]
format = "%Y-m-%d" # max length: 256 characters
hide_date = false
utc = false
show_seconds = false
format_12h = false
# color = "white"
# blink = false
# timer_mode = "none"
font = "digital" # default
# Some default font you can have: `digital` (from tty-clock), `tenki` (from tenki)

# [general.weather]
# mode = "rain" # [rain, snow,]
# wind = "random" # [random, disable, only-right, only-left, right,left]
# level = 60 # [0,1000]

[general.meridiem]
am = "[AM]"
pm = "[PM]"

[performance]
fps = 60
tps = 60

[fontconfig]
padding_width_between_digits = 1
padding_between_timer_and_date = 1

[font.YOUR_CUSTOM_FONT_NAME]
width_number = 5 # your number digit width
width_colon = 5 # your colon digit width
height = 3 # your digits' height

zero  = ["12345", "12345", "12345"]
two   = ["12345", "12345", "12345"]
three = ["12345", "12345", "12345"]
four  = ["12345", "12345", "12345"]
five  = ["12345", "12345", "12345"]
six   = ["12345", "12345", "12345"]
seven = ["12345", "12345", "12345"]
eight = ["12345", "12345", "12345"]
nine  = ["12345", "12345", "12345"]
colon = ["colon", "colon", "colon"]
```

## LICENSE

[MIT](./LICENSE)