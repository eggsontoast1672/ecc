use colored::Colorize;

fn main() {
    let mut args = std::env::args();
    let program_name = args.next().unwrap(); // This should never panic
    let Some(file_name) = args.next() else {
        let program_name = format!("{}:", program_name).bold().white();
        let fatal_error = "fatal error:".bold().red();

        eprintln!("{} {} no input files", program_name, fatal_error);
        eprintln!("compilation terminated.");

        std::process::exit(1);
    };

    ecc::compile_and_link(file_name);
}
