fn main() {
    let a: i64 = 1;
    let b: i32 = a as i32;
    let c: i16 = b as i16;
    let d: i8  = c as i8;
    let e: u8  = d as u8;
    let f: u16 = e as u16;
    let g: u32 = f as u32;
    let h: u64 = g as u64;
    let i: f64 = h as f64;
    let j: f32 = i as f32;
    println!("j = {}", j);
}
