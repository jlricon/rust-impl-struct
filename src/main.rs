/// An example of the problem we want to overcome
/// Don't code this way!

struct Config {
    do_foo_op: bool,
    conditional_add: Option<u32>,
}
type Data = (u32, u32, String);

trait Operation {
    fn step_one(&self, d: &mut Data, c: &Config) {}
    fn step_two(&self, d: &mut Data, c: &Config) {}
}
struct FooOp;
struct ConditionalAdd;
impl Operation for FooOp {
    fn step_one(&self, d: &mut Data, c: &Config) {
        if c.do_foo_op {
            d.0 += 1;
            d.1 *= 2;
        }
    }

    fn step_two(&self, d: &mut Data, c: &Config) {
        if c.do_foo_op {
            d.0 += 1;
        }
    }
}
impl Operation for ConditionalAdd {
    fn step_one(&self, d: &mut Data, c: &Config) {
        if let Some(val) = c.conditional_add {
            d.1 += val;
        }
    }
}
fn step_one<T: Operation>(op: T, d: &mut Data, c: &Config) {
    op.step_one(d, c)
}
fn step_two<T: Operation>(op: T, d: &mut Data, c: &Config) {
    op.step_two(d, c)
}
// Using traits, we can cleanly split what each operation does; all its behaviour is collected in one place
fn transform(mut d: Data, c: &Config) -> Data {
    let ops: [Box<Operation>; 2] = [Box::new(FooOp), Box::new(ConditionalAdd)];
    // Step 1
    ops.iter().for_each(|op| step_one(**op, &mut d, c));
    // Step 2
    ops.iter().for_each(|op| step_two(**op, &mut d, c));
    d
}
fn main() {
    let d = (0, 0, "data_id".to_owned());
    let config = Config {
        do_foo_op: true,
        conditional_add: Some(3),
    };
    let _result = transform(d, &config);
}
