struct Config {
    do_foo_op: bool,
    conditional_add: Option<u32>,
    append_and_prepend: Option<f32>,
}
type Data = (u32, u32, f32);
enum Operation {
    FooOp,
    ConditionalAdd,
    PrependAndAppend,
}
enum Step {
    First,
    Second,
}
fn foo_op(d: &mut Data, c: &Config, step: &Step) {
    match step {
        Step::First => {
            if c.do_foo_op {
                d.0 += 1;
                d.1 *= 2;
            }
        }
        Step::Second => {
            d.0 += 1;
        }
    }
}
fn conditional_add(d: &mut Data, c: &Config, step: &Step) {
    match step {
        Step::First => {
            if let Some(val) = c.conditional_add {
                d.1 += val;
            }
        }
        Step::Second => {}
    }
}
fn prepend_and_append(d: &mut Data, c: &Config, step: &Step) {
    match step {
        Step::First => {
            if let Some(val) = &c.append_and_prepend {
                d.2 += *val;
            }
        }
        Step::Second => {
            if let Some(val) = &c.append_and_prepend {
                d.2 -= *val;
            }
        }
    }
}
impl Operation {
    fn apply(&self, d: &mut Data, c: &Config, step: &Step) {
        match self {
            Self::FooOp => foo_op(d, c, step),
            Self::ConditionalAdd => conditional_add(d, c, step),
            Self::PrependAndAppend => prepend_and_append(d, c, step),
        }
    }
}
// Now we have achieved everything we wanted:
// 1. No dynamic dispatch
// 2. Group the behaviours by function
fn transform(d: &mut Data, c: &Config) {
    let ops = [
        Operation::ConditionalAdd,
        Operation::FooOp,
        Operation::PrependAndAppend,
    ];
    [Step::First, Step::Second].iter().for_each(|step| {
        ops.iter().for_each(|op| op.apply(d, c, step));
    });
}
fn main() {
    let mut d = (0, 0, 2.3);
    let config = Config {
        do_foo_op: true,
        conditional_add: Some(3),
        append_and_prepend: Some(1.2),
    };
    transform(&mut d, &config);
}
// Look, no jumps!
// https://godbolt.org/z/TFf5dv
