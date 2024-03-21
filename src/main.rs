use num_complex::Complex;
use sfml::graphics::*;
use sfml::system::*;
use sfml::window::*;
use std::cmp;

fn calculate(c: Complex<f64>, max_iterations: i32) -> i32 {
    let mut iterations = 0;
    let mut z = Complex::new(0.0, 0.0);
    while z.norm() <= 4.0 && iterations < max_iterations {
        iterations += 1;
        z = z * z + c;
    }
    iterations
}

fn color(iterations: i32, max_iterations: i32) -> Color {
    let hue = iterations % 360;
    let sat = 0.8;
    let val = if max_iterations == iterations {
        0.0
    } else {
        1.0
    };

    let h = hue / 60;
    let f = hue as f64 / 60.0 - h as f64;
    let p = val * (1.0 - sat);
    let q = val * (1.0 - sat * f);
    let t = val * (1.0 - sat * (1.0 - f));

    match h {
        1 => Color::rgb((q * 255.0) as u8, (val * 255.0) as u8, (p * 255.0) as u8),
        2 => Color::rgb((p * 255.0) as u8, (val * 255.0) as u8, (t * 255.0) as u8),
        3 => Color::rgb((p * 255.0) as u8, (q * 255.0) as u8, (val * 255.0) as u8),
        4 => Color::rgb((t * 255.0) as u8, (p * 255.0) as u8, (val * 255.0) as u8),
        5 => Color::rgb((val * 255.0) as u8, (p * 255.0) as u8, (0.0 * 255.0) as u8),
        _ => Color::rgb((val * 255.0) as u8, (t * 255.0) as u8, (p * 255.0) as u8),
    }
}

fn render_rows(
    image: &mut Image,
    extent: &f64,
    origin: &Complex<f64>,
    max_iterations: &i32,
    start: u32,
    end: u32,
) {
    assert!(end > start);
    let length = image.size().x;
    for i in start..end {
        for j in 0..image.size().x {
            unsafe {
                image.set_pixel(
                    j,
                    i,
                    color(
                        calculate(
                            extent
                                * Complex::new(
                                    j as f64 / length as f64 - 0.5,
                                    -(i as f64) / length as f64 + 0.5,
                                )
                                + origin,
                            *max_iterations,
                        ),
                        *max_iterations,
                    ),
                );
            }
        }
    }
}

fn main() {
    let length = 600;
    let initial_origin = Complex::new(-0.5, 0.0);
    let initial_extent = 2.5;
    let initial_max_iterations = 250;
    let max_extent = 4.0 * initial_extent;

    let mut image = Image::new(length, length);

    let mut origin = initial_origin;
    let mut extent = initial_extent;
    let mut max_iterations = initial_max_iterations;
    let mut clock = Clock::start();
    let mut recalculate = true;
    let mut texture = Texture::new().unwrap();

    let font = Font::from_file("data/font.ttf").unwrap();

    let mut text = Text::new("", &font, 24);
    text.set_fill_color(Color::WHITE);
    text.set_outline_thickness(2.0);
    text.set_outline_color(Color::BLACK);
    text.set_position((10.0, 5.0));

    let mut window = RenderWindow::new(
        (length, length),
        "Mandelbrot",
        Style::CLOSE,
        &Default::default(),
    );
    window.set_framerate_limit(60);

    while window.is_open() {
        while let Some(ev) = window.poll_event() {
            match ev {
                Event::Closed => {
                    window.close();
                }
                Event::KeyPressed { code, .. } => {
                    match code {
                        Key::Escape => window.close(),
                        Key::Up => origin = Complex::new(origin.re, origin.im + extent / 25.0),
                        Key::Down => origin = Complex::new(origin.re, origin.im - extent / 25.0),
                        Key::Left => origin = Complex::new(origin.re - extent / 25.0, origin.im),
                        Key::Right => origin = Complex::new(origin.re + extent / 25.0, origin.im),
                        Key::W => extent /= 1.5,
                        Key::S => extent = f64::min(extent * 1.2, max_extent),
                        Key::R => {
                            origin = initial_origin;
                            extent = initial_extent;
                            max_iterations = initial_max_iterations;
                        }
                        Key::RBracket => max_iterations += 25,
                        Key::LBracket => max_iterations = cmp::max(max_iterations - 25, 25),
                        _ => {}
                    }
                    recalculate = true;
                }
                Event::MouseButtonPressed { x, y, .. } => {
                    origin += extent
                        * Complex::new(
                            x as f64 / window.size().x as f64 - 0.5,
                            -(y as f64) / window.size().y as f64 + 0.5,
                        );
                    recalculate = true;
                }
                Event::MouseWheelScrolled { delta, .. } => {
                    if delta > 0.0 {
                        extent /= 1.2;
                    } else if delta < 0.0 {
                        extent = f64::min(extent * 1.2, max_extent);
                    }
                    recalculate = true;
                }
                _ => {}
            }
        }

        window.clear(Color::BLACK);

        if recalculate {
            recalculate = false;

            render_rows(&mut image, &extent, &origin, &max_iterations, 0, length);

            texture
                .load_from_image(&image, Rect::<i32>::new(0, 0, length as i32, length as i32))
                .unwrap();
        }

        window.draw(&Sprite::with_texture(&texture));
        window.draw(&text);
        window.display();

        text.set_string(&format!(
            "{:.0} fps\n{} iters\n{:.1e}",
            1.0 / clock.restart().as_seconds(),
            max_iterations,
            initial_extent / extent
        ));
    }
}
