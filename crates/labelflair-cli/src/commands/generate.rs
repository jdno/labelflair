//! Generate the labels and write them to a file
//!
//! This command generates labels based on the configuration file and writes them to the specified
//! path. If no path is specified, the labels will be written to the current working directory as
//! `labels.yml`.

use std::path::{Path, PathBuf};

use clawless::prelude::*;
use labelflair::Labelflair;
use labelflair::config::v1::ConfigV1;
use labelflair::label::Label;

/// Generate the labels and write them to a file
///
/// This command generates labels based on the configuration file and writes them to a file. The
/// location of the file can be specified using the `--path` argument. If no path is specified, the
/// labels will be written to the current working directory as `labels.yml`.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Args)]
struct GenerateArgs {
    /// The path to the configuration file
    #[clap(short, long, default_value = "labelflair.toml")]
    config: PathBuf,
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
    let config = load_config(&args.config);
    let labels = Labelflair::generate(&config);

    write_labels(labels, args.path);

    Ok(())
}

/// Load the configuration from the specified path
///
/// This function reads the configuration file at the given path and deserializes it into the
/// configuration struct. If the file cannot be read or parsed, the function will panic with an
/// error message.
fn load_config(path: &Path) -> ConfigV1 {
    // Read the file at the given path
    let config_content =
        std::fs::read_to_string(path).expect("failed to read the configuration file");

    // Deserialize the content into a ConfigV1 object
    toml::from_str(&config_content).expect("failed to parse the configuration file")
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
