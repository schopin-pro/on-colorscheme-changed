# On colorscheme changed

Rust-based service to listen on DBUS for color scheme changing (e.g. when toggling light/dark mode in the menubar in Gnome 42+),
and updating the theme for terminal emulators / terminal editors accordingly.

## Usage

Prerequisites:

  * Install rust and cargo. See [rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).

  * You'll need to be running a linux distribution which uses systemd, and where the desktop environment
    notifies about color scheme changing on D-Bus. Gnome does, KDE probably as well.

Use the `deploy.sh` script to build and deploy the program as a local user service.

## Blog post

The original author wrote a blog post about this [here](https://www.christianfosli.com/posts/2024-on-colorscheme-changed/).
