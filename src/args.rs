use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about)]
pub struct IconImageArgs {
    /// Image path.
    /// Example: yourIconFolder/yourImage.png
    pub input_path: String,
    /// Export images to output path.
    /// It will create a folder 'output' under your output path.
    /// Example: yourProject/Icon
    pub output_path: String,
}
