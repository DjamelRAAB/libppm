# Big Project

Members:
  ```
  Arnauld BIAM
  Amine FAIZ
  Zohir MALTI
  Djamel RAAB
  ```


A rust library for `.ppm` image manipulation and processing.

The aim of this project is to learn how to set up a library 
which can be useful for other libraries.

With this library, one is able to resize an image and play with pixels.

## Get Started with the project


- Fork the project on your personal repository
- Clone it on your local machine or download it
- Build the project through the following commands
  ` cargo build `
- You can have access to the specific documentation we have created to help you. 
Just run `cargo open --doc`
- launch Test with follow command line 
  ` cargo test `




# ppm 
* level 1 => Ok
* level 2 => Ok 
* level 3 => Ok
* level 4 => Ok
To use external libraries, you have to change the path in `build.rs` file like : 
```
    println!("other_lib_other_language/ppm/src/");
```


* level 5 => !
<br/>
![programation](./ob_637c51_57483454.jpg)

## Extras
We added a feature called `use_library_ppm` which uses the library(as its name says) for tests and appreciations
you have to change the path in `cargo.toml` file  like : 
```
[dependencies.ppm]
path = "my_path/ppm"
```