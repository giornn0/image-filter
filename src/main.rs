// use std::io::{Read, Result, Write};
// use std::fs::File;
// use std::path::Path;
// use std::env::current_dir;

// fn main()->Result<()> {
//     let curr_dir = current_dir().expect("Error while getting current dir");
//     let test = File::open(Path::new(&curr_dir).join("src/test/test2.jpeg"))?;
//     let mut file = File::create(Path::new(&curr_dir).join("src/test/testing.jpeg"))?;
//     let mut bytes : Vec<u8>= Vec::new();

//     // print!("{:?}",(&test).clone().);

    
//     for (i,byte) in test.bytes().enumerate().into_iter(){
//       if i>200000{
//         bytes.push(0);
//       }else{
//         bytes.push(byte.unwrap());
//       }
//     }
    
//     // let half = &bytes.len() /2 ;

//     // bytes.reverse();
//     file.write_all(&bytes.by_ref()).expect("Error while writing");

//     Ok(())
// }

// use std::env::current_dir;
// use std::path::Path;

// use num_complex::Complex;
// use image::{GenericImage, GenericImageView, ImageBuffer, RgbImage};

// fn main(){
//   let imgx = 800;
//   let imgy = 800;

//   let scalex = 3.0 / imgx as f32;
//   let scaley = 3.0 / imgy as f32;

//   // Create a new ImgBuf with width: imgx and height: imgy
//   let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

//   // Iterate over the coordinates and pixels of the image
//   for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
//       let r = (0.3 * x as f32) as u8;
//       let b = (0.3 * y as f32) as u8;
//       *pixel = image::Rgb([r, 0, b]);
//   }

//   // A redundant loop to demonstrate reading image data
//   for x in 0..imgx {
//       for y in 0..imgy {
//           let cx = y as f32 * scalex - 1.5;
//           let cy = x as f32 * scaley - 1.5;

//           let c = Complex::new(-0.4, 0.6);
//           let mut z = Complex::new(cx, cy);

//           let mut i = 0;
//           while i < 255 && z.norm() <= 2.0 {
//               z = z * z + c;
//               i += 1;
//           }

//           let pixel = imgbuf.get_pixel_mut(x, y);
//           let image::Rgb(data) = *pixel;
//           *pixel = image::Rgb([data[0], i as u8, data[2]]);
//       }
//   }

//   let curr_dir = current_dir().unwrap();

//   // Save the image as “fractal.png”, the format is deduced from the path
//   imgbuf.save(Path::new(&curr_dir).join("src/fractal.png")).unwrap();
// }
use std::env::current_dir;
use std::path::Path;
use image::{GenericImage, GenericImageView, ImageBuffer, RgbImage, open, Pixel};

fn main (){
    let curr_dir = current_dir().expect("Error reading current dir");
    let img = open(Path::new(&curr_dir).join("src/test/test5.jpeg")).unwrap();

    
    let (width, height) = img.dimensions();

    let mut test = ImageBuffer::new(width, height);
    for (x, y, pixel) in test.enumerate_pixels_mut() {
            let r = (0.3 * x as f32) as u8;
            let b = (0.3 * y as f32) as u8;
            *pixel = image::Rgb([255, 255, 255]);
    }
    
    println!("{}, {}", width, height);

    for x in 0..width{
      for y in 0..height{
        let pixel = img.get_pixel(x, y);
        let sum: f32 = f32::from(pixel.0[1]) + f32::from(pixel.0[0]);
        if f32::from(pixel.0[2]) * 1.4 > sum && pixel.0[2]>80{
          println!("{:?}",pixel);
          test.put_pixel(x,y,pixel.to_rgb());
        }
      }
    }

    test.save(Path::new(&curr_dir).join("src/test/testing5.png"));
} 