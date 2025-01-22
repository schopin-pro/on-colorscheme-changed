use futures_util::stream::StreamExt;
use serde::Deserialize;
use std::{collections::HashMap, ffi::OsStr, path::PathBuf};
use tokio::fs;
use anyhow::Result;
use ashpd::desktop::settings::{ColorScheme, Settings};

use dirs::config_dir;

fn dotconfig(subpath: &str) -> PathBuf {
    let mut path = config_dir().unwrap();
    path.push(subpath);
    path
}

fn normalize_path(path: &PathBuf) -> PathBuf {
    if path.is_relative() {
        let mut p = dirs::home_dir().unwrap();
        p.push(path);
        p
    } else {
        path.clone()
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let settings = Settings::new().await?;

    println!("Connected to dbus! Waiting for changes...");

    let prop = settings.read::<ColorScheme>("org.freedesktop.appearance", "color-scheme").await?;

    let config: Config = toml::from_str(&fs::read_to_string(dotconfig("theme-switcher.toml")).await?)?;
    on_colorscheme_changed(prop, config.links.values()).await?;

    while let Some(Ok(scheme)) = settings
        .receive_setting_changed_with_args::<ColorScheme>(
            "org.freedesktop.appearance",
            "color-scheme",
        )
        .await?
        .next()
        .await {
            on_colorscheme_changed(scheme, config.links.values()).await?
    }

    Ok(())
}

type Fallible = Result<()>;

#[derive(Deserialize)]
struct Link {
    symlink: PathBuf,
    light: PathBuf,
    dark: PathBuf,
    touch: Option<PathBuf>
}

#[derive(Deserialize)]
struct Config {
    links: HashMap<String, Link>
}

impl Link{
    async fn update(&self, cs: ColorScheme) -> Fallible {
        let wanted = match cs {
            ColorScheme::PreferDark => &self.dark,
            ColorScheme::PreferLight  => &self.light,
            ColorScheme::NoPreference  => &self.light,
        };

        let symlink = normalize_path(&self.symlink);

        let mut tmp = symlink.clone();
        tmp.set_extension("tmp");
        fs::symlink(&wanted, &tmp).await.unwrap();
        fs::rename(&tmp, &symlink).await.unwrap();
        if let Some(touch) = &self.touch {
            touch_file(touch)?;
        }
        Ok(())
    }
}

fn touch_file(filename: &PathBuf) -> Fallible {
    let now = chrono::Local::now();
    let times = std::fs::FileTimes::new()
        .set_accessed(now.into())
        .set_modified(now.into());
    std::fs::File::open(normalize_path(filename))?.set_times(times)?;
    Ok(())
}

async fn on_colorscheme_changed(cs: ColorScheme, links: impl Iterator<Item = &Link>) -> Fallible {
    println!("Colorscheme changed: {cs:?}");

    for link in links {
        link.update(cs).await?;
    }
    Ok(())
}
