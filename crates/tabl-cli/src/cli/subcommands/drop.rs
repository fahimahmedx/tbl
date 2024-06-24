use crate::styles::FontStyle;
use crate::{DropArgs, TablCliError};
use std::path::PathBuf;
use toolstr::Colorize;

pub(crate) async fn drop_command(args: DropArgs) -> Result<(), TablCliError> {
    inquire::set_global_render_config(crate::styles::get_render_config());

    // determine which paths to use
    let output_spec = tabl::filesystem::OutputPathSpec::new()
        .inputs(args.inputs.clone())
        .output_dir(args.output_dir.clone())
        .tree(args.tree)
        .file_prefix(args.output_prefix.clone())
        .file_postfix(args.output_postfix.clone());
    let (inputs, outputs) = tabl::filesystem::get_output_paths(output_spec)?;

    // get schemas of input paths
    let schemas = tabl::parquet::get_parquet_schemas(&inputs).await?;

    // check that all files have the relevant columns
    for (input, schema) in inputs.iter().zip(schemas) {
        for column in args.columns.iter() {
            if !schema.contains(column) {
                let msg = format!(
                    "File does not contain column {}: {}",
                    column,
                    input.to_string_lossy()
                );
                return Err(TablCliError::Error(msg));
            }
        }
    }

    // print summary
    print_drop_summary(&args, &inputs, &outputs).await?;

    // get confirmation to edit files
    if !args.confirm {
        let prompt = "continue? ";
        if let Ok(true) = inquire::Confirm::new(prompt).with_default(false).prompt() {
        } else {
            return Ok(());
        }
    }

    // edit files
    println!("not implemented yet");

    Ok(())
}

async fn print_drop_summary(
    args: &DropArgs,
    inputs: &[PathBuf],
    outputs: &[PathBuf],
) -> Result<(), TablCliError> {
    // print files
    let n_show_files = 10;
    println!("files:");
    if args.inputs.is_none() {
        let cwd = std::env::current_dir()?;
        for path in inputs.iter().take(n_show_files) {
            println!(
                "- {}",
                path.strip_prefix(cwd.clone())?
                    .to_string_lossy()
                    .colorize_string()
            );
        }
    } else {
        for path in inputs.iter().take(n_show_files) {
            println!("- {}", path.to_string_lossy().colorize_string());
        }
    }
    if inputs.len() > n_show_files {
        println!("...");
    }

    // print summary
    let first_column = if let Some(first_column) = args.columns.first() {
        first_column.clone()
    } else {
        return Err(TablCliError::Arg(
            "must specify column(s) to drop".to_string(),
        ));
    };
    let mut columns_str = first_column.colorize_variable().bold();
    for column in args.columns.iter().skip(1) {
        columns_str = format!("{}, {}", columns_str, column.colorize_variable().bold()).into()
    }
    let column_str = if args.columns.len() == 1 {
        "column"
    } else {
        "columns"
    };
    let file_str = if inputs.len() == 1 { "file" } else { "files" };

    let output_location = if let Some(output_dir) = args.output_dir.as_ref() {
        format!(
            "\nwriting outputs to directory {}",
            output_dir.to_string_lossy().colorize_string()
        )
    } else {
        ", editing files inplace".to_string()
    };

    println!();
    println!(
        "dropping {} {} from {} {} {}",
        column_str,
        columns_str,
        tabl::formats::format_with_commas(inputs.len() as u64)
            .colorize_constant()
            .bold(),
        file_str,
        output_location
    );

    if args.output_dir.is_some() {
        let n_existing = tabl::filesystem::count_existing_files(outputs).await;
        if n_existing > 0 {
            println!(
                "{} of the output files already exist and will be overwritten",
                tabl::formats::format_with_commas(n_existing as u64).colorize_constant(),
            );
        }
    }

    Ok(())
}
