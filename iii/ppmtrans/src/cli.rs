use clap::Parser;
/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
pub struct Cli {

    /// The path to the file to read
    pub path : Option<String>,
    #[clap(long = "row-major", takes_value = false, required = false)]
    pub row_major_arg : bool, //--row-major
    #[clap(long = "col-major",takes_value = false, required = false)]
    pub col_major_arg : bool, //--col-major
    // Flip
    #[clap(long = "flip", required = false)]
    pub flip: Option<String>, //--flip horizontal OR --flip vertical
    // Rotation
    #[clap(short = 'r', long = "rotate")]
    pub rotate: Option<u32>, //--rotate 90 OR 180,270,0
    // Transposition
    #[clap(long = "transpose", takes_value = false, required = false)]
    pub transpose: bool, //--transpose
}