struct Rect{
    width:u32,
    height:u32
}

impl Rect {
    fn area(&self) -> u32{
        return self.width * self.height; 
    }
}
pub fn  main() {
    let rect = Rect{
        width:20,
        height:20
    };
    print!("rect area 为 {:?}", rect.area());
}