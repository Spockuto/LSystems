use colors_transform::{Color, Rgb};
use std::borrow::Cow;
use std::collections::HashMap;
use std::f64;
use std::f64::consts::PI;
use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use wasm_bindgen::JsCast;

#[macro_use]
extern crate lazy_static;

#[derive(Clone)]
struct Rule {
    identifier: char,
    rule: Cow<'static, str>,
}

struct CanvasScaling {
    initial_angle: f64,
    length: f64,
    height: f64,
    width: f64,
}

struct LSystem {
    variables: Cow<'static, str>,
    axiom: Cow<'static, str>,
    rules: Vec<Rule>,
    angle: f64,
    max_rounds: u32,
    canvas: CanvasScaling,
}

struct Line {
    x: f64,
    y: f64,
    angle: f64,
}

lazy_static! {
    static ref BARNSLEY_FERN: LSystem = LSystem {
        variables: Cow::Borrowed("XF"),
        axiom: Cow::Borrowed("X"),
        rules: vec![
            Rule {
                identifier: 'X',
                rule: Cow::Borrowed("F-[[X]+X]+F[+FX]-X")
            },
            Rule {
                identifier: 'F',
                rule: Cow::Borrowed("FF")
            }
        ],
        angle: 22.5,
        max_rounds: 7,
        canvas: CanvasScaling {
            initial_angle: PI / 3.0,
            length: 0.025,
            height: 1.0,
            width: 0.5
        }
    };
}

lazy_static! {
    static ref DRAGON_CURVE: LSystem = LSystem {
        variables: Cow::Borrowed("XY"),
        axiom: Cow::Borrowed("FX"),
        rules: vec![
            Rule {
                identifier: 'X',
                rule: Cow::Borrowed("X+YF+")
            },
            Rule {
                identifier: 'Y',
                rule: Cow::Borrowed("-FX-Y")
            }
        ],
        angle: 90.0,
        max_rounds: 12,
        canvas: CanvasScaling {
            initial_angle: 60.0,
            length: 0.1,
            height: 0.5,
            width: 0.5
        }
    };
}

lazy_static! {
    static ref SEGMENT_32: LSystem = LSystem {
        variables: Cow::Borrowed("F"),
        axiom: Cow::Borrowed("F+F+F+F"),
        rules: vec![Rule {
            identifier: 'F',
            rule: Cow::Borrowed("-F+F-F-F+F+FF-F+F+FF+F-F-FF+FF-FF+F+F-FF-F-F+FF-F-F+F+F-F+")
        }],
        angle: 90.0,
        max_rounds: 3,
        canvas: CanvasScaling {
            initial_angle: 90.0,
            length: 0.013,
            height: 0.5,
            width: 0.6
        }
    };
}

lazy_static! {
    static ref FRACTAL_PLANT: LSystem = LSystem {
        variables: Cow::Borrowed("F"),
        axiom: Cow::Borrowed("F"),
        rules: vec![Rule {
            identifier: 'F',
            rule: Cow::Borrowed("FF-[-F+F+F]+[+F-F-F]")
        }],
        angle: 22.5,
        max_rounds: 5,
        canvas: CanvasScaling {
            initial_angle: PI / 3.0,
            length: 0.045,
            height: 0.9,
            width: 0.5
        }
    };
}

lazy_static! {
    static ref KOCH_ISLAND: LSystem = LSystem {
        variables: Cow::Borrowed("F"),
        axiom: Cow::Borrowed("F+F+F+F"),
        rules: vec![Rule {
            identifier: 'F',
            rule: Cow::Borrowed("F+F-F-FF+F+F-F")
        }],
        angle: 90.0,
        max_rounds: 4,
        canvas: CanvasScaling {
            initial_angle: 90.0,
            length: 0.025,
            height: 0.5,
            width: 0.6
        }
    };
}

// both A, B means move forward
lazy_static! {
    static ref PEANO_GOSPER: LSystem = LSystem {
        variables: Cow::Borrowed("AB"),
        axiom: Cow::Borrowed("A"),
        rules: vec![
            Rule {
                identifier: 'A',
                rule: Cow::Borrowed("A-B--B+A++AA+B-")
            },
            Rule {
                identifier: 'B',
                rule: Cow::Borrowed("+A-BB--B-A++A+B")
            }
        ],
        angle: 60.0,
        max_rounds: 5,
        canvas: CanvasScaling {
            initial_angle: 60.0,
            length: 0.035,
            height: 0.3,
            width: 0.5
        }
    };
}

lazy_static! {
    static ref HILBERT_CURVE: LSystem = LSystem {
        variables: Cow::Borrowed("XY"),
        axiom: Cow::Borrowed("X"),
        rules: vec![
            Rule {
                identifier: 'X',
                rule: Cow::Borrowed("+YF-XFX-FY+")
            },
            Rule {
                identifier: 'Y',
                rule: Cow::Borrowed("-XF+YFY+FX-")
            }
        ],
        angle: 90.0,
        max_rounds: 7,
        canvas: CanvasScaling {
            initial_angle: -90.0,
            length: 0.035,
            height: 0.7,
            width: 0.3
        }
    };
}

lazy_static! {
    static ref FREC_FRACTAL: LSystem = LSystem {
        variables: Cow::Borrowed("XY"),
        axiom: Cow::Borrowed("XYXYXYX+XYXYXYX+XYXYXYX+XYXYXYX"),
        rules: vec![
            Rule {
                identifier: 'F',
                rule: Cow::Borrowed("")
            },
            Rule {
                identifier: 'X',
                rule: Cow::Borrowed("FX+FX+FXFY-FY-")
            },
            Rule {
                identifier: 'Y',
                rule: Cow::Borrowed("+FX+FXFY-FY-FY")
            }
        ],
        angle: 90.0,
        max_rounds: 4,
        canvas: CanvasScaling {
            initial_angle: 45.0,
            length: 0.02,
            height: 0.5,
            width: 0.5
        }
    };
}


