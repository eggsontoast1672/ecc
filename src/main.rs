fn main() {
    let mut args = std::env::args();
    let program = args.nth(1).unwrap();
    ecc::compile_and_link(program);
}
