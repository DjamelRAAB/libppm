use ppm::*;


#[test]
fn test_new_pixel() {
    let pixel = Pixel::new(120,150,200);
    assert_eq!(120, pixel.red());
    assert_eq!(150, pixel.green());
    assert_eq!(200, pixel.blue());
}

#[test]
fn test_red() {
    let pixel = Pixel::new(120,150,200);
    assert_eq!(120, pixel.red());
}

#[test]
fn test_green() {
    let pixel = Pixel::new(120,150,200);
    assert_eq!(150, pixel.green());
}

#[test]
fn test_blue() {
    let pixel = Pixel::new(120,150,200);
    assert_eq!(200, pixel.blue());
}

#[test]
fn test_invert_pixel() {
    let pixel = Pixel::new(120,150,200);
    let inv_pixel = pixel.invert();
    assert_eq!(135, inv_pixel.red());
    assert_eq!(105, inv_pixel.green());
    assert_eq!(55, inv_pixel.blue());}

#[test]
fn test_grayscale_pixel() {
    let pixel = Pixel::new(120,150,200);
    let gray_pixel = pixel.gray_scale();
    assert_eq!(156, gray_pixel.red());
    assert_eq!(156, gray_pixel.green());
    assert_eq!(156, gray_pixel.blue());
}

#[test]
fn test_partial_eq_() {
    let pixel1 = Pixel::new(120,150,200);
    let pixel2 = Pixel::new(120,150,200);
    assert!(pixel1==pixel2);
}

