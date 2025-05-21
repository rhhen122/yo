cd ~
git clone http://github.com/rhhen122/yo.git ~/yo
cd ~/yo
cargo install --path .
echo "export PATH=$PATH:~/.cargo/bin/" >> ~/.zshrc