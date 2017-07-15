extern crate qrcode;

use std::env;
use std::process::exit;
use std::io::{self, Read};
use std::iter;
use qrcode::QrCode;

fn main() {

    let code = match env::args().skip(1).next() {
        Some(arg) => arg.into_bytes(),
        None => read_stdin(),
    };

    let code = match QrCode::new(&code[..]) {
        Ok(code) => code,
        Err(e) => {
            println!("Can't generate QR code: {:?}", e);
            exit(1)
        }
    };

    let string = code.render()
        .light_color(' ')
        .dark_color('#')
        .build();

    let mut empty_str: String;
    let mut line_buffer = String::new();
    let mut lines = string.lines().into_iter();

    while let Some(line_top) = lines.next() {
        let line_bottom = match lines.next() {
            Some(l) => l,
            None => {
                empty_str = iter::repeat(' ').take(line_top.len()).collect();
                empty_str.as_str()
            }
        };

        for (top, bottom) in line_top.chars().zip(line_bottom.chars()) {
            let block = match (top, bottom) {
                ('#', '#') => '█',
                (' ', '#') => '▄',
                ('#', ' ') => '▀',
                _ => ' ',
            };
            line_buffer.push(block);
        }

        println!("{}", line_buffer);
        line_buffer.clear();
    }
}

fn read_stdin() -> Vec<u8> {
    let mut data = Vec::new();
    io::stdin().read_to_end(&mut data).expect("Read from STDIN");
    data
}
