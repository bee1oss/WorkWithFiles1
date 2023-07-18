use std::fs::File;
use std::io::{Read};

fn main() {
    //File::create("text.txt").expect("Error creating file!!");create sadece file olusturma degil only-write yani sadece icine bir seyler yazabiliyoruz
    //File::open("text.txt").expect("Error creating file!!");open ise bir dosyayi acmak ve sadece okumak icin

    let path = "text.txt";

    let mut f = File::open(path).expect("Error opening file!");

    //f.write_all("Hello from bega".as_bytes()).expect("Error writing to file!");

    let mut file_text = String::new();

    f.read_to_string(&mut file_text).expect("Error reading file!");

    println!("{}",file_text);

}
