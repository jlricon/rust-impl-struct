struct Config {
    do_foo_op: bool,
    conditional_add: Option<u32>,
}
type Data = (u32, u32, String);
enum Operation {
    FooOp,
    ConditionalAdd,
}
impl Operation {
    fn step_one(&self, d: &mut Data, c: &Config) {
        match self {
            Self::FooOp => {
                if c.do_foo_op {
                    d.0 += 1;
                    d.1 *= 2;
                }
            }
            Self::ConditionalAdd => {
                if let Some(val) = c.conditional_add {
                    d.1 += val;
                }
            }
        }
    }
    fn step_two(&self, d: &mut Data, _c: &Config) {
        match self {
            Self::FooOp => {
                d.0 += 1;
            }
            Self::ConditionalAdd => {}
        }
    }
}
// Now we have an enum, no more dynamic dispatch!
// But the code looks kind of weird, imagine if we had lots of lines up there in the impl of the enum
// It would quickly become hard to read and we'd like to make functions, but what do we do?
// Do we create files? It's an option, but it add more lines to the code
// Also, with traits we have to implement them (If we leave the default case unimplemented) while for enums
// the fact that we have to manually write lots of matches increases the odds for a lazy _ => default case
fn transform(mut d: Data, c: &Config) -> Data {
    let ops: Vec<Operation> = vec![Operation::ConditionalAdd, Operation::FooOp];
    // Step 1
    ops.iter().for_each(|op| op.step_one(&mut d, c));
    // Step 2
    ops.iter().for_each(|op| op.step_two(&mut d, c));

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
