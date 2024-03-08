mod image_process;
mod output_format;
mod args;

use image_process::ImageProcess;
use args::IconImageArgs;
use clap::Parser;

fn main() {
    let args: IconImageArgs = IconImageArgs::parse();

    let mut img_p = ImageProcess::new(args.input_path, args.output_path);
    img_p.run().unwrap();
}

