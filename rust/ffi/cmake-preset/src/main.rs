unsafe extern "C" {
    #[link_name = "Add"]
    unsafe fn add(a: i32, b: i32) -> i32;
    #[link_name = "Hello"]
    unsafe fn hello();
}

fn main() {
    println!("{}", unsafe { add(1, 2) });
    unsafe { hello() };
}
