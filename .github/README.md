<div align="center">

# yo
<img src="image.png">

<br>

yo gives you a nice lil message.

<br>

<img src="https://badgen.net/static/Made%20with/Rust/orange?icon=https://upload.wikimedia.org/wikipedia/commons/archive/d/d5/20160221220028%21Rust_programming_language_black_logo.svg">|
<img src="https://badgen.net/static/Awesomely/Does%20Nothing%20Important!/green?icon=awesome">
</div>

<br>

## Let me do it for you
<img src="https://badgen.net/static/Supported%20for/Linux/orange?icon=https://upload.wikimedia.org/wikipedia/commons/3/35/Tux.svg">|
<img src="https://badgen.net/static/Supported%20for/Mac/grey?icon=https://upload.wikimedia.org/wikipedia/commons/f/fa/Apple_logo_black.svg">

```bash
# For Bash shells
curl -s https://raw.githubusercontent.com/rhhen122/yo/refs/heads/master/.github/bash-install.sh | bash
```
```bash
# For Zsh shells
curl -s https://raw.githubusercontent.com/rhhen122/yo/refs/heads/master/.github/zsh-install.sh | zsh
```
made by roky henderson (:

tip: you can change the name it calls you in the /src/main.rs file!

note: if you wish to change the actual code then you will have to run `cargo install --path .`

## DIY - Do it Yourself
```bash
# first clone the repo with git
git clone https://github.com/rhhen122/yo.git ./yo
```
```bash
# Now inside the yo dir run the following
cargo install --path .
```
```bash
# Now if your on Mac then you will have to run the following
echo "export PATH=$PATH:~/.cargo/bin/" >> ~/.bashrc
# or for zsh
echo "export PATH=$PATH:~/.cargo/bin/" >> ~/.zshrc
```