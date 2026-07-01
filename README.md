# cosmic-dwt

A command-line tool to toggle the `disable_while_typing` (DWT) touchpad flag in the COSMIC Desktop environment configuration.

## Motivation

When playing games on a laptop using the built-in keyboard and touchpad, the desktop environment automatically disables the touchpad while keys are being pressed to prevent accidental input. However, many games require simultaneous keyboard and mouse inputs. 

This tool provides a programmatic way to disable the `disable_while_typing` flag when launching a game and restore the original user configuration once the game exits.

## Features

- Modifies the underlying COSMIC configuration directly on disk.
- Integrates with external daemons like `gamemoded` to automate state transitions.
- Supports storing a temporary save state to safely roll back changes.

## Usage

```text
Usage: cosmic-dwt [-v]... [-q]... COMMAND ...

Available options:
    -v, --verbose  Increase output verbosity, can be used several times
    -q, --quiet    Decrease output verbosity, can be used several times
    -h, --help     Print help information
    -V, --version  Print version information

Available commands:
    status         Check current status of disable-while-typing
    toggle         Toggle disable-while-typing state
    enable         Enable disable-while-typing
    disable        Disable disable-while-typing
    restore        Restore the previously saved disable-while-typing state
    help           Print help information
```

### Examples

Save the current configuration state and turn off DWT before launching a game:

```bash
cosmic-dwt disable --save
```

Restore the original configuration state and clear the temporary save file:

```bash
cosmic-dwt restore --delete
```

---

## Integrations

### Feral GameMode (`~/.config/gamemode.ini`)

To automate toggling when launching games via GameMode, add the binary commands to your configuration file:

```ini
[custom]
start=cosmic-dwt disable --save
end=cosmic-dwt restore --delete
```

---

## License

This repository contains components under different licenses:

* **`src/main.rs`** and the command-line interface logic are licensed under the [MIT License](LICENSE-MIT).
* **`src/config.rs`** contains configuration structures modified from `cosmic-comp` and is licensed under the [GPL-3.0-only License](LICENSE).

Because these components are compiled together into a single static executable, the resulting distributed binary is governed by the terms of the **GPL-3.0-only**.
