mod framebuffer;
mod line;
mod polygon;
mod color;
mod bmp;

use framebuffer::Framebuffer;
use line::Line;
use polygon::Polygon;
use color::Color;
use nalgebra_glm as glm;

fn main() {
    let mut framebuffer = Framebuffer::new(800, 600); // Crea un Framebuffer de 800x600

    // Define un array de puntos para un polígono
    let points: Vec<[isize; 2]> = vec![
        [165, 380],
        [185, 360],
        [180, 330],
        [207, 345],
        [233, 330],
        [230, 360],
        [250, 380],
        [220, 385],
        [205, 410],
        [193, 383],
    ];

    // Define los colores para el borde y el relleno
    let border_color = Color::new(255, 255, 255); // Rojo
    let fill_color = Color::new(255, 255, 0);   // Verde

    // Dibuja el polígono usando el framebuffer
    framebuffer.polygon(&points, border_color, fill_color);

    // Guarda el framebuffer como un archivo BMP
    if let Err(e) = framebuffer.save_as_bmp("out.bmp") {
        eprintln!("Error al guardar el archivo BMP: {}", e);
    }
}
