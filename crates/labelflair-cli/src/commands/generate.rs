//! Generate the labels and write them to a file
//!
//! This command generates labels based on the configuration file and writes them to the specified
//! path. If no path is specified, the labels will be written to the current working directory as
//! `labels.yml`.

use std::path::PathBuf;

use clawless::prelude::*;
use kawauso_config::AncestorsSearch;
use kawauso_config::Loader;
use kawauso_config::error::LoadConfigurationError;
use labelflair::Labelflair;
use labelflair::config::v1::ConfigV1;
use labelflair::label::Label;

/// The name of the application whose configuration is loaded
///
/// The search looks for a file that has the name of the application and the extension `.toml`,
/// which means that this constant determines that the configuration file is called
/// `labelflair.toml`.
const APPLICATION: &str = "labelflair";

/// The subdirectory that the search reads in addition to each directory itself
///
/// Repositories keep the configuration of their tools in `.github`, and Labelflair's own GitHub
/// Action reads `.github/labelflair.toml` by default. The search reads the directory itself first,
/// so a project that keeps the file in its root still wins.
const SUBDIRECTORY: &str = ".github";

/// Generate the labels and write them to a file
///
/// This command generates labels based on the configuration file and writes them to a file. The
/// location of the file can be specified using the `--path` argument. If no path is specified, the
/// labels will be written to the current working directory as `labels.yml`.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Args)]
struct GenerateArgs {
    /// The path to the configuration file
    ///
    /// If no path is given, Labelflair searches the current working directory and its ancestors
    /// for a `labelflair.toml` file, in the directory itself and in `.github`, and uses the first
    /// one that it finds.
    #[clap(short, long)]
    config: Option<PathBuf>,
    /// The path to which the generated labels should be written
    #[clap(default_value = "labels.yml")]
    path: Option<PathBuf>,
}

/// Generate the labels and write them to a file
///
/// This function reads the configuration file specified in the arguments, generates the list of
/// labels, and writes them either to the specified path or to the default location.
#[command]
async fn generate(args: GenerateArgs, _context: Context) -> CommandResult {
    let config = load_config(args.config)?;
    let labels = Labelflair::generate(&config);

    write_labels(labels, args.path);

    Ok(())
}

/// Loads the configuration for Labelflair
///
/// This function loads the configuration from the given path. If no path is given, it searches the
/// current working directory and its ancestors for a `labelflair.toml` file, and loads the first
/// one that it finds. Each directory is searched before its `.github` subdirectory, and both are
/// searched before the directory above.
///
/// # Errors
///
/// Returns an error if no configuration file can be found or read, or if its contents are not a
/// valid configuration for Labelflair.
fn load_config(path: Option<PathBuf>) -> Result<ConfigV1, LoadConfigurationError> {
    let loader = match path {
        Some(path) => Loader::path(path),
        None => Loader::ancestors(AncestorsSearch::new(APPLICATION).subdirectory(SUBDIRECTORY)),
    };

    loader.load()
}

/// Write the generated labels to the specified path
///
/// This function takes a vector of labels and writes them to the specified path. If no path is
/// specified, it defaults to writing the labels to `labels.yml` in the current working directory.
fn write_labels(labels: Vec<Label>, path: Option<PathBuf>) {
    // Determine the output path
    let output_path = path.unwrap_or_else(|| PathBuf::from("labels.yml"));

    // Serialize the labels to YAML format
    let yaml_content = serde_yaml_ng::to_string(&labels).expect("failed to serialize labels");

    // Write the YAML content to the specified file
    std::fs::write(&output_path, yaml_content).expect("failed to write labels to file");

    println!("Labels written to {}", output_path.display());
}
