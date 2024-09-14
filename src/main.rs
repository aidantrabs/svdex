use image::{ GenericImageView };

fn decomposition(img: &str) {
    let A = image::open(format!("{img}")).unwrap();
    let rgb = A.to_rgb8();
    let chunks: Vec<&[u8]> = rgb.chunks(3).collect();

    for chunk in &chunks {
        println!("{:?}", chunk);
    }
}

fn main() {
    let img_name = "examples/picture.jpg";
    decomposition(img_name);
}
