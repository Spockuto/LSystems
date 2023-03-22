use std::collections::HashMap;
use std::f64::consts::PI;

pub struct CanvasScaling {
    pub initial_angle: f64,
    pub length: f64,
    pub height: f64,
    pub width: f64,
}

struct LSystem {
    variables: &'static str,
    axiom: &'static str,
    rules: HashMap<char, &'static str>,
    angle: f64,
    max_rounds: u32,
    canvas: CanvasScaling,
}

pub struct LSystemImpl<'a>(&'a LSystem);

impl LSystemImpl<'_> {
    pub fn new(fractal_type: u32) -> Self {
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
        Self(system)
    }

    pub fn expand(&self, iterations: u32) -> String {
        if iterations > self.0.max_rounds {
            panic!("Max limit reached");
        }

        let mut sequence = String::new();
        for i in 0..iterations {
            if i == 0 {
                sequence.insert_str(0, self.0.axiom);
            } else {
                let sequence_copy = sequence.to_string();
                let mut insert_index = 0;
                for identifier in sequence_copy.chars() {
                    if !self.0.variables.contains(identifier) {
                        insert_index += 1;
                        continue;
                    }
                    let rule = self.0.rules.get(&identifier).unwrap();
                    sequence.remove(insert_index);
                    sequence.insert_str(insert_index, rule);
                    insert_index += &rule.len();
                }
            }
        }
        sequence
    }

    pub fn get_canvas_scaling(&self) -> &CanvasScaling {
        &self.0.canvas
    }

    pub fn get_angle(&self) -> f64 {
        self.0.angle
    }
}

lazy_static! {
    static ref BARNSLEY_FERN: LSystem = LSystem {
        variables: "XF",
        axiom: "X",
        rules: vec![('X', "F-[[X]+X]+F[+FX]-X"), ('F', "FF"),]
            .into_iter()
            .collect(),
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
        variables: "XY",
        axiom: "FX",
        rules: vec![('X', "X+YF+"), ('Y', "-FX-Y")].into_iter().collect(),
        angle: 90.0,
        max_rounds: 12,
        canvas: CanvasScaling {
            initial_angle: 60.0,
            length: 0.08,
            height: 0.5,
            width: 0.8
        }
    };
}

lazy_static! {
    static ref SEGMENT_32: LSystem = LSystem {
        variables: "F",
        axiom: "F+F+F+F",
        rules: vec![(
            'F',
            "-F+F-F-F+F+FF-F+F+FF+F-F-FF+FF-FF+F+F-FF-F-F+FF-F-F+F+F-F+"
        )]
        .into_iter()
        .collect(),
        angle: 90.0,
        max_rounds: 3,
        canvas: CanvasScaling {
            initial_angle: 90.0,
            length: 0.01,
            height: 0.5,
            width: 0.75
        }
    };
}

lazy_static! {
    static ref FRACTAL_PLANT: LSystem = LSystem {
        variables: "F",
        axiom: "F",
        rules: vec![('F', "FF-[-F+F+F]+[+F-F-F]")].into_iter().collect(),
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
        variables: "F",
        axiom: "F+F+F+F",
        rules: vec![('F', "F+F-F-FF+F+F-F")].into_iter().collect(),
        angle: 90.0,
        max_rounds: 4,
        canvas: CanvasScaling {
            initial_angle: 90.0,
            length: 0.015,
            height: 0.5,
            width: 0.8
        }
    };
}

// both A, B means move forward
lazy_static! {
    static ref PEANO_GOSPER: LSystem = LSystem {
        variables: "AB",
        axiom: "A",
        rules: vec![('A', "A-B--B+A++AA+B-"), ('B', "+A-BB--B-A++A+B")]
            .into_iter()
            .collect(),
        angle: 60.0,
        max_rounds: 5,
        canvas: CanvasScaling {
            initial_angle: 60.0,
            length: 0.035,
            height: 0.3,
            width: 0.75
        }
    };
}

lazy_static! {
    static ref HILBERT_CURVE: LSystem = LSystem {
        variables: "XY",
        axiom: "X",
        rules: vec![('X', "+YF-XFX-FY+"), ('Y', "-XF+YFY+FX-")]
            .into_iter()
            .collect(),
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
        variables: "XY",
        axiom: "XYXYXYX+XYXYXYX+XYXYXYX+XYXYXYX",
        rules: vec![('F', ""), ('X', "FX+FX+FXFY-FY-"), ('Y', "+FX+FXFY-FY-FY")]
            .into_iter()
            .collect(),
        angle: 90.0,
        max_rounds: 4,
        canvas: CanvasScaling {
            initial_angle: 45.0,
            length: 0.02,
            height: 0.3,
            width: 0.85
        }
    };
}

lazy_static! {
    static ref SIERPINSKI_TRIANGLE: LSystem = LSystem {
        variables: "XF",
        axiom: "FXF--FF--FF",
        rules: vec![('X', "--FXF++FXF++FXF--"), ('F', "FF")]
            .into_iter()
            .collect(),
        angle: 60.0,
        max_rounds: 7,
        canvas: CanvasScaling {
            initial_angle: -60.0,
            length: 0.025,
            height: 0.01,
            width: 0.01
        }
    };
}

lazy_static! {
    static ref SIERPINSKI_SQUARE: LSystem = LSystem {
        variables: "F",
        axiom: "F+F+F+F",
        rules: vec![('F', "FF+F+F+F+FF")].into_iter().collect(),
        angle: 90.0,
        max_rounds: 5,
        canvas: CanvasScaling {
            initial_angle: -90.0,
            length: 0.025,
            height: 0.8,
            width: 0.1
        }
    };
}

lazy_static! {
    static ref FRACTAL_PLANT_2: LSystem = LSystem {
        variables: "FVWXYZ",
        axiom: "VZFFF",
        rules: vec![
            ('F', "F"),
            ('V', "[+++W][---W]YV"),
            ('W', "+X[-W]Z"),
            ('X', "-W[+X]Z"),
            ('Y', "YZ"),
            ('Z', "[-FFF][+FFF]F")
        ]
        .into_iter()
        .collect(),
        angle: 18.0,
        max_rounds: 11,
        canvas: CanvasScaling {
            initial_angle: -PI / 4.0,
            length: 0.1,
            height: 0.8,
            width: 0.5
        }
    };
}

lazy_static! {
    static ref KOCH_SNOWFLAKE: LSystem = LSystem {
        variables: "F",
        axiom: "F++F++F",
        rules: vec![('F', "F-F++F-F")].into_iter().collect(),
        angle: 60.0,
        max_rounds: 6,
        canvas: CanvasScaling {
            initial_angle: -60.0,
            length: 0.01,
            height: 0.7,
            width: 0.15
        }
    };
}
