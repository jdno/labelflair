use clap::Args;
use clawless::command;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Args)]
struct LabelsYmlArgs {}

#[command]
async fn labels(args: LabelsYmlArgs) {}
