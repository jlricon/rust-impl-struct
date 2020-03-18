struct Config {
    do_foo_op: bool,
    conditional_add: Option<u32>,
    append_and_prepend: Option<String>,
}
type Data = (u32, u32, String);

fn append_and_prepend(d: &mut Data, c: &Config) {
    if let Some(val) = &c.append_and_prepend {
        d.2 = format!("{}{}", val, d.2);
    }
    // Rest
    if let Some(val) = c.conditional_add {
        d.1 += val;
    };
    if let Some(val) = &c.append_and_prepend {
        d.2 = format!("{}{}", d.2, val);
    }
}
fn transform(mut d: Data, c: &Config) -> Data {
    if c.do_foo_op {
        d.0 += 1;
        d.1 *= 2;
    }
    // Rest
    append_and_prepend(&mut d, c);
    if c.do_foo_op {
        d.0 += 1;
    };
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
