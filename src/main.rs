mod kmeans;
use kmeans::*;
use image::{GenericImageView};
use hex::encode;

fn main() {
    let img = image::open("/Users/yoavnir/Documents/vs code/rust/work in progress/data_analysis/extract_colors/annegret-kammer-sGptUDSrMVU-unsplash.jpg").unwrap();
    let pixels = img.pixels();

    let mut data1 : Vec<Point> = Vec::new();
    for pixel in pixels.map(|i| (i.2)) {
        data1.push(Point {
            r : pixel[0] as f64,
            g : pixel[1] as f64,
            b : pixel[2] as f64
        })
    }


    let mut kmeans = Kmeans::new(data1,6);

    for _i in 0..30 {
        let diff = kmeans.new_means();
        println!("{}",diff);
        if diff < 20.0 {
            break
        }
    }

    for mean in kmeans.means {
        println!("{}",rgb_to_hex(&mean.point_to_pixel()));
    };
}

fn rgb_to_hex(point:&[u8]) -> String {
    return format!("#{}",encode(point))
}