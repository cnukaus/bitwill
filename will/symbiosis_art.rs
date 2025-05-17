use image::{ImageBuffer, Rgb};
use std::f64::consts::PI;

fn draw_circle(img: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, x: i32, y: i32, radius: i32, color: Rgb<u8>) {
    for i in -radius..=radius {
        for j in -radius..=radius {
            if i*i + j*j <= radius*radius {
                let px = (x + i) as u32;
                let py = (y + j) as u32;
                if px < img.width() && py < img.height() {
                    img.put_pixel(px, py, color);
                }
            }
        }
    }
}

fn draw_line(img: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, x1: i32, y1: i32, x2: i32, y2: i32, color: Rgb<u8>) {
    let dx = (x2 - x1).abs();
    let dy = (y2 - y1).abs();
    let sx = if x1 < x2 { 1 } else { -1 };
    let sy = if y1 < y2 { 1 } else { -1 };
    let mut err = dx - dy;

    let mut x = x1;
    let mut y = y1;

    loop {
        if x >= 0 && x < img.width() as i32 && y >= 0 && y < img.height() as i32 {
            img.put_pixel(x as u32, y as u32, color);
        }

        if x == x2 && y == y2 {
            break;
        }

        let e2 = 2 * err;
        if e2 > -dy {
            err -= dy;
            x += sx;
        }
        if e2 < dx {
            err += dx;
            y += sy;
        }
    }
}

fn main() {
    // Create a new image with a white background
    let width = 800;
    let height = 600;
    let mut img = ImageBuffer::new(width, height);
    
    // Fill with white
    for pixel in img.pixels_mut() {
        *pixel = Rgb([255, 255, 255]);
    }

    // Colors
    let black = Rgb([0, 0, 0]);
    let blue = Rgb([0, 0, 255]);
    let red = Rgb([255, 0, 0]);
    let green = Rgb([0, 255, 0]);

    // Draw human circle
    let human_x = 200;
    let human_y = 300;
    draw_circle(&mut img, human_x, human_y, 50, blue);
    
    // Draw AI circle
    let ai_x = 600;
    let ai_y = 300;
    draw_circle(&mut img, ai_x, ai_y, 50, red);

    // Draw connecting lines
    draw_line(&mut img, human_x + 50, human_y, ai_x - 50, ai_y, green);
    draw_line(&mut img, human_x, human_y + 50, ai_x, ai_y + 50, green);
    draw_line(&mut img, human_x, human_y - 50, ai_x, ai_y - 50, green);

    // Draw evolution spiral
    let center_x = width as i32 / 2;
    let center_y = height as i32 / 2;
    let mut prev_x = center_x;
    let mut prev_y = center_y;
    
    for t in 0..360 {
        let angle = t as f64 * PI / 180.0;
        let radius = t as f64 * 0.5;
        let x = center_x + (radius * angle.cos()) as i32;
        let y = center_y + (radius * angle.sin()) as i32;
        
        if t > 0 {
            draw_line(&mut img, prev_x, prev_y, x, y, black);
        }
        
        prev_x = x;
        prev_y = y;
    }

    // Save the image
    img.save("symbiosis.png").unwrap();
    println!("Image saved as symbiosis.png");
} 