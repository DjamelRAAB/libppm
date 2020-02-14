use std::path::Path;
use ppm::*;


fn main() {
    let a : Pixel = Pixel::new(120, 30, 180);
    let b : Pixel = Pixel::new(120, 30, 180);
    //  let b : Pixel = Pixel::new(10, 130, 80);
    let mut v: Vec<Pixel> = Vec::new();
    v.push(a);
    v.push(b);
    // Test PartialEq
    if a == b {
        println!("{}",a);
    }
    else {
        println!("a : {}, b : {}",a,b);
    }
    // Test inv 
    let im : Image= Image::new(String::from("P3") ,2,1,255, v) ;
    let inv_im = im.invert();

    //Test image save 
    // définir une image dans test_save et elle sera répliquer dans test_save2
    let path = Path::new("test_save.ppm");
    let _ = inv_im.save(&path);

    let path = Path::new("./big_image/mandelbrot.ppm");
    let image_loaded = new_with_file(&path);
    let inv_path = Path::new("./big_image/inv_mandelbrot.ppm");
    let gray_path = Path::new("./big_image/gray_mandelbrot.ppm");
    let _ = image_loaded.invert().save(&inv_path);
    let _ = image_loaded.gray_scale().save(&gray_path);

    // new_with_binary_file
    let image_binary_loaded  = new_with_binary_file("binary_image/Lena.512.ppm");
    let bin_path = Path::new("binary_image/dix_Lena.512.ppm");
    let _res = image_binary_loaded.save(&bin_path);
}
