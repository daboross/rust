fn main() {
    let val = 42;
    let x = match val {
        (0 if true) => {
        //~^ ERROR unexpected parentheses surrounding `match` arm pattern
            42u8
        }
        _ => 0u8,
    };
    let _y: u32 = x; //~ ERROR mismatched types
}
