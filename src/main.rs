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
    let mut points: Vec<[isize; 2]> = vec![
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
    let mut border_color = Color::new(255, 255, 255); 
    let mut fill_color = Color::new(255, 255, 0);   

    // Dibuja el polígono usando el framebuffer
    framebuffer.polygon(&points, border_color, fill_color);

    points = vec![
        [321, 335], [288, 286], [339, 251], [374, 302]
    ];

    border_color = Color::new(255, 255, 255); 
    fill_color = Color::new(0, 0, 255); 

    framebuffer.polygon(&points, border_color, fill_color);

    points = vec![
        [377, 249], [411, 197], [436, 249]
    ];

    border_color = Color::new(255, 255, 255); 
    fill_color = Color::new(255, 0, 0); 

    framebuffer.polygon(&points, border_color, fill_color);

    points = vec![
        [413, 177], [448, 159], [502, 88], [553, 53], [535, 36], [676, 37], [660, 52],
        [750, 145], [761, 179], [672, 192], [659, 214], [615, 214], [632, 230], [580, 230],
        [597, 215], [552, 214], [517, 144], [466, 180]
    ];

    border_color = Color::new(255, 255, 255); 
    fill_color = Color::new(0, 255, 0); 

    framebuffer.polygon(&points, border_color, fill_color);

    points = vec![
        [682, 175], [708, 120], [735, 148], [739, 170]
    ];

    border_color = Color::new(255, 255, 255);
    fill_color = Color::new(0, 0, 0); // Puedes cambiar el color de relleno si lo deseas

    framebuffer.polygon(&points, border_color, fill_color);

    // Guarda el framebuffer como un archivo BMP
    if let Err(e) = framebuffer.save_as_bmp("out.bmp") {
        eprintln!("Error al guardar el archivo BMP: {}", e);
    }
}

