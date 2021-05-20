/*
Copyright 2021 Volt Contributors
Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at
    http://www.apache.org/licenses/LICENSE-2.0
Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/

//! Clone and setup a repository from Github.

// Std Imports
use std::process;
use std::sync::Arc;

// Library Imports
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use colored::Colorize;

// Crate Level Imports
use crate::utils::App;
use crate::VERSION;

// Super Imports
use super::Command;

struct Clone {}

#[async_trait]
impl Command for Clone {
    /// Display a help menu for the `volt add` command.
    fn help() -> String {
        format!(
            r#"volt {}
    
Clone a project and setup a project from a repository.
Usage: {} {} {} {}
Options: 
    
  {} {} Output verbose messages on internal operations.
  {} {} Disable progress bar."#,
            VERSION.bright_green().bold(),
            "volt".bright_green().bold(),
            "clone".bright_purple(),
            "[repository]".white(),
            "[flags]".white(),
            "--verbose".blue(),
            "(-v)".yellow(),
            "--no-progress".blue(),
            "(-np)".yellow()
        )
    }

    async fn exec(app: Arc<App>) -> Result<()> {
        let exit_code = process::Command::new("git")
            .arg(format!("clone {} --depth=1", app.args[2]).as_str())
            .status()
            .unwrap();

        if exit_code.success() {
            process::Command::new("volt")
                .arg("install")
                .spawn()
                .unwrap();
        } else {
            anyhow!("Failed to Clone Repository");
        }
        Ok(())
    }
}