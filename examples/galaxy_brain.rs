struct Config {
    do_foo_op: bool,
    conditional_add: Option<u32>,
    append_and_prepend: Option<String>,
}
type Data = (u32, u32, String);
type Transformer = Option<(fn(d: &mut Data, c: &Config), fn(d: &mut Data, c: &Config))>;
type TransformerFactory = fn(c: &Config) -> Transformer;
fn foo_op(c: &Config) -> Transformer {
    if c.do_foo_op {
        fn step_one(d: &mut Data, _: &Config) {
            d.0 += 1;
            d.1 *= 2;
        }
        fn step_two(d: &mut Data, _: &Config) {
            d.0 += 1;
        }
        Some((step_one, step_two))
    } else {
        None
    }
}
fn append_prepend(c: &Config) -> Transformer {
    if c.append_and_prepend.is_some() {
        fn step_one(d: &mut Data, c: &Config) {
            if let Some(val) = &c.append_and_prepend {
                d.2 = format!("{}{}", val, d.2);
            }
        }
        fn step_two(d: &mut Data, c: &Config) {
            if let Some(val) = &c.append_and_prepend {
                d.2 = format!("{}{}", d.2, val);
            }
        }
        Some((step_one, step_two))
    } else {
        None
    }
}
fn conditional_add(c: &Config) -> Transformer {
    if c.conditional_add.is_some() {
        fn step_one(d: &mut Data, c: &Config) {
            if let Some(val) = c.conditional_add {
                d.1 += val;
            };
        }
        fn step_two(_: &mut Data, _: &Config) {}
        Some((step_one, step_two))
    } else {
        None
    }
}
const TRANSFORMS: [TransformerFactory; 3] = [foo_op, conditional_add, append_prepend];
fn transform(mut d: Data, c: &Config) -> Data {
    for transformer in TRANSFORMS.iter() {
        if let Some(t) = transformer(c) {
            t.0(&mut d, c);
        }
    }
    for transformer in TRANSFORMS.iter() {
        if let Some(t) = transformer(c) {
            t.1(&mut d, c);
        }
    }
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
