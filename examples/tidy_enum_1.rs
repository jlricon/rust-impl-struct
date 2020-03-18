struct Config {
    do_foo_op: bool,
    conditional_add: Option<u32>,
    append_and_prepend: Option<String>,
}
type Data = (u32, u32, String);
enum Operation {
    FooOp,
    ConditionalAdd,
    PrependAndAppend,
}
#[derive(Copy, Clone)]
enum Step {
    First,
    Second,
}
fn foo_op(d: &mut Data, c: &Config, step: Step) {
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
fn conditional_add(d: &mut Data, c: &Config, step: Step) {
    match step {
        Step::First => {
            if let Some(val) = c.conditional_add {
                d.1 += val;
            }
        }
        Step::Second => {}
    }
}
fn prepend_and_append(d: &mut Data, c: &Config, step: Step) {
    match step {
        Step::First => {
            if let Some(val) = &c.append_and_prepend {
                d.2 = format!("{}{}", val, d.2);
            }
        }
        Step::Second => {
            if let Some(val) = &c.append_and_prepend {
                d.2 = format!("{}{}", d.2, val);
            }
        }
    }
}
impl Operation {
    fn apply(&self, d: &mut Data, c: &Config, step: Step) {
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
fn transform(mut d: Data, c: &Config) -> Data {
    let ops = [
        Operation::ConditionalAdd,
        Operation::FooOp,
        Operation::PrependAndAppend,
    ];
    [Step::First, Step::Second].iter().for_each(|step| {
        ops.iter().for_each(|op| op.apply(&mut d, c, *step));
    });

    d
}
fn main() {
    let d = (0, 0, "data_id".to_owned());
    let config = Config {
        do_foo_op: true,
        conditional_add: Some(3),
        append_and_prepend: Some("stuff".to_owned()),
    };
    let _result = transform(d, &config);
}
// We do get some jumping https://godbolt.org/z/dvd846 (203 lines)
// With manual unrolling of the loop https://godbolt.org/z/HctnMk (195)
