use std::error::Error;
use clap::{Arg, Command};
use rand::seq::SliceRandom;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
  pub names: Vec<String>,
}

pub fn run (config: Config) -> MyResult<()> {
  println!("{:?}", config);

  let mut names = config.names;
  let mut rng = rand::thread_rng();
  names.shuffle(&mut rng);

  let name = match names.pop() {
    Some(name) => name,
    None => {
      eprintln!("No name(s) found.");
      std::process::exit(1);
    }
  };
  println!("The winner is {}", name);

  Ok(())
}

pub fn get_args() -> MyResult<Config> {
  let m = Command::new("lottery")
    .version("0.1.0")
    .author("Yuta Hirata <morrisfreeman0718@gmail.com")
    .about("Lottery")
    .arg(
      Arg::new("names")
        .value_name("NAMES")
        .help("input name(s)")
        .num_args(1..)
    )
    .get_matches();

  let names_ref = match m.get_many::<String>("names") {
    Some(names) => names,
    None => {
      eprintln!("Please input name(s)");
      std::process::exit(1)
    }
  };
  let names = names_ref
    .map(|s| s.to_string())
    .collect();

  Ok(
    Config {
      names,
    }
  )
}
