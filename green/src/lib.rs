#[no_mangle]
pub extern "C" fn green_stuff() {
    let mut s = String::new();
    s.push_str("Hello!");
    s.push_str("World!");
    println!("green! s = {s}");

    use_regex();
}

// force instantiation of external crate
#[inline(never)]
fn use_regex() {
    let rx = regex::Regex::new("\\w");
    println!("green: {}", rx.is_ok());
}
