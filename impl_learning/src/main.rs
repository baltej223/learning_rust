struct Complex {
    real: i32,
    img: i32,
}

impl Complex {
    fn distance(self) {
        let distance = ((self.real.pow(2) + (self.img).pow(2)) as f64).sqrt();
        println!("{}", distance);
    }
}

fn main() {
    let c = Complex { real: 21, img: 32 };
    c.distance();
}
