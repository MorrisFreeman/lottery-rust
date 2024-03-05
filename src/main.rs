

fn main() {
    if let Err(e) = lottery::get_args().and_then(lottery::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
