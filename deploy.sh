#!/bin/sh

cargo build --release
cp target/release/on-colorscheme-changed $HOME/.local/bin
cp on-colorscheme-changed.service $HOME/.config/systemd/user
systemctl --user daemon-reload
systemctl --user enable on-colorscheme-changed
systemctl --user restart on-colorscheme-changed
