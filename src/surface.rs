//!
//! #Surface
//!
//! This module is intended to abstract how the output color of the raytracer
//! is saved into a file.
//! 
//! For now it's only purpose is to write the image into a PPM format.
//!
//! *Workflow*
//!
//! |      Step 1      |      Step 2      |      Step 3      |      Step 4      |
//! |------------------|------------------|------------------|------------------|
//! | enable(filepath) |  Set up world    |      Render      |   write surface  |
//! |                  |  Set up camera   |                  |                  |
//! 
//! Might need some refactorization in order to render the scene into a buffer
//! This could be useful in the case of future works around concurrency
//! 

use std::io::Write;

///
/// The surface struct stores data relative to the render image
/// Two coordinate systems are defined for the data it represents.
/// A tuple of int is the position of a pixel data in the buffer array 
/// the axis y is downward.
/// For a tuple of floats this origin is located at the bottom left corner.
/// 
pub struct Surface {
    pub width: u32,
    pub height: u32,
    buffer: Vec<(u8, u8, u8)>, // Could use array width const width and fov
    output_file: String,
}

impl Surface {

    pub fn new(width: u32, height: u32, output_file: &String) -> Surface 
    {
        Surface { 
            width, 
            height, 
            //buffer: Vec::<(u8, u8, u8)>::with_capacity((width*height) as usize),
            buffer: vec![(0, 0, 0); (width*height) as usize],
            output_file: output_file.clone(), 
        }
    }
    
    pub fn default() -> Surface {
        let default_size = (1200, 800);
        Surface { 
            width: default_size.0, 
            height: default_size.1, 
            buffer: Vec::<(u8, u8, u8)>::with_capacity((default_size.0*default_size.1) as usize),
            output_file: String::from("render"), 
        }
    }
    
    pub fn to_float(&self, i: &i32, j: &i32) -> (f64, f64) {
        (
            ((i-(self.width/2) as i32) as f64 /self.height as f64), 
            ((j-(self.height/2) as i32) as f64 /self.height as f64),
        )
    } 

    pub fn to_int(&self, x: &f64, y: &f64) -> (u32, u32) {
        (
            (x*(self.height as f64)) as u32 + self.width/2,
            (y*(self.height as f64)) as u32 + self.height/2,
        )
    }

    ///
    /// This function is intended to be called 'before' rendering since this step can be
    /// quite long to avoid suprises afterward.
    /// It register the filename where the surface will be written to.
    /// A check is performed a the same time to verify that the file is accessible.
    ///
    pub fn save_to(&mut self, output_file: &String) {
        self.output_file = match is_accessible(output_file) {
            true => output_file.clone(),
            false => panic!("Error file {} cannot be accessed", output_file),
        };
    }

    ///
    /// This function write the buffer array to the file set by the [save_to] function's call
    /// 
    pub fn export(&self) {
        let mut file = std::fs::File::options()
            .read(true)
            .write(true)
            .open(self.output_file.clone())
            .expect("Error: could not open file to save surface");
        
        // Writing PPM header
        let header = format!("P3\n{} {}\n256\n\n", self.width, self.height);
        file.write(header.as_bytes())
            .expect("Error: could not write to file");

        // Writing surface
        for j in 0..self.height {
            for i in 0..self.width {
                let val = self.get_val(i, j);
                let val =  format!("{} {} {}\n", val.0, val.1, val.2);
                file.write(val.as_bytes())
                    .expect("Error: could not write to file");
            }
        }
    }

    pub fn get_val(&self, i: u32, j: u32) -> (u8, u8, u8) {
        self.buffer[(i + j*self.width) as usize]
    }

    pub fn set_val(&mut self, i: u32, j: u32, v: (u8, u8, u8)) {
        self.buffer[(i + j*self.width) as usize] = v;
    }
    
}


/// Utiliy function to ensure it is possible to open a file 
fn is_accessible(filepath: &String) -> bool {
    match std::fs::File::options()
        .read(true)
        .write(true)
        .open(filepath) 
    {
        Result::Ok(_) => true,
        Result::Err(_) => return false,
    }
} 




