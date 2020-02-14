/*!
 * # PPM image manipulation library
 * Each PPM image consists of the following:
 * * A "magic number" for identifying the file type. A ppm image's magic number is the two characters "P6".
 * * Whitespace (blanks, TABs, CRs, LFs).
 * * A width, formatted as ASCII characters in decimal.
 * * Whitespace.
 * * A height, again in ASCII decimal.
 * * Whitespace.
 * * The maximum color value (Maxval), again in ASCII decimal. Must be less than 65536 and more than zero.
 * * A single whitespace character (usually a newline).
 * * A raster of Height rows, in order from top to bottom. Each row consists of Width pixels, in order from left to right. Each pixel is a triplet of red, green, and blue samples, in that order. Each sample is represented in pure binary by either 1 or 2 bytes. If the Maxval is less than 256, it is 1 byte. Otherwise, it is 2 bytes. The most significant byte is first.
!*/
extern crate libc;
use std::fmt;
use std::path::Path;
use std::io::Write;
use std::fs::File;
use std::fs;
use std::io::Read;
use std::io::BufReader;
use std::io::BufRead;

// Level 1
#[derive(Debug,Clone, Copy)]
/// Pixel structure to encode an RGB color.
/// ```ignore
/// pub struct Pixel {
/// r : u8,
/// g : u8,
/// b : u8,
/// }
/// ```
pub struct Pixel {
    r : u8,
    g : u8,
    b : u8,
}

impl Pixel {
/// Function make build pixel.
    pub fn new(red: u8, green: u8, blue: u8 ) -> Pixel {
        Pixel{ r : red, g : green, b : blue}
    }
/// Function make access to red color of pixel.
    pub fn red(self) -> u8{
        self.r
    } 
/// Function make access to green color of pixel.
    pub fn green(self) -> u8{
        self.g
    }
/// Function make access to blue color of pixel.
    pub fn blue(self) -> u8{
        self.b
    }   
/// Function inverts pixel : 
/// each color is trasformed by follow formul :
/// `color = 255 - color` 
    pub fn invert (&self) -> Pixel{
        Pixel::new(255-&self.r, 255-&self.g ,255-&self.b)
    }
/// Function to compare two pixel by checking RGB compose 
    pub fn eq(&self, other: &Self) -> bool {
        if self.r==other.r && self.b==other.b && self.g==other.g {
            true
        }else {
            false
        }
    }
/// Function who convert an RGB pixel to a grayscale pixel
    pub fn gray_scale (&self) -> Pixel{
        let gray = self.r/3 + self.g/3 + self.b/3;
        Pixel::new(gray, gray, gray)
    }

} 

/// Function display pixel in terminal in hexadecimal form.
impl fmt::Display for Pixel {
    fn fmt ( &self ,f : &mut fmt::Formatter ) -> fmt::Result {
        write!( f, "#{:X}{:X}{:X}" , self.r , self.g, self.b)
    }
}
/// Function allow compare two pixel by `==` operator
/// ```ignore
/// pixel1 == pixel2
/// ```  
impl PartialEq for Pixel {
    fn eq(&self, other: &Self) -> bool {
        if self.r==other.r && self.b==other.b && self.g==other.g{
            true
        }else {
            false
        }
    }
}


// Level 2
#[derive(Debug,Clone)]
/// Image structure to encode image with RGB color in PPM format.
/// ```ignore
/// pub struct Pixel {
/// magic : String ,
/// height : usize,
/// width : usize,
/// max_color: usize,
/// pixels: Vec<Pixel>
/// }
/// ```
pub struct Image {
    magic : String ,
    height : usize,
    width : usize,
    max_color: usize, 
    pixels: Vec<Pixel>
}

