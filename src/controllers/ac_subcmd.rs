////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use anyhow::Context;
use anyhow::Result as anyResult;
use clap::ArgMatches;
use clap_generate::generators::*;
use colored::*;
use std::process::exit;

////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use crate::utils::cli::{
  completion::print_completions,
  help::cli_chapulin,
};

////////////////////////////////////////////////////////////////////////////////////////////////////

// error handler
use crate::error::config_error::ChapulinConfigError;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn ac_subcmd(matches: &ArgMatches) -> anyResult<()> {
  let _subcmd = "AC";

  ////////////////////////////////////////////////////////////////////////////////////////////////////

  // collect settings
  let manual = matches.is_present("manual");

  let shell = matches
    .value_of("shell")
    .context(ChapulinConfigError::TODO)?;

  ////////////////////////////////////////////////////////////////////////////////////////////////////

  if manual {
    match shell {
      "bash" => {
        print!(
          "\n{sh}\n{cmd}\n\n{cmt}\n{lx}\n{lxcmd}\n\n{os}\n{oscmd}\n",
          sh = "Bash:".blue().bold(),
          cmd = "$ source <(chapulin AC --shell bash)".cyan(),
          cmt = "# To load completions for each session, execute once:"
            .yellow()
            .dimmed(),
          lx = "Linux:".green(),
          lxcmd = "$ chapulin AC --shell bash > /etc/bash_completion.d/chapulin".cyan(),
          os = "MacOS:".green(),
          oscmd = "$ chapulin AC --shell bash > /usr/local/etc/bash_completion.d/chapulin".cyan(),
        )
      }

      "elvish" => {
        print!(
          "\n{}\n{}\n",
          "Documentation not available. Apologies.".green(),
          "Come back soon...".cyan(),
        )
      }

      "fish" => {
        print!(
          "\n{sh}\n\n{cmd1}\n\n{cmt}\n{cmd2}\n",
          sh = "Fish:".blue().bold(),
          cmd1 = "$ chapulin AC --shell fish | source".cyan(),
          cmt = "# To load completions for each session, execute once:"
            .yellow()
            .dimmed(),
          cmd2 = "$ chapulin AC --shell fish > ~/.config/fish/completions/chapulin.fish".cyan(),
        )
      }

      "powershell" => {
        print!(
          "\n{sh}\n\n{cmd1}\n\n{cmt1}\n{cmd2}\n\n{cmt2}\n",
          sh = "Powershell:".blue().bold(),
          cmd1 = "PS> chapulin AC --shell powershell | Out-String | Invoke-Expression".cyan(),
          cmt1 = "# To load completions for every new session, run:"
            .yellow()
            .dimmed(),
          cmd2 = "PS> chapulin AC --shell powershell > chapulin.ps1".cyan(),
          cmt2 = "# and source this file from your powershell profile"
            .yellow()
            .dimmed(),
        )
      }

      "zsh" => {
        print!(
          "\n{sh}\n\n{cmt1}\n{cmt2}\n{cmd1}\n\n{cmt3}\n{cmd2}\n\n{cmt4}\n",
          sh = "Zsh:".blue().bold(),
          cmt1 = "# If shell completion is not already enabled in your environment you will need"
            .yellow()
            .dimmed(),
          cmt2 = "# to enable it.  You can execute the following once:"
            .yellow()
            .dimmed(),
          cmd1 = "$ echo \"autoload -U compinit; compinit\" >> ~/.zshrc".cyan(),
          cmt3 = "# To load completions for each session, execute once:"
            .yellow()
            .dimmed(),
          cmd2 = "$ chapulin AC --shell zsh > \"${fpath[1]}/_chapulin\"".cyan(),
          cmt4 = "# You will need to start a new shell for this setup to take effect"
            .yellow()
            .dimmed(),
        )
      }

      _ => panic!("Unknown generator"),
    }

    exit(0);
  }

  ////////////////////////////////////////////////////////////////////////////////////////////////////

  let mut app = cli_chapulin();

  match shell {
    "bash" => print_completions::<Bash>(&mut app),
    "elvish" => print_completions::<Elvish>(&mut app),
    "fish" => print_completions::<Fish>(&mut app),
    "powershell" => print_completions::<PowerShell>(&mut app),
    "zsh" => print_completions::<Zsh>(&mut app),
    _ => panic!("Unknown generator"),
  }

  ////////////////////////////////////////////////////////////////////////////////////////////////////

  Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////
