struct Config {
    do_foo_op: bool,
    conditional_add: Option<u32>,
    append_and_prepend: Option<String>,
}
type Data = (u32, u32, String);
struct FooOp;
struct AppendAndPrepend;
impl FooOp {
    pub fn step_one(d: &mut Data) {
        d.0 += 1;
        d.1 *= 2;
    }
    pub fn step_two(d: &mut Data) {
        d.0 += 1;
    }
}
impl AppendAndPrepend {
    pub fn step_one(d: &mut Data, val: &str) {
        d.2 = format!("{}{}", val, d.2);
    }
    pub fn step_two(d: &mut Data, val: &str) {
        d.2 = format!("{}{}", d.2, val);
    }
}
fn transform(mut d: Data, c: &Config) -> Data {
    // Step 1
    if c.do_foo_op {
        FooOp::step_one(&mut d);
    }
    if let Some(val) = &c.append_and_prepend {
        AppendAndPrepend::step_one(&mut d, val);
    }
    if let Some(val) = c.conditional_add {
        d.1 += val;
    }
    // Step 2
    if c.do_foo_op {
        FooOp::step_two(&mut d);
    }
    if let Some(val) = &c.append_and_prepend {
        AppendAndPrepend::step_two(&mut d, val);
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
