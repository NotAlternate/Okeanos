# Ωκεανός (Okeanos) : Shell done wrong.

![GitHub top language](https://img.shields.io/github/languages/top/NotAlternate/Okeanos?color=2A68B0&style=flat-square)
![GitHub repo size](https://img.shields.io/github/repo-size/NotAlternate/Okeanos?color=2A68B0&style=flat-square)
![GitHub commit activity](https://img.shields.io/github/commit-activity/m/NotAlternate/Okeanos?color=2A68B0&style=flat-square)

Okeanos is a shell project for fun, not aimed to replace your current shell.

<p align="center"><img alt="Okeanos banner" src="./banner.png"></p>

*Preview of Okeanos' logo and default prompt running: echo "Hello, GitHub!"*

## Prerequisites

If you are running a Windows NT system, you are unable to use Okeanos, support for it is unlikely to happen.<br>
However, if you are using MacOS or (any) Linux (distro), you can continue the installation process.

It is recommended to install the Rust programming language via ([Rustup](https://rustup.rs))

- Rust programming language
    - If you want to replace your default shell to Okeanos: <br>
    Do not follow the instructions of the next item and proceed.

    - If you want to run Okeanos directly from your shell: <br>
        Try configuring your current shell to include Cargo's binaries in the `PATH` variable (if it wasn't automatically included by `rustup`) by including the following:
        `export PATH="$PATH:$HOME/.cargo/bin"`
        In your shell's configurations
- `git` or `gh` ([Github CLI](https://cli.github.com/)) <br> *\* not required if you download the repository via Web or Desktop*

## Installation (unstable)

Currently, there are no stable releases of Okeanos, cloning the repository directly will give you the in-development or unstable version.
Proceed if you are willing to face consequences.

#### CLONING, BUILDING AND INSTALLING

1. Clone the repository via any option in the "**<> Code**" tab, which can be found on the top of the GitHub repository's page.
2. Run the following command in the `./Okeanos/` directory (The cloned repository, replace if you cloned under a different name).
```sh
$ cargo build --release
$ cargo install --path .
```
3. Okeanos should be compiled as an executable inside the `./target/release/` directory and installed to your Cargo binaries.

#### CHANGING DEFAULT SHELL (OPTIONAL)

\* Not recommended for you to change the default shell after following the unstable installation.

Note that changing your default shell to Okeanos requires you to have `sudo` or root access.

1. Adding a new entry for Okeanos inside of `/etc/shells`
```sh
sudo echo "$HOME/.cargo/bin/okeanos" >> /etc/shells      # Normal user
echo "$HOME/.cargo/bin/okeanos" >> /etc/shells           # Root user

# CLI Editors :: Append "$HOME/.cargo/bin/okeanos" to the opened file
# Note that you have to replace $HOME with your exact home directory.
sudo nano /etc/shells     # nano
sudo vim /etc/shells      # vim

# $HOME example: /home/notalternate
#  Prepending result: /home/notalternate/.cargo/bin/okeanos
```

2. Change the default shell using `chsh`.
3. In `chsh`, Enter your password and Input: `$HOME/.cargo/bin/okeanos` (Example can be seen above)
4. Restart or Create a new shell session and your default shell should change to Okeanos.

## Executable details

Okeanos is an interactive shell, meaning you cannot make Okeanos parse a shell file. 

After compiling and installing Okeanos, running the interactive shell directly from your current shell is simple:

```
okeanos
```

Note that in first-time running or if important Okeanos files were deleted, Okeanos will prompt you.

### Built-in commands

`--version`, `-v`: Outputs installed Okeanos version.

### Okeanos usage

You can use Okeanos like any typical shells, however some implementations might break some commands
because Okeanos is not aimed to replace your shell, so do not complain about that.

## Contributing

You can support the project by contributing to it via forking the repository, commit changes and opening up a pull request (Instructions on how to fork it can be read [here](https://docs.github.com/en/get-started/quickstart/contributing-to-projects)), leaving a feedback or reporting an issue.

After a pull request, please be patient for me to review and manage conflicts.

## Repository license

The Okeanos shell repository owned by NotAlternate is licensed under the [MIT](LICENSE) license.