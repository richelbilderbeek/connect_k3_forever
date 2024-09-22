# Connect K3 Forever

[![Check build](https://github.com/richelbilderbeek/bevy_tdd_book_add_player_sprite_with_texture/actions/workflows/check_build.yaml/badge.svg?branch=master)](https://github.com/richelbilderbeek/bevy_tdd_book_add_player_sprite_with_texture/actions/workflows/check_build.yaml)
[![Check links](https://github.com/richelbilderbeek/bevy_tdd_book_add_player_sprite_with_texture/actions/workflows/check_links.yaml/badge.svg?branch=master)](https://github.com/richelbilderbeek/bevy_tdd_book_add_player_sprite_with_texture/actions/workflows/check_links.yaml)
[![Check markdown](https://github.com/richelbilderbeek/bevy_tdd_book_add_player_sprite_with_texture/actions/workflows/check_markdown.yaml/badge.svg?branch=master)](https://github.com/richelbilderbeek/bevy_tdd_book_add_player_sprite_with_texture/actions/workflows/check_markdown.yaml)
[![Check Rust style](https://github.com/richelbilderbeek/bevy_tdd_book_add_player_sprite_with_texture/actions/workflows/check_rust_style.yaml/badge.svg?branch=master)](https://github.com/richelbilderbeek/bevy_tdd_book_add_player_sprite_with_texture/actions/workflows/check_rust_style.yaml)
[![Check spelling](https://github.com/richelbilderbeek/bevy_tdd_book_add_player_sprite_with_texture/actions/workflows/check_spelling.yaml/badge.svg?branch=master)](https://github.com/richelbilderbeek/bevy_tdd_book_add_player_sprite_with_texture/actions/workflows/check_spelling.yaml)
[![Measure code coverage](https://github.com/richelbilderbeek/bevy_tdd_book_add_player_sprite_with_texture/actions/workflows/measure_codecov.yaml/badge.svg?branch=master)](https://github.com/richelbilderbeek/bevy_tdd_book_add_player_sprite_with_texture/actions/workflows/measure_codecov.yaml)
[![codecov](https://codecov.io/gh/richelbilderbeek/bevy_tdd_book_add_player_sprite_with_texture/graph/badge.svg?token=XAVFZYDQKZ)](https://codecov.io/gh/richelbilderbeek/bevy_tdd_book_add_player_sprite_with_texture)

## History

In 2007, the first version of this game was called 'K3-Op-Een-Rij' (literally: 'K3 on a row')
honoring the Dutch connection of the band.

The [first versions](https://github.com/richelbilderbeek/K3OpEenRijVcl)
were programming in C++ Builder and used the Borland's VCL library,
due to which the game could only be played on Windows.

Around 2015, the game was rewritten and
[those later versions](https://github.com/richelbilderbeek/K3OpEenRij)
using Qt Creator using the Qt library, due to which the game
became platform independent.

## Setup

Setup is done as described in [the Bevy 'getting started' guide](https://bevyengine.org/learn/quick-start/getting-started/setup/):

```bash
git clone https://github.com/richelbilderbeek/bevy_tdd_book_add_player_sprite_with_texture
cd bevy_tdd_book_add_player_sprite_with_texture
cargo init
cargo add bevy
```

To [Cargo.toml](Cargo.toml) add:

```bash
# Enable a small amount of optimization in debug mode
[profile.dev]
opt-level = 1

# Enable high optimizations for dependencies (incl. Bevy), but not for our code:
[profile.dev.package."*"]
opt-level = 3
```

To [.cargo/config.toml](.cargo/config.toml) add:

```bash
[target.x86_64-unknown-linux-gnu]
linker = "clang"
rustflags = ["-C", "link-arg=-fuse-ld=lld"]
```


## Files used by continuous integration scripts

Filename                                  |Descriptions
------------------------------------------|--------------------------------------------------------------------------------------------------------------------------------------
[mlc_config.json](mlc_config.json)        |Configuration of the link checker, use `markdown-link-check --config mlc_config.json --quiet docs/**/*.md` to do link checking locally
[.spellcheck.yml](.spellcheck.yml)        |Configuration of the spell checker, use `pyspelling -c .spellcheck.yml` to do spellcheck locally
[.wordlist.txt](.wordlist.txt)            |Whitelisted words for the spell checker, use `pyspelling -c .spellcheck.yml` to do spellcheck locally
[.markdownlint.jsonc](.markdownlint.jsonc)|Configuration of the markdown linter, use `markdownlint "**/*.md"` to do markdown linting locally. The name of this file is a default name.
[.markdownlintignore](.markdownlintignore)|Files ignored by the markdown linter, use `markdownlint "**/*.md"` to do markdown linting locally. The name of this file is a default name.
