extern crate youtube_dl;

use youtube_dl::downloader;
use std::io::Write;

fn main() {
  let args: Vec<String> = env::args().collect();
  let url = args.get(1).unwrap_or(&String::new());
  let filename = args.get(2).unwrap_or(&String::new());


  let result = downloader::download(url);

  match result {
    Ok(file) => {
      // proceseaza fisierul descarcat
      let mut local_file = std::fs::file::create(filename).unwrap();
      local_file.write_all(&file).unwrap();
    };

    Err(err) => {
      // afiseaza eroarea
      println!("{}", err);
    };
  };
};