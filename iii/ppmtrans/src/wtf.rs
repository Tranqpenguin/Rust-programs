//use std::iter::Iterator;
use csc411_image::{Read,Write,RgbImage,Rgb};
//use std::env;
use std::time::Instant;
//storage model for crate is always in row major order
pub use array2::Array2;
Expand
only_squares_work_main.txt
19 KB
//these functions simply make a new transformed vector
use crate::Array2;

//have to do a quick and dirty fix for my iterators getting into unknown indexes
pub fn make_transform_vector<T:std::clone::Clone>(original_load: & Array2<T>, col_major_arg: bool, transform_flag:  &str)->Vec<T>{ //probably should make this generic for whatever the is Vec<T::values> okay?
    //some type of closure to include algo here
Expand
only_squares_work_transformations.txt
3 KB
﻿
//use std::iter::Iterator;
use csc411_image::{Read,Write,RgbImage,Rgb};
//use std::env;
use std::time::Instant;
//storage model for crate is always in row major order
pub use array2::Array2;
//use std::io::{self, Write};
//use std::path::PathBuf;
use clap::{Parser};//, arg, command, value_parser, ArgAction, Command};
//use std::fs;
//use std::io::BufReader;
//use structopt::StructOpt;
//use std::rc::Rc;
mod transformations;
/* 
#[derive(StructOpt)
#[structopt(rename_all = "kebab-case")]
struct Opt
*/

//take a single operation
//error checking for input 411 image read function is the only checking needed


#[derive(Parser, Debug)]
#[clap(author ="Justin W. <justin_watkins@uri.edu>", version ="1.0", about ="just a guy struggling with Rust", long_about = None)]
struct Args {
    //file pathway
    //#[clap(required = false)]
    //not too sure if I want to have PathBuf as an option but honestly if none exists
    //the read from the csc411 image will take care of it?

    
    path : Option<String>,
    //path : Option<PathBuf>, //should be taking the first non flagged argument?//////////////////////////////////////////BIG CONCERN IS WHETHER THIS WILL TAKE IN THE FILENAME OF THE PROGRAM AS THE PATHNAME ARGUMENT INSTEAD OF THE PPM FILENAME WE WANT TO USE


    /*
    Quick breakdown of how CLAP works
    Each explicit declarative statement (the thing with the #) will specify a flag
    is need for every argument it's declaring under it
    when we specify long = "row-major" what we're doing is expecting a double dash
    flag with the statement "row-major" following it, then after that statement
    we're taking the next value (String, num, whatever) to be the value we're specifying

    had to use an older version of CLAP for the takes_value to work
    
    if no value is follows the specified flag, then we use takes_value = false
        then we set whatever is being specified as a bool to ensure processing of the command
    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////UNSURE IF I SHOULD HAVE A REQUIRED FOR ANYTHING
    ///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////pretty sure if no arguments are entered, then the default of CLAP will pop up saying an error

    */
        //multiple = true will allow multiple arguments after it?
    //row major or column major argument have to be required?
    #[clap(long = "row-major", takes_value = false, required = false)]
    row_major_arg : bool, //--row-major
    #[clap(long = "col-major",takes_value = false, required = false)]
    col_major_arg : bool, //--col-major
    // Flip
    #[clap(long = "flip", required = false)]
    flip: Option<String>, //--flip horizontal OR --flip vertical
    // Rotation
    #[clap(short = 'r', long = "rotate")]
    rotate: Option<u32>, //--rotate 90 OR 180,270,0
    // Transposition
    #[clap(long = "transpose", takes_value = false, required = false)]
    transpose: bool, //--transpose

    //think this is sufficient enough, by making the path option the only thing with
    //no flag, the first thing with no flag within the specific argument being called
    //from the program's first running will be taken in as the pathway for the regular
    //run command and if file is given through stdin then read for csc411 will
    //take a None argument from the path and look for stdin input

    //only one command should be able to be done at a time?

    //maybe add documentation saying which order they'll be performed? idk

}



 

fn main() {

                                    ///////////Argument intake from CLAP structure\\\\\\\\\\\\\\\
    let args = Args::parse();
    let rotate = args.rotate;
    let flip = args.flip;
    let transpose = args.transpose;
    let _row_major_arg = args.row_major_arg;
    let col_major_arg = args.col_major_arg;
    
                                            ////other initial code to read in source image and create array\\\\\

    //assuming we have to use Rgb, unsure if needed for both
    //let result_img = csc411_image::read(filename)

    //takes in an option string
    //dont think dereference is needed with CLAP, buf read stuff is handled in RgbImage csc411 lib
    let image = RgbImage::read((args.path).as_deref()).unwrap();//UNWRAP WAS FINE HE SAID, JUST NEED SOME SORT OF STDERR OR PANIC
    let width = image.width as usize;
    let height = image.height as usize;
    //
    let mut new_height = height; //need these as the transformation needs the old height and width but the new height and width have to be switched sometimes based off the transformation
    let mut new_width = width;  //these allow me to do the switch right after the initial match statements
    let original_load = Array2::from_row_major(width, height, image.pixels); 
        //vector of pixels to be modified
    //let modified_img = Vec<csc411_image::imgtype::Rgb> = Vec::new();
    //or let modified_img = Vec<_> = Vec::new();
    let mut transform_flag = String::new();
    //let mut modified_coordinates : Vec<(usize,usize)> = Vec::new(); //allowed as long as type is defined within this program's lifetime
    //Vec::new();
    //based on the argument given in the command will iterate row or column major to operate on everything
    /* 
    if row_major_arg {
        image_as_is = row_major(original_load);
    } else {
        assert!(col_major_arg); //should catch if no iterator is specified
        image_as_is = col_major(original_load);
    }
    */

    


    //so only handle one case at a time BUT need specification on what type of iterator to use
    //so basically,
    
    //trying to deal with if more than one rotate,flip, or transpose occurs in command line 

    //DeMorgan's theorem for XOR to filter out multiple arguments 
    //will not filter out multiple flip or rotates but will probably do only the last one specified as the argument will get overwritten throughout args parsing????
    //  NEED WARNING IN HELP 
    //(a!=0)^(b!=0)
    //if (!transpose) && (!rotate.is_none()) && (!flip.is_none()){ //ensures that no two are written in teh argument at a time or 0 are written into the argument (0 part is redundant)
        //match case depending on the arg inputs
        //will go through possible states of each argument and perform specified transformation
        
        //SWITCH WIDTH AND HEIGHT DEPENDING ON TRANSFORMATION RIGHT WITHIN 
        //switch width and height parameters depending on rotation, transpose, etc

        //these matches will simply set the status flag and switch height and width depending on the transformation
    match rotate {
        Some(90) => {let temp = new_width; new_width = new_height; new_height = temp; transform_flag = ("rot90").to_string() },
        Some(180) => transform_flag = ("rot180").to_string(),
        Some(270) => {let temp = new_width; new_width = new_height; new_height = temp; transform_flag = ("rot270").to_string()},
        Some(0) => transform_flag = ("rot0").to_string(),
        None => (),
        _ =>(),
    }

    match flip.as_deref() {
        Some("horizontal") => transform_flag = ("hf").to_string(),
        Some("vertical") => transform_flag = ("vf").to_string(),
        None => (),
        Some(&_) => (),
    }

    if transpose {
        let temp = new_width;
        new_width = new_height;
        new_height = temp;
        transform_flag = ("trans").to_string()

    }
    //} else {
    //    eprintln!("You got an error here, try this again with just one transformation flag at a time");
    //}
    
    //benchmarking rotation
    let now = Instant::now();
    //function here makes new vector transform
    let modified_img : Vec<Rgb> = transformations::make_transform_vector(&original_load,col_major_arg, transform_flag.as_ref());
    //values = modified_img store the modified image values here
    eprintln!("{:.2?}", now.elapsed());
    eprintln!("{:?},new_width",new_width);
    eprintln!("{:?}",modified_img);
    //taking the modified values image and converting it into the proper RgbImage format for stdout display
    let dest = RgbImage{
        width: new_width as u32,
        height: new_height as u32,
        denominator: image.denominator,
        pixels: modified_img,
    };

    //println!("{:?},new_width{:?}old_width",new_width,width);
    
    //write should write to stdout as it handles the None case in this way
    dest.write(None).unwrap();
    //from the command line redirect to a file and then display that file if needed | or  > (2> for stderr is pretty cool)
    
    //unsure if need for wrapping in if statement to decide if I want to benchmark it or not
    //let elapsed = now.elapsed();
}




//basically each function has to be able to do row_major or col_major args

//--row-major and col-major catches to compute whatever image traversal occurs from just their input
/* 
pub fn row_major(original_load: & mut Array2, modified_img: &mut Vec<T>){
    for (row, column, pixel) in original_load.iter_row_major(){
        //this will store the file now in row_major form
        iterated_pixels.push(original_load.get_mut_element(row,col));////////////////////////////////////////////////major fix if end up using professor's array2
    }
    iterated_pixels
}



//--col major to compute whatever image traversal occurs
pub fn col_major(original_load: & mut Array2, modified_img: &mut Vec<T>){
    for (row,column,pixel) in original_load.iter_col_major(){
        modified_img.push(original_load.get_mut_element(row,col))
    }
    modified_img
}

*/

//some map_row_major or map_col_major code to do the rotation
//can't come up right away with anything clever to modularize the iterators]
//Rotate image 90 degrees clockwise.

//IF I HAVE MORE THAN ONE REFERENCE I HAVE TO INCLDUE LIFETIMES

//what if i just returned an array of coordinates back?
//then the 
/* 
pub fn rotate_90<T:std::clone::Clone>(original_load: & mut Array2<T>, col_major_arg: bool)->Vec<&T>{
    if col_major_arg {
        let new_coords = original_load.iter_col_major().map(move |(row,column,pixel)| (column,original_load.height()-row-1)).collect()
        //flat_map(move |(row,column,_pixel)|{modified_img.push(original_load.get_element(column,original_load.height()-row-1).unwrap())} )
        //for (row,column) in new_coords{
        //  modified_img.push(*original_load.get_element(row,column).unwrap());
        //}
    } else { //default value is col-major is not specified
        //assert!(row_major_arg); //should catch if no iterator is specified
        //for (row,column,_pixel) in original_load.iter_row_major(){
            //are these going to be the same? probably
        //    modified_img.push(*original_load.get_mut_element(column,original_load.height()-row-1).unwrap());
        //}
        let new_coords : Vec<(usize,usize)> = original_load.iter_col_major().map(move |(row,column,pixel)| (column,original_load.height()-row-1)).collect()
    }
}

//Rotate image 180 degrees.
pub fn rotate_180<'a,T:std::clone::Clone>( original_load: &'a mut Array2<T>, row_major_arg: bool, col_major_arg: bool, modified_img: &'a mut Vec<T>){
    if col_major_arg {
        for (row,column,_pixel) in original_load.iter_col_major(){
            modified_img.push(*original_load.get_mut_element(original_load.height() - row - 1 ,original_load.width() - column -1).unwrap());
        }
    } else { //default value is col-major is not specified
        //assert!(row_major_arg); //should catch if no iterator is specified
        for (row,column,_pixel) in original_load.iter_row_major(){
            modified_img.push(*original_load.get_mut_element(original_load.width() - row -1,original_load.height() - column - 1 ).unwrap());
        }        
    }
}

//Rotate image 270 degrees clockwise (or 90 ccw).
pub fn rotate_270<'a,T:std::clone::Clone>( original_load: &'a mut Array2<T>, row_major_arg: bool, col_major_arg: bool, modified_img: &'a mut Vec<T>){
    if col_major_arg {
        for (row,column,_pixel) in original_load.iter_col_major(){
            modified_img.push(*original_load.get_mut_element(original_load.width()-1-column,row).unwrap());
        }
    } else { //default value is col-major is not specified
        //assert!(row_major_arg); //should catch if no iterator is specified
        for (row,column,_pixel) in original_load.iter_row_major(){
            modified_img.push(*original_load.get_mut_element(original_load.width()-1-column,row).unwrap());
        }
    }
}

//change nothing
pub fn rotate_0<'a,T:std::clone::Clone>( original_load: &'a mut Array2<T>, row_major_arg: bool, col_major_arg: bool, modified_img: &'a mut Vec<T>){
    //literally do nothing but copy the pixels so you can get them to stdout
    if col_major_arg {
        for (row,column,_pixel) in original_load.iter_col_major(){
            modified_img.push(*original_load.get_mut_element(row,column).unwrap());
        }
    } else {
        //assert!(row_major_arg); //should catch if no iterator is specified
        for (row,column,_pixel) in original_load.iter_row_major(){
            modified_img.push(*original_load.get_mut_element(row,column).unwrap());
        }
    }  
}
//Mirror image horizontally (left-right).
pub fn horizontal_flip<'a,T:std::clone::Clone>( original_load: &'a mut Array2<T>, row_major_arg: bool, col_major_arg: bool, modified_img: &'a mut Vec<T>){
    if col_major_arg {
        for (row,column,_pixel) in original_load.iter_col_major(){
            modified_img.push(*original_load.get_mut_element(row,original_load.width()-column-1).unwrap());
        }
    } else {
        //assert!(row_major_arg); //should catch if no iterator is specified
        for (row,column,_pixel) in original_load.iter_row_major(){
            modified_img.push(*original_load.get_mut_element(row,original_load.width()-column-1).unwrap());
        }
    }
//algo is (row,width-col-1)
}

//Mirror image vertically (top-bottom).
pub fn vertical_flip<'a,T:std::clone::Clone>( original_load: &'a mut Array2<T>, row_major_arg: bool, col_major_arg: bool, modified_img: &'a mut Vec<T>){
    //top to bottom algo is (height-row-1,col)
    if col_major_arg {
        for (row,column,_pixel) in original_load.iter_col_major(){
            modified_img.push(*original_load.get_mut_element(original_load.height()-row-1, column).unwrap());
        }
    } else {
        //assert!(row_major_arg); //should catch if no iterator is specified
        for (row,column,_pixel) in original_load.iter_row_major(){
            modified_img.push(*original_load.get_mut_element(original_load.height()-row-1, column).unwrap());
        }
    }
}

//Transpose image (across UL-to-LR axis).
pub fn transpose_transformation<'a,T:std::clone::Clone>( original_load: &'a mut Array2<T>, row_major_arg: bool, col_major_arg: bool, modified_img: &'a mut Vec<T>){
//just switch row and col indicies 
    if col_major_arg {
        for (row,column,_pixel) in original_load.iter_col_major(){
            modified_img.push(*original_load.get_mut_element(column,row).unwrap());
        }
    } else {
        //assert!(row_major_arg); //should catch if no iterator is specified
        for (row,column,_pixel) in original_load.iter_row_major(){
            modified_img.push(*original_load.get_mut_element(column,row).unwrap());
        }
    }
}
*/

/*
properly being able to writeout
let stdout = io::stdout();
let mut handle = stdout.lock();
let mut final_handle = io::BufWriter::new(handle);

loop doing writing

writeln!(final_handle, "")
 */

/* 
fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) {
    for line in content.lines() {
        if line.contains(pattern) {
            writeln!(writer, "{}", line);
        }
    }
}

he new parameter is mut writer, i.e., a mutable thing we call “writer”. 
Its type is impl std::io::Write, which you can read as “a placeholder 
for any type that implements the Write trait”. Also note how we replaced 
the println!(…) we used earlier with writeln!(writer, …). println! works 
the same as writeln! but always uses standard output.

#[test]
fn find_a_match() {
    let mut result = Vec::new();
    find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result);
    assert_eq!(result, b"lorem ipsum\n");
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;

    find_matches(&content, &args.pattern, &mut std::io::stdout());

    Ok(())
}

Note: Since stdout expects bytes (not strings), we use std::io::Write instead of std::fmt::Write. As a result, we give an empty vector as “writer” in our tests (its type will be inferred to Vec<u8>), in the assert_eq! we use a b"foo". (The b prefix makes this a byte string literal so its type is going to be &[u8] instead of &str).

Note: We could also make this function return a String, but that would change its behavior. Instead of writing to the terminal directly, it would then collect everything into a string, and dump all the results in one go at the end.


*/

    //do we need invalid filename handling??
    /* This is not the best implementation: It will read the whole file 
    into memory – however large the file may be. Find a way to optimize it! 
    (One idea might be to use a BufReader instead of read_to_string().) */

    /*
    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
     */
    //let filename = std::fs::File::open(&args.path);//.expect("could not read file");
    /*
    let content = match filename {
        Ok(content) => { content }
        Err(error) => { println!("Oh noes: {}", error); }
    }

    */
    //let mut reader = BufReader::new(filename);
    //assert!(env::args().len() /*unsure of arguments */);
    //let filename = env::args().nth(/*unsure of index */);

    /* 
    use std::io::{self, Write};

    fn main() -> io::Result<()> {
        let stdout = io::stdout();
        let mut handle = stdout.lock();

        handle.write_all(b"hello world")?;

        Ok(())
    }

    use std::io::{self, Write};

    fn main() -> io::Result<()> {
        io::stdout().write_all(b"hello world")?;

        Ok(())
    }
    */
