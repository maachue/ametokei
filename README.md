# Jikan(時間)

> [!WARNING]  
> Jikan archived, unmaintained. Move to [ametokei](https://github.com/maachue/ametokei) instead!


_[**unofficial** fork from [Tenki(天気)](https://github.com/ckaznable/tenki)]_

tty-clock with customizable configuration written in Rust and powered by [ratatui](https://github.com/ratatui-org/ratatui)

> [!NOTE]
> Current project doesn't support all features available in Tenki.

![demo](./doc/demo.gif)

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
Usage: jikan [OPTIONS]

Options:
  -t, --tps <TPS>
          tick per seconds [default: 60]
  -f, --fps <FPS>
          frame per seconds [default: 60]
      --date <DATE>
          set the date format [default: "%Y-%m-%d"]
  -u, --utc <UTC>
          use UTC time [possible values: true, false]
  -D, --hide-date <HIDE_DATE>
          hide date [possible values: true, false]
  -s, --show-seconds <SHOW_SECONDS>
          show seconds [possible values: true, false]
  -c, --center <CENTER>
          center of the terminal [possible values: true, false]
      --hour12 <HOUR12>
          set the hour in 12h format [possible values: true, false]
      --timer-color <TIMER_COLOR>
          color of the timer & date
      --config <CONFIG>
          custom config path
      --no-config
          no-config mode
      --generate-config [<GENERATE_CONFIG>]
          create config
  -h, --help
          Print help
  -V, --version
          Print version
```

### Config

UNIX: `~/.config/jikan/config.toml`  
Windows: `%APPDATA%\\maachue\\jikan\\config\\config.toml`

```toml
[general]
format = "%Y-m-%d" # max length: 256 characters
hide_date = false
utc = false
show_seconds = false
format_12h = false
color = "White"
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
spacing_width_between_digits = 1
spacing_between_timer_and_date = 1

[fonts.YOUR_CUSTOM_FONT_NAME]
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
