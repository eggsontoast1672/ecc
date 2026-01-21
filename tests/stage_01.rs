#[test]
fn valid_multi_digit() {
    let source = r"int main(void) {
        return 100;
    }
    ";

    let assembly = ecc::compile_source(source);
}
