pub mod entities;

use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "codeanalyzer",
    about = "Analyze java application.",
    version,
    long_about = None,
)]
pub struct CodeAnalyzer {
    /// Path to the project root directory
    #[arg(short = 'i', long = "input")]
    pub input: Option<PathBuf>,

    /// Paths to files to be analyzed from the input application
    #[arg(short = 't', long = "target-files")]
    pub target_files: Vec<PathBuf>,

    /// Analyze a single string of rust source code instead of the project
    #[arg(short = 's', long = "source-analysis")]
    pub source_analysis: Option<String>,

    /// Destination directory to save the output graphs.
    /// By default, the SDG formatted as a JSON will be printed to the console.
    #[arg(short = 'o', long = "output")]
    pub output: Option<PathBuf>,

    /// Custom build command. Defaults to auto build.
    #[arg(short = 'b', long = "build-cmd")]
    pub build: Option<String>,

    /// Do not build your application.
    /// Use this option if you have already built your application.
    #[arg(long = "no-build")]
    pub no_build: bool,

    /// Do not attempt to auto-clean dependencies
    #[arg(long = "no-clean-dependencies")]
    pub no_clean_dependencies: bool,

    /// Path to the root cargo.toml file of the project
    #[arg(short = 'f', long = "project-root-path")]
    pub project_root_pom: Option<PathBuf>,

    /// Level of analysis to perform.
    /// Options: 1 (for just symbol table) or 2 (for call graph)
    #[arg(short = 'a', long = "analysis-level", default_value = "1")]
    pub analysis_level: u8,

    /// Print logs to console
    #[arg(short = 'v', long = "verbose")]
    pub verbose: bool,
}

impl CodeAnalyzer {
    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Implementation would go here
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = CodeAnalyzer::parse();
    cli.run()
}