lazy_static! {
    static ref SIERPINSKI_TRIANGLE: LSystem = LSystem {
        variables: Cow::Borrowed("XF"),
        axiom: Cow::Borrowed("FXF--FF--FF"),
        rules: vec![
            Rule {
                identifier: 'X',
                rule: Cow::Borrowed("--FXF++FXF++FXF--")
            },
            Rule {
                identifier: 'F',
                rule: Cow::Borrowed("FF")
            }
        ],
        angle: 60.0,
        max_rounds: 7,
        canvas: CanvasScaling {
            initial_angle: -60.0,
            length: 0.03,
            height: 0.2,
            width: 0.2
        }
    };
}

lazy_static! {
    static ref SIERPINSKI_SQUARE: LSystem = LSystem {
        variables: Cow::Borrowed("F"),
        axiom: Cow::Borrowed("F+F+F+F"),
        rules: vec![Rule {
            identifier: 'F',
            rule: Cow::Borrowed("FF+F+F+F+FF")
        }],
        angle: 90.0,
        max_rounds: 5,
        canvas: CanvasScaling {
            initial_angle: -90.0,
            length: 0.025,
            height: 0.8,
            width: 0.2
        }
    };
}

lazy_static! {
    static ref FRACTAL_PLANT_2: LSystem = LSystem {
        variables: Cow::Borrowed("FVWXYZ"),
        axiom: Cow::Borrowed("VZFFF"),
        rules: vec![
            Rule {
                identifier: 'F',
                rule: Cow::Borrowed("F")
            },
            Rule {
                identifier: 'V',
                rule: Cow::Borrowed("[+++W][---W]YV")
            },
            Rule {
                identifier: 'W',
                rule: Cow::Borrowed("+X[-W]Z")
            },
            Rule {
                identifier: 'X',
                rule: Cow::Borrowed("-W[+X]Z")
            },
            Rule {
                identifier: 'Y',
                rule: Cow::Borrowed("YZ")
            },
            Rule {
                identifier: 'Z',
                rule: Cow::Borrowed("[-FFF][+FFF]F")
            }
        ],
        angle: 18.0,
        max_rounds: 11,
        canvas: CanvasScaling {
            initial_angle: - PI / 4.0,
            length: 0.15,
            height: 0.8,
            width: 0.5
        }
    };
}


lazy_static! {
    static ref KOCH_SNOWFLAKE: LSystem = LSystem {
        variables: Cow::Borrowed("F"),
        axiom: Cow::Borrowed("F++F++F"),
        rules: vec![Rule {
            identifier: 'F',
            rule: Cow::Borrowed("F-F++F-F")
        }],
        angle: 60.0,
        max_rounds: 6,
        canvas: CanvasScaling {
            initial_angle: -60.0,
            length: 0.01,
            height: 0.7,
            width: 0.2
        }
    };
}


fn expand_lsystem(system: &LSystem, iterations: u32) -> String {
    if iterations > system.max_rounds {
        panic!("Max limit reached");
    }

    let mut sequence = String::new();
    let rules = system
        .rules
        .clone()
        .into_iter()
        .map(|r| (r.identifier, r.rule))
        .collect::<HashMap<_, Cow<'_, str>>>();

    for i in 0..iterations {
        if i == 0 {
            sequence.insert_str(0, &system.axiom);
        } else {
            let sequence_copy = sequence.to_string();
            let mut insert_index = 0;
            for identifier in sequence_copy.chars() {
                if !system.variables.contains(identifier) {
                    insert_index += 1;
                    continue;
                }
                let rule = rules.get(&identifier).unwrap();
                sequence.remove(insert_index);
                sequence.insert_str(insert_index, rule);
                insert_index += &rule.len();
            }
        }
    }
    sequence
}

// Called when the wasm module is instantiated
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    Ok(())
}

#[wasm_bindgen(js_name = generateCanvas)]
pub fn generate_canvas(fractal_type: u32, iterations: u32, color1: String, color2: String) {
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
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    let system = match fractal_type {
        1 => &*BARNSLEY_FERN,
        2 => &*DRAGON_CURVE,
        3 => &*SEGMENT_32,
        4 => &*FRACTAL_PLANT,
        5 => &*KOCH_ISLAND,
        6 => &*PEANO_GOSPER,
        7 => &*HILBERT_CURVE,
        8 => &*FREC_FRACTAL,
        9 => &*SIERPINSKI_TRIANGLE,
        10 => &*SIERPINSKI_SQUARE,
        11 => &*FRACTAL_PLANT_2,
        12 => &*KOCH_SNOWFLAKE,
        _ => panic!("Type unknown"),
    };

    let scale = if width < 600 { height * 2 } else { width };

    context
        .translate(
            system.canvas.width * width as f64,
            system.canvas.height * height as f64,
        )
        .unwrap();

    context.clear_rect(0.0, 0.0, width as f64, height as f64);
    context.move_to(0.0, 0.0);
    //let default_color = Rgb::from_hex_str(&"4DFE44").unwrap();

    let sequence = expand_lsystem(system, iterations);
    let length = system.canvas.length * scale as f64 / iterations as f64;
    let (mut x, mut y) = (0.0, 0.0);
    let mut angle = system.angle;
    let angle_m = PI * angle / 180.0;
    let angle_p = -1.0 * PI * angle / 180.0;

    context.rotate(system.canvas.initial_angle).unwrap();
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
                angle += angle_p;
            }
            '-' => {
                angle += angle_m;
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
