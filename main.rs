extern crate youtube_dl;

use youtube_dl::downloader;
use std::io::Write;

fn main() {
  let args: Vec<String> = env::args().collect();
  let mut url = String::new();
 
  println!("Enter URL:");
  std::io::stdin().read_line(&mut url).unwrap();
  url = url.trim().to_string();
 
  let mut filename = String::new();
  println!("Enter Filename:");
  std::io::stdin().read_line(&mut filename).unwrap();

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
