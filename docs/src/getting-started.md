![Project's logo](https://raw.githubusercontent.com/SkwalExe/rsfrac/main/assets/logo.png)

<p align="center">💠 The Terminal-Based Fractal Explorer 💠</p>

::: warning Windows Compatibility 🪟
Rsfrac **CAN NOT** be compiled to run natively on windows 🙁. But you can make it work under [WSL](https://en.wikipedia.org/wiki/Windows_Subsystem_for_Linux) ([how to install WSL](https://learn.microsoft.com/en-us/windows/wsl/install)).
:::

## How to install 📥 {#installation}

::: info `cargo` not found?

Cargo is the package manager for rust projects, it can be installed with:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
:::


To install build dependencies:
```bash
sudo apt install build-essentials m4
```

From [crates.io](https://crates.io/):

```bash
cargo install rsfrac
```

From the [Github repo](https://github.com/SkwalExe/rsfrac)

```bash
cargo install --git https://github.com/SkwalExe/rsfrac
```


## Starting the app

::: info Caution
If the command is not found after installation, you may need to add `~/.cargo/bin` to your path. You can do this by adding `export PATH=$PATH:~/.cargo/bin` to your `.bashrc` or `.zshrc` file. You will also need to **open a new shell session**.

```bash
# For bash
echo 'export PATH=$PATH:~/.cargo/bin' >> ~/.bashrc && bash
# For zsh
echo 'export PATH=$PATH:~/.cargo/bin' >> ~/.zshrc && zsh
```

:::

Now you can start the application with this command:

```bash
rsfrac
```

![Preview](https://raw.githubusercontent.com/SkwalExe/rsfrac/main/assets/banner.png)
