use colored::Colorize;

fn main() {
    let mut args = std::env::args();
    let program_name = args.next().unwrap(); // This should never panic
    let Some(file_name) = args.next() else {
        eprintln!(
            "{program_name}: {} {}",
            "error:".bold().red(),
            "no input files".bold().white()
        );

        std::process::exit(1);
    };

    ecc::compile_and_link(file_name);
}
