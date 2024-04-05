#[no_mangle]
pub extern "C" fn blue_stuff() {
    let mut s = alloc_stuff();
    s.push_str(" Hello!");
    s.push_str(" World!");
    println!("blue! s = {s}");

    use_regex();
}

#[inline(never)]
fn alloc_stuff() -> String {
    "blah blah".to_string()
}

// force instantiation of external crate
#[inline(never)]
fn use_regex() {
    let rx = regex::Regex::new("\\w");
    println!("blue: {}", rx.is_ok());
}
