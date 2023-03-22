use colors_transform::{Color, Rgb};
use std::f64;
use std::f64::consts::PI;
use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use wasm_bindgen::JsCast;
use web_sys::CanvasRenderingContext2d;

#[macro_use]
extern crate lazy_static;

mod lsystem;
use lsystem::LSystemImpl;

struct Line {
    x: f64,
    y: f64,
    angle: f64,
}

// Called when the wasm module is instantiated
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    Ok(())
}

#[wasm_bindgen(js_name = generateCanvas)]
pub fn generate_canvas(fractal_type: u32, iterations: u32, color1: String, color2: String) {
    let (context, width, height) = get_canvas();

    let lsystem = LSystemImpl::new(fractal_type);
    let canvas_scaling = lsystem.get_canvas_scaling();
    let scale = if width < 600 {
        height as f64 * 1.5
    } else {
        width as f64
    };
    let sequence = lsystem.expand(iterations);
    let length = canvas_scaling.length * scale / iterations as f64;
    let (mut x, mut y) = (0.0, 0.0);
    let mut angle = lsystem.get_angle();
    let angle_rad = -1.0 * PI * angle / 180.0;

    context
        .translate(
            canvas_scaling.width * width as f64,
            canvas_scaling.height * height as f64,
        )
        .unwrap();
    context.clear_rect(0.0, 0.0, width as f64, height as f64);
    context.move_to(0.0, 0.0);
    context.rotate(canvas_scaling.initial_angle).unwrap();
    let mut stack = vec![];

    for seq in sequence.chars() {
        match seq {
            'F' | 'A' | 'B' => {
                x += length * angle.cos();
                y += length * angle.sin();
                context.line_to(x, y);
                context.stroke();
            }
            '+' => {
                angle += angle_rad;
            }
            '-' => {
                angle -= angle_rad;
            }
            '[' => {
                stack.push(Line { x, y, angle });
            }
            ']' => {
                let line = stack.pop().unwrap();
                (x, y, angle) = (line.x, line.y, line.angle);
                context.move_to(x, y);
            }
            _ => continue,
        }
    }
    let image_data = context
        .get_image_data(0.0, 0.0, width as f64, height as f64)
        .unwrap();
    let mut data = image_data.data();
    let c1 = Rgb::from_hex_str(&color1).unwrap();
    let c2 = Rgb::from_hex_str(&color2).unwrap();
    for index in 0..(width * height * 4) as usize {
        if data[index] > 0 {
            let fraction = index as f32 / (height * width * 4) as f32;
            data[index - 3] = (c1.get_red() + (c2.get_red() - c1.get_red()) * fraction) as u8;
            data[index - 2] = (c1.get_green() + (c2.get_green() - c1.get_green()) * fraction) as u8;
            data[index - 1] = (c1.get_blue() + (c2.get_blue() - c1.get_blue()) * fraction) as u8;
        }
    }
    let slice_data = Clamped(&data.0[..]);
    let image_data = web_sys::ImageData::new_with_u8_clamped_array_and_sh(
        slice_data,
        width as u32,
        height as u32,
    )
    .unwrap();
    context.put_image_data(&image_data, 0.0, 0.0).unwrap();
}

fn get_canvas() -> (CanvasRenderingContext2d, i32, i32) {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let canvas_div = document
        .get_element_by_id("canvasdiv")
        .unwrap()
        .dyn_into::<web_sys::HtmlElement>()
        .unwrap();

    let height = canvas_div.client_height();
    let width = canvas_div.client_width();

    canvas.set_width(width as u32);
    canvas.set_height(height as u32);

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();
    (context, width, height)
}
