#[cfg(feature = "selfupdate")]
use anyhow::Result;

#[cfg(feature = "selfupdate")]
pub fn run_command_config_backgroundselfupdate(value: Option<i64>) -> Result<()> {
    use crate::operations::{install_background_selfupdate, uninstall_background_selfupdate};
    use crate::config_file::{load_mut_config_db, save_config_db, load_config_db};
    use anyhow::{bail, Context};


    match value {
        Some(value) => {
            if value < 0 {
                bail!("Invalid argument.");
            }

            let mut config_file = load_mut_config_db()
                .with_context(|| "`config` command failed to load configuration data.")?;
    
            let mut value_changed = false;

            let value = if value==0 {None} else {Some(value)};

            if value != config_file.data.settings.background_selfupdate_interval {
                config_file.data.settings.background_selfupdate_interval = value;

                value_changed = true;

                match value {
                    Some(value) => {
                        install_background_selfupdate(value).unwrap();
                    },
                    None => {
                        uninstall_background_selfupdate().unwrap();
                    }
                }
            }

            save_config_db(&mut config_file)
                .with_context(|| "Failed to save configuration file from `config` command.")?;

            if value_changed {
                eprintln!("Property 'backgroundselfupdateinterval' set to '{}'", match value {
                    Some(value) => value,
                    None => 0
                });
            }
            else {
                eprintln!("Property 'backgroundselfupdateinterval' is already set to '{}'", match value {
                    Some(value) => value,
                    None => 0
                });
            }
        },
        None => {
            let config_data = load_config_db()
                .with_context(|| "`config` command failed to load configuration data.")?;

            eprintln!(
                "Property 'backgroundselfupdateinterval' set to '{}'",
                match config_data.settings.background_selfupdate_interval {
                    Some(value) => value,
                    None => 0
                }
            );
        }
    };

    Ok(())
}
