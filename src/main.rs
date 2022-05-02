mod kmeans;
use kmeans::*;
use image::{GenericImageView};
use hex::encode;

fn main() {
    let img = image::open("/Users/yoavnir/Documents/vs code/rust/work in progress/data_analysis/extract_colors/test1.jpg").unwrap();
    let pixels = img.pixels();

    let mut data1 : Vec<Point> = Vec::new();
    for pixel in pixels.map(|i| (i.2)) {
        data1.push(Point {
            r : pixel[0] as f64,
            g : pixel[1] as f64,
            b : pixel[2] as f64
        })
    }

    let mut kmeans = Kmeans::new(data1,8);
    //println!("{:?}",kmeans.means);

    for _i in 0..20 {
        let diff = kmeans.new_means();
        println!("{}",diff);
        if diff < 20.0 {
            break
        }
    }

    let mut colors = Vec::new();

    for mean in kmeans.means {
        colors.push(mean.point_to_pixel());
        println!("{}",rgb_to_hex(&mean.point_to_pixel()));
    };

    rbg_to_image(colors);
}

fn rgb_to_hex(point:&[u8]) -> String {
    return format!("#{}",encode(point))
}

fn rbg_to_image(colors:Vec<[u8;3]>) {
    let mut buffer : image::RgbImage = image::ImageBuffer::new(colors.len() as u32 * 100,500);
    let mut inc = 0;
    for idx in 0..colors.len() {
        for y in 0..500 {
            for x in 0..100 {
                buffer.put_pixel(x+inc, y, image::Rgb(colors[idx]))
            }
        }
        inc+=100;
    }
    buffer.save("colors.jpg").unwrap()
}