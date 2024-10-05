pub mod db;
mod server;
mod gui;

use std::error::Error;


use crate::db::get_clipboard_content;
use crate::server::server;

fn main() -> Result<(), Box<dyn Error>> {
    // Levantar el servidor solo si se pasa el argumento "server"
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 && args[1] == "server" {
        return server();
    }

    // Buscar en el historial de portapapeles si se pasa el argumento "search"
    if args.len() > 1 && args[1] == "search" {
        return search();
    }

    // Iniciar la interfaz gráfica
    gui::start();
    Ok(())
}


fn search() -> Result<(), Box<dyn Error>> {
    get_clipboard_content()?.iter().for_each(|content| {
        println!("{}", content);
    });
    Ok(())
}


// Copiar imagen
// let mut ctx = Clipboard::new().unwrap();
// let img = ctx.get_image().unwrap();
// println!("Image data is:\n{:?}", img.bytes);

// use std::error::Error;
// use arboard::{Clipboard, SetExtLinux, LinuxClipboardKind};
//
// fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
//     Clipboard::new()?.set().wait().text("Hello, world!")?;
//     println!("Clipboard set to 'Hello, world!'");
//      Ok(())
// }