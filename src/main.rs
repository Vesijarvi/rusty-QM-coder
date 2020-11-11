pub mod coder;
use coder::qmcoder;
extern crate clap;
use clap::{App, Arg , ArgGroup};
use std::fs;
use std::io::Read;
use crate::fs::File;

fn get_file_as_byte_vec(filename: &String)->Vec<u8>{
    let error_msg = "Error reading file: "
        .to_string() + filename;
    let mut f = File::open(&filename).expect(&error_msg);
    let metadata = fs::metadata(&filename)
        .expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");
    buffer
}

/* fn is_binary_image(byte_stream: &Vec<u8>)->bool {
    let mut is_binary = true;
    for byte in byte_stream {
        if byte > &1 {
            is_binary = false;
            break;
        }
    }
    is_binary
} */

fn to_bit_plane(byte_stream: &Vec<u8>)->Vec<bool>{
    let mut bit_planes: Vec<bool> = Vec::new();
    
    for c in 0..8 {
        for byte in byte_stream {
            let bit = byte & 2_u8.pow(c);
            match bit {
                0 => {bit_planes.push(false);}
                _ => {bit_planes.push(true);}
            };
        }
    }    
    bit_planes
}

fn main() {
    let matches = App::new("QM coder")
        .arg(
            Arg::with_name("compress")
                .short("c")
                .long("compress")
                .value_name("FILE")
                .takes_value(true),
        )
        .get_matches();

    if let Some(file) = matches.value_of("compress"){
        let byte_stream: Vec<u8> = get_file_as_byte_vec(&file.to_string());

        // we do not separate gray/binary image
        let bit_plane: Vec<bool> = to_bit_plane(&byte_stream);
        

    }
}
