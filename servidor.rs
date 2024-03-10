// um programa que escuta solicitações http

use std::io::{BufRead, Write};

fn main(){
	let ouvinte = std::net::TcpListener::bind("127.0.0.1:9999").unwrap();

	// dentro do loop alguem ira se conectar
	for mut stream in ouvinte.incoming().flatten(){
		let mut rdr = std::io::BufReader::new(&mut stream);

		loop {
			let mut l = String::new();
			rdr.read_line(&mut l).unwrap();
			if l.trim().is_empty(){
				break;
			}
			print!("{l}");
		}
		// resposta de sucesso
		stream.write_all(b"HTTP/1.1 200 OK\r\n\r\nEntendido!").unwrap();
	}
}