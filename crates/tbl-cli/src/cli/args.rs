use super::subcommands::*;
use crate::TablCliError;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

pub(crate) async fn run_cli() -> Result<(), TablCliError> {
    match Cli::parse().command {
        // read
        Commands::Ls(args) => ls_command(args).await,
        Commands::Schema(args) => schema_command(args).await,
        Commands::Cat(args) => cat_command(args).await,
        Commands::Head(args) => head_command(args).await,
        Commands::Tail(args) => tail_command(args).await,
        Commands::Count(args) => count_command(args).await,
        // edit
        Commands::Insert(args) => insert_command(args).await,
        Commands::Drop(args) => drop_command(args).await,
        Commands::Cast(args) => cast_command(args),
        Commands::Merge(args) => merge_command(args).await,
        Commands::Partition(args) => partition_command(args),
        Commands::Pl(args) => pl_command(args),
        // interactive
        Commands::Df(args) => df_command(args),
        Commands::Lf(args) => lf_command(args),
    }
}

/// Utility for creating and managing MESC RPC configurations
#[derive(Parser)]
#[clap(author, version, about, long_about = None, disable_help_subcommand = true, styles=get_styles())]
pub(crate) struct Cli {
    #[clap(subcommand)]
    pub(crate) command: Commands,
}

pub(crate) fn get_styles() -> clap::builder::Styles {
    let white = anstyle::Color::Rgb(anstyle::RgbColor(255, 255, 255));
    let green = anstyle::Color::Rgb(anstyle::RgbColor(0, 225, 0));
    let grey = anstyle::Color::Rgb(anstyle::RgbColor(170, 170, 170));
    let title = anstyle::Style::new().bold().fg_color(Some(green));
    let arg = anstyle::Style::new().bold().fg_color(Some(white));
    let comment = anstyle::Style::new().fg_color(Some(grey));
    clap::builder::Styles::styled()
        .header(title)
        .error(comment)
        .usage(title)
        .literal(arg)
        .placeholder(comment)
        .valid(title)
        .invalid(comment)
}

/// Define your subcommands as an enum
#[derive(Subcommand)]
#[command()]
pub(crate) enum Commands {
    //
    // // read commands
    //
    /// Show list of tabular files
    Ls(LsArgs),
    /// Show schema of tabular files
    Schema(SchemaArgs),
    /// Show first N rows of a dataset
    Cat(CatArgs),
    /// Show first N rows of a dataset (alias for `cat`)
    Head(HeadArgs),
    /// Show last N rows of a dataset (alias for `cat --tail`)
    Tail(TailArgs),
    /// Count value occurences within column(s) of data
    Count(CountArgs),
    //
    // // edit commands
    //
    /// Insert columns into tabular files
    Insert(InsertArgs),
    /// Drop columns from tabular files
    Drop(DropArgs),
    /// Cast columns of tabular files
    Cast(CastArgs),
    /// Merge tabular files
    Merge(MergeArgs),
    /// Partition tabular files
    Partition(PartitionArgs),
    /// Edit files using polars python syntax
    Pl(PlArgs),
    //
    // // interactive commands
    //
    /// Load inputs as a dataframe in an interactive python session
    Df(DfArgs),
    /// Load inputs as a lazyframe in an interactive python session
    Lf(LfArgs),
}

//
// // read commands
//

#[derive(Parser)]
pub(crate) struct InputArgs {
    /// input path(s) to use
    #[clap(short, long)]
    pub(crate) inputs: Option<Vec<PathBuf>>,

    /// recursively list all files in tree
    #[clap(long)]
    pub(crate) tree: bool,
}

#[derive(Parser)]
pub(crate) struct CatArgs {
    #[clap(flatten)]
    pub(crate) head_args: HeadArgs,

    #[clap(long)]
    pub(crate) tail: bool,
}

#[derive(Parser)]
pub(crate) struct HeadArgs {
    #[clap(flatten)]
    pub(crate) input_args: InputArgs,

    /// columns to print
    #[clap(long)]
    pub(crate) columns: Option<Vec<String>>,

    /// number of file names to print
    #[clap(long)]
    pub(crate) n: Option<usize>,

    /// sort before showing preview
    #[clap(short, long)]
    pub(crate) sort: Option<Vec<String>>,

    /// offset before printing head
    #[clap(short, long)]
    pub(crate) offset: Option<usize>,
}

#[derive(Parser)]
pub(crate) struct TailArgs {
    #[clap(flatten)]
    pub(crate) input_args: InputArgs,

    /// columns to print
    #[clap(long)]
    pub(crate) columns: Option<Vec<String>>,

    /// number of file names to print
    #[clap(long)]
    pub(crate) n: Option<usize>,

    /// sort before showing preview
    #[clap(short, long)]
    pub(crate) sort: Option<Vec<String>>,

    /// limit to this number of rows
    #[clap(short, long)]
    pub(crate) limit: Option<usize>,
}

#[derive(Parser)]
pub(crate) struct CountArgs {
    /// columns to print
    #[clap()]
    pub(crate) columns: Vec<String>,

    #[clap(flatten)]
    pub(crate) input_args: InputArgs,

    /// number of file names to print
    #[clap(long)]
    pub(crate) n: Option<usize>,
}

/// Arguments for the `ls` subcommand
#[derive(Parser)]
pub(crate) struct LsArgs {
    /// input path(s) to use
    #[clap(short, long)]
    pub(crate) inputs: Option<Vec<PathBuf>>,

