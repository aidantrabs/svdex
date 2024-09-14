use image::{ GenericImageView };

fn decomposition(img: &str) {
    let A = image::open(format!("{img}")).unwrap();
    println!("{:?}", A.dimensions());
}

fn main() {
    println!("Hello, world!");
    let img_name = "examples/picture.jpg";
    decomposition(img_name);
}
