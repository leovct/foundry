use clap::{Parser, ValueHint};
use eyre::Result;
use foundry_cli::utils::LoadConfig;
use foundry_config::impl_figment_convert_basic;
use std::{collections::BTreeMap, path::PathBuf};

/// CLI arguments for `forge remappings`.
#[derive(Clone, Debug, Parser)]
pub struct RemappingArgs {
    /// The project's root path.
    ///
    /// By default root of the Git repository, if in one,
    /// or the current working directory.
    #[arg(long, value_hint = ValueHint::DirPath, value_name = "PATH")]
    root: Option<PathBuf>,
    /// Pretty-print the remappings, grouping each of them by context.
    #[arg(long)]
    pretty: bool,
}
impl_figment_convert_basic!(RemappingArgs);

impl RemappingArgs {
    pub fn run(self) -> Result<()> {
        let config = self.load_config()?;

        if self.pretty {
            let mut groups = BTreeMap::<_, Vec<_>>::new();
            for remapping in config.remappings {
                groups.entry(remapping.context.clone()).or_default().push(remapping);
            }
            for (group, remappings) in groups {
                if let Some(group) = group {
                    sh_println!("Context: {group}")?;
                } else {
                    sh_println!("Global:")?;
                }

                for mut remapping in remappings {
                    remapping.context = None; // avoid writing context twice
                    sh_println!("- {remapping}")?;
                }
                sh_println!()?;
            }
        } else {
            for remapping in config.remappings {
                sh_println!("{remapping}")?;
            }
        }

        Ok(())
    }
}