    /// recursively list all files in tree
    #[clap(long)]
    pub(crate) tree: bool,

    /// number of file names to print
    #[clap(long)]
    pub(crate) n: Option<usize>,

    /// show absolute paths instead of relative
    #[clap(long)]
    pub(crate) absolute: bool,

    /// show long version with extra metadata
    #[clap(long)]
    pub(crate) long: bool,

    /// show files only, no totals
    #[clap(long)]
    pub(crate) files_only: bool,
}

/// Arguments for the `schema` subcommand
#[derive(Parser)]
pub(crate) struct SchemaArgs {
    /// input path(s) to use
    #[clap()]
    pub(crate) inputs: Option<Vec<PathBuf>>,

    /// recursively list all files in tree
    #[clap(long)]
    pub(crate) tree: bool,

    /// show absolute paths instead of relative
    #[clap(long)]
    pub(crate) absolute: bool,

    /// print top n schemas
    #[clap(long)]
    pub(crate) n_schemas: Option<usize>,

    /// print example paths of each schema
    #[clap(long)]
    pub(crate) include_example_paths: bool,

    /// sort schemas by row count, file count, or byte count
    #[clap(long, default_value = "bytes")]
    pub(crate) sort: String,
}

#[derive(Parser)]
pub(crate) struct InsertArgs {
    /// column specifications, in pairs of COLUMN_NAME DTYPE
    pub(crate) new_columns: Vec<String>,

    /// input path(s) to use
    #[clap(short, long, value_delimiter = ' ', num_args = 1..)]
    pub(crate) inputs: Option<Vec<String>>,

    /// recursively list all files in tree
    #[clap(long)]
    pub(crate) tree: bool,

    /// output directory to write modified files
    #[clap(long)]
    pub(crate) output_dir: Option<PathBuf>,

    /// index of inserted column(s)
    #[clap(long)]
    pub(crate) index: Option<Vec<usize>>,

    /// default value(s) of inserted column(s)
    #[clap(long)]
    pub(crate) default: Option<Vec<String>>,

    /// confirm that files should be edited
    #[clap(long)]
    pub(crate) confirm: bool,

    /// prefix to add to output filenames
    #[clap(long)]
    pub(crate) output_prefix: Option<String>,

    /// postfix to add to output filenames
    #[clap(long)]
    pub(crate) output_postfix: Option<String>,
}

//
// // edit commands
//

/// Arguments for the `drop` subcommand
#[derive(Parser)]
pub(crate) struct DropArgs {
    /// columns to drop
    #[clap()]
    pub(crate) columns: Vec<String>,

    /// input path(s) to use
    #[clap(short, long)]
    pub(crate) inputs: Option<Vec<PathBuf>>,

    /// recursively list all files in tree
    #[clap(long)]
    pub(crate) tree: bool,

    /// confirm that files should be edited
    #[clap(long)]
    pub(crate) confirm: bool,

    /// prefix to add to output filenames
    #[clap(long)]
    pub(crate) output_prefix: Option<String>,

    /// postfix to add to output filenames
    #[clap(long)]
    pub(crate) output_postfix: Option<String>,

    /// output directory to write modified files
    #[clap(long)]
    pub(crate) output_dir: Option<PathBuf>,

    /// show output paths
    #[clap(long)]
    pub(crate) show_output_paths: bool,
}

/// Arguments for the `cast` subcommand
#[derive(Parser)]
pub(crate) struct CastArgs {
    /// input path(s) to use
    #[clap(short, long)]
    pub(crate) inputs: Option<Vec<PathBuf>>,
}

/// Arguments for the `merge` subcommand
#[derive(Parser)]
pub(crate) struct MergeArgs {
    /// output path to use
    #[clap()]
    pub(crate) output_path: PathBuf,

    /// input path(s) to use
    #[clap()]
    pub(crate) inputs: Vec<PathBuf>,

    /// keep original files after merging
    #[clap(long)]
    pub(crate) keep: bool,

    /// confirm merge
    #[clap(long)]
    pub(crate) confirm: bool,
}

/// Arguments for the `partition` subcommand
#[derive(Parser)]
pub(crate) struct PartitionArgs {
    /// input path(s) to use
    #[clap(short, long)]
    pub(crate) inputs: Option<Vec<PathBuf>>,
}

/// Arguments for the `pl` subcommand
#[derive(Parser)]
pub(crate) struct PlArgs {
    /// input path(s) to use
    #[clap(short, long)]
    pub(crate) inputs: Option<Vec<PathBuf>>,
}

//
// // interactive commands
//

/// Arguments for the `df` subcommand
#[derive(Parser)]
pub(crate) struct DfArgs {
    /// input path(s) to use
    #[clap()]
    pub(crate) inputs: Option<Vec<PathBuf>>,

    /// use tree of inputs
    #[clap(long)]
    pub(crate) tree: bool,

    /// python executable to use
    #[clap(short, long)]
    pub(crate) executable: Option<String>,

    /// load lazily
    #[clap(short, long)]
    pub(crate) lazy: bool,
}

/// Arguments for the `lf` subcommand
#[derive(Parser)]
pub(crate) struct LfArgs {
    /// input path(s) to use
    #[clap()]
    pub(crate) inputs: Option<Vec<PathBuf>>,

    /// use tree of inputs
    #[clap(long)]
    pub(crate) tree: bool,

    /// python executable to use
    #[clap(short, long)]
    pub(crate) executable: Option<String>,
}