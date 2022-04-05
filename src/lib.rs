use std::error::Error;
use std::fs;

pub struct Config {
  pub query: String,
  pub filename: String
}

impl Config {
  pub fn new(env_args: &[String]) -> Result<Config, &'static str> {
      if env_args.len() < 3 {
        return Err("Not enough argument");
      }

      let query = env_args[1].clone();
      let filename = env_args[2].clone();

      Ok(Config {query, filename})
  }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let contents = fs::read_to_string(config.filename)?;
  println!("With text:\n{:?}", contents);
  Ok(())
}