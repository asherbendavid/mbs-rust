use eframe::egui;

const MAX_ITER: u32 = 100;

// Fixed view area for Phase 1 (centered on the most "interesting" part of the set)
const VIEW_MIN_X: f64 = -2.0;
const VIEW_MAX_X: f64 = 1.0;
const VIEW_MIN_Y: f64 = -1.5;
const VIEW_MAX_Y: f64 = 1.5;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "Mandelbrot Set Viewer",
        options,
        Box::new(|_cc| Ok(Box::new(MbsApp::default()))),
    )
}

#[derive(Default)]
struct MbsApp;

impl eframe::App for MbsApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let size = ui.available_size();
        let width = size.x.round() as usize;
        let height = size.y.round() as usize;

        if width == 0 || height == 0 {
            return;
        }

        let image = render_mandelbrot(width, height);

        let texture = ui.ctx().load_texture(
            "mandelbrot",
            image,
            egui::TextureOptions::default(),
        );

        ui.image(&texture);
    }
}

fn render_mandelbrot(width: usize, height: usize) -> egui::ColorImage {
    let mut pixels = Vec::with_capacity(width * height * 4);

    for py in 0..height {
        for px in 0..width {
            // Map pixel coordinates to the complex plane
            let x0 = VIEW_MIN_X + (px as f64 / width as f64) * (VIEW_MAX_X - VIEW_MIN_X);
            let y0 = VIEW_MIN_Y + (py as f64 / height as f64) * (VIEW_MAX_Y - VIEW_MIN_Y);

            let iterations = mandelbrot_iterations(x0, y0);
            let color = iterations_to_color(iterations);

            pixels.extend_from_slice(&color);
        }
    }

    egui::ColorImage::from_rgba_unmultiplied([width, height], &pixels)
}

fn mandelbrot_iterations(x0: f64, y0: f64) -> u32 {
    let mut x = 0.0;
    let mut y = 0.0;
    let mut iteration = 0;

    while x * x + y * y <= 4.0 && iteration < MAX_ITER {
        let x_new = x * x - y * y + x0;
        let y_new = 2.0 * x * y + y0;
        x = x_new;
        y = y_new;
        iteration += 1;
    }

    iteration
}

fn iterations_to_color(iterations: u32) -> [u8; 4] {
    if iterations == MAX_ITER {
        [0, 0, 0, 255] // Inside the set: black
    } else {
        // Outside the set: simple grayscale based on escape speed
        let brightness = (iterations as f32 / MAX_ITER as f32 * 255.0) as u8;
        [brightness, brightness, brightness, 255]
    }
}