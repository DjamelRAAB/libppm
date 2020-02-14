use ppm::*;
use std::path::Path;


#[test]
fn test_invert_image() {
    let p1 : Pixel = Pixel::new (120, 30, 180);
    let p2 : Pixel = Pixel::new (120, 3, 180);
    let mut pixels: Vec<Pixel> = Vec::new();
    pixels.push(p1);
    pixels.push(p2);
    let im : Image= Image::new ( String::from("P3") ,2,1,255, pixels) ;
    let inv_im = im.invert();
    assert!(im != inv_im);
}


#[test]
fn test_grayscale_image() {
    let p1 : Pixel = Pixel::new (120, 30, 180);
    let p2 : Pixel = Pixel::new (120, 3, 180);
    let mut pixels: Vec<Pixel> = Vec::new();
    pixels.push(p1);
    pixels.push(p2);
    let im : Image= Image::new ( String::from("P3") ,2,1,255, pixels) ;
    let gray_im = im.gray_scale();
    assert!(im != gray_im);
}


#[test]
fn test_save_and_load() {
    let p1 : Pixel = Pixel::new (120, 30, 180);
    let p2 : Pixel = Pixel::new (120, 3, 180);
    let mut pixels: Vec<Pixel> = Vec::new();
    pixels.push(p1);
    pixels.push(p2);
    let im : Image= Image::new ( String::from("P3") ,2,1,255, pixels) ;
    let path = Path::new("test_save_load.ppm");
    let _ = im.save(&path);
    let im_loaded = new_with_file(&path);
    assert!(im == im_loaded);

}