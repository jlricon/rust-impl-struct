/// An example of the problem we want to overcome
/// Don't code this way!

struct Config {
    do_foo_op: bool,
    conditional_add: Option<u32>,
    append_and_prepend: Option<String>,
}
type Data = (u32, u32, String);
fn foo_op(d: &mut Data) {
    d.0 += 1;
    d.1 *= 2;
}
// Problem: The behaviour is decoupled from the 'flag'. It's hard to know what do_foo_op is doing
// One'd have to read all the code and find all the if statements. There may be many functions
// that call each other and this may be hard
fn transform(mut d: Data, c: &Config) -> Data {
    // Step 1
    if c.do_foo_op {
        foo_op(&mut d);
    };
    if let Some(val) = &c.append_and_prepend {
        d.2 = format!("{}{}", val, d.2);
    }
    if let Some(val) = c.conditional_add {
        d.1 += val;
    }
    // Step 2
    if c.do_foo_op {
        d.0 += 1;
    }
    if let Some(val) = &c.append_and_prepend {
        d.2 = format!("{}{}", d.2, val);
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
