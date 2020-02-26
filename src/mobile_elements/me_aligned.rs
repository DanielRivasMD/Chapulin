
use crate::file_reader;

pub fn me_identificator(
  me_bam_file: &String
) -> std::io::Result<()> {

  let (mut reader, mut buffer) = file_reader::file_reader(&me_bam_file);

  while let Some(line) = reader.read_line(&mut buffer) {

    println!("{}", line?.trim());

  }

  Ok(println!("{} {}", "File read: ", &me_bam_file))
}
