use std::borrow::Cow;
use std::collections::HashMap;
use std::f64;
use std::f64::consts::PI;

#[derive(Clone)]
struct Rule {
    identifier: char,
    rule: Cow<'static, str>,
}

pub struct CanvasScaling {
    pub initial_angle: f64,
    pub length: f64,
    pub height: f64,
    pub width: f64,
}

struct LSystem {
    variables: Cow<'static, str>,
    axiom: Cow<'static, str>,
    rules: Vec<Rule>,
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
        let rules = self
            .0
            .rules
            .clone()
            .into_iter()
            .map(|r| (r.identifier, r.rule))
            .collect::<HashMap<_, Cow<'_, str>>>();

        for i in 0..iterations {
            if i == 0 {
                sequence.insert_str(0, &self.0.axiom);
            } else {
                let sequence_copy = sequence.to_string();
                let mut insert_index = 0;
                for identifier in sequence_copy.chars() {
                    if !self.0.variables.contains(identifier) {
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

    pub fn get_canvas_scaling(&self) -> &CanvasScaling {
        &self.0.canvas
    }

    pub fn get_angle(&self) -> f64 {
        self.0.angle
    }
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
            length: 0.08,
            height: 0.5,
            width: 0.8
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
            length: 0.01,
            height: 0.5,
            width: 0.75
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
            length: 0.015,
            height: 0.5,
            width: 0.8
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
            width: 0.75
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
            height: 0.3,
            width: 0.85
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
            length: 0.025,
            height: 0.01,
            width: 0.01
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
            width: 0.1
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
            initial_angle: -PI / 4.0,
            length: 0.1,
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
            width: 0.15
        }
    };
}
