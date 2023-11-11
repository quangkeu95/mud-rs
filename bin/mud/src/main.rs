fn main() {
    if let Err(err) = mud::cli::run() {
        eprintln!("Error: {err:?}");
        std::process::exit(1);
    }
}