impl Image {
/// Function make build image.
    pub fn new(magic : String ,height : usize,width : usize,max_color: usize, pixels: Vec<Pixel>) -> Image {
        Image{
            magic : magic,
            height : height,
            width : width,
            max_color : max_color,
            pixels : pixels
        }
    }

/// Function who saves struct into a file.
/// 
/// An example file in text mode looks like:
/// ```ignore
/// P3
/// 3 2
/// 255
/// # The part above is the header
/// # "P3" means this is an RGB color image in ASCII
/// # "3 2" is the width and height of the image in pixels
/// # "255" is the maximum value for each color
/// # The part below is image data: RGB triplets
/// 255 0   0   # red
/// 0   255 0   # green
/// 0   0   255 # blue
/// 255 255 0   # yellow
/// 255 255 255 # white
/// 0   0   0   # black
/// ```
    pub fn save(&self ,filename: &Path) -> std::io::Result<()> {
        let mut file = File::create(filename).expect("Create file failed");
        let header = format!("{}\n{} {}\n{}\n", self.magic, self.height, self.width, self.max_color );
        file.write(header.as_bytes()).expect("write failed");
        for h in 0..self.height {
            for w in 0..self.width {
                let pixel = self.pixels[h*self.width+w];
                if h*w==self.height*self.width-1 {
                    let row = format!("{} {} {}", pixel.red(), pixel.green(), pixel.blue() );
                    file.write(row.as_bytes()).expect("write failed");
                }
                else {
                    let row = format!("{} {} {}\n", pixel.red(), pixel.green(), pixel.blue() );
                    file.write(row.as_bytes()).expect("write failed");
                }
            }
        }
        Ok(())
    }

/// Function that inverts image colors.
pub fn invert(&self)->Image{
    let mut inv_pixels : Vec<Pixel> = Vec::new();
    for i in 0 ..(self.height  * self.width ){
        let inv_pixel : Pixel = self.pixels[i].invert();
        inv_pixels.push(inv_pixel);
    }
    
    Image::new(
                self.magic.clone(),
                self.height,
                self.width,
                self.max_color,
                inv_pixels
    )
}

/// Function that makes image B&W based on a filter color.
    pub fn gray_scale(&self)->Image{
        let mut gray_pixels : Vec<Pixel> = Vec::new();
        for i in 0 ..(self.height  * self.width ){
            let gray_pixel : Pixel = self.pixels[i].gray_scale();
            gray_pixels.push(gray_pixel);
        }
        
        Image::new(
                    self.magic.clone(),
                    self.height,
                    self.width,
                    self.max_color,
                    gray_pixels
        )
    }

/// Function make access to pixels of some image.
    pub fn pixels(self) -> Vec<Pixel> {
        self.pixels
    }
}

/// Function allow compare two image by `==` operator
/// ```ignore
/// image1 == image2
/// ```  
impl PartialEq for Image {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..(self.height*self.width){
            if !(self.pixels[i]==other.pixels[i]) {
                return false;
            }
        }
        true
    }
}


/// Function read in text mode a ppm image, they need path access to file
/// ```ignore
/// // To use it : 
/// let path : Path = Path::new("file.ppm");
/// let image_loaded_with_file : Image = new_with_file(&path);
/// ```
pub fn new_with_file(filename: &Path) -> Image {

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let mut data:Vec<&str> = Vec::new();
    for line in contents.split("\n"){
        data.push(line);
    }

    let mut pixels:Vec<Pixel> = Vec::new();
    for line in &data[3..] {
        let colors:Vec<&str>= line.split_whitespace().collect();
        for color in 0..(colors.len()/3){
            let pixel:Pixel = Pixel {r:colors[0+color*3].to_string().parse::<u8>().unwrap(),
                                    g:colors[1+color*3].to_string().parse::<u8>().unwrap(),
                                    b:colors[2+color*3].to_string().parse::<u8>().unwrap(),
                                }; 
            pixels.push(pixel)
        }
    }

    let height_width:Vec<&str>= data[1].split_whitespace().collect();
    
    Image::new(
        data[0].to_string(),
        height_width[0].to_string().parse::<usize>().unwrap(),
        height_width[1].to_string().parse::<usize>().unwrap(),
        data[2].to_string().parse::<usize>().unwrap(),
        pixels
    )
}

/// Function to read a header
pub fn read_header(filename: &str) -> Vec<String>{
    let file = File::open(filename)
        .expect(filename);
    let reader = BufReader::new(file);
    let mut index = 0;
    let mut head:Vec<String> = Vec::new();
    for line in reader.lines() {
        if let Ok(ip) = line {
            head.push(ip);
        };
        index = index+ 1;
        if index >2 {
            break;
        }
    }
    head
}

/// Function read in binary mode a ppm image, they need path in str form for access to file
/// ```ignore
/// // To use it : 
/// let image_loaded_with_binary_file : Image = new_with_binary_file("image_binary.ppm");
/// ```
pub fn new_with_binary_file(filename: &str)-> Image{
    let head = read_header(filename);
    let height_width:Vec<&str>= head[1].split_whitespace().collect();
    let h =height_width[0].to_string().parse::<usize>().unwrap();
    let w = height_width[1].to_string().parse::<usize>().unwrap();
    
    let mut pixels:Vec<Pixel> = Vec::new();
    let mut file=File::open(filename).unwrap();
    let mut data=[0x8;3];
    for _ in 0..(h*w){
        file.read(&mut data).unwrap();
        let pixel:Pixel = Pixel::new(data[0].to_string().parse::<u8>().unwrap(),
                        data[1].to_string().parse::<u8>().unwrap(),
                        data[2].to_string().parse::<u8>().unwrap());
        println!("{}",pixel);
        pixels.push(pixel)
    }

    Image::new(head[0].to_string(),h, w,head[2].parse::<usize>().unwrap(),pixels)
}


// Level 3 is in test directory

// Level 4 
mod ffi;
pub fn read_ppm(file:&str){
    unsafe{
        ffi::ppma_read_test(file.as_ptr());
    }
}
pub fn write_ppm (file:&str){
    unsafe{
        ffi::ppma_write_test(file.as_ptr());
    }
}
