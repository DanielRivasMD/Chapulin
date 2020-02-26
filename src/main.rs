#![allow(dead_code)]

mod file_reader;
mod mobile_elements;

fn main() -> std::io::Result<()> {
  mobile_elements::me_controller()?;
  Ok(())
}
