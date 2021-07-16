use std::env;
use std::io::*;
use std::{fs::File, path::Path};

mod machine;

use machine::{Instruction, Machine};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        println!("not enough args");
        return;
    }

    let enable_logging = args[2].parse::<u32>().expect("cannot parse to number") == 1;
    let inbox = args
        .iter()
        .skip(3)
        .map(|x| x.parse().expect("cannot parse to number"))
        .collect::<Vec<i32>>();

    let mut machine = Machine::new_from_file(&args[1], enable_logging).unwrap();
    let outbox = machine.run(&inbox);

    println!(
        "instructions: {}",
        machine.get_instruction_count() as usize + outbox.len()
    );

    if enable_logging {
        println!("{:?}", outbox);
    }
}

#[cfg(test)]
mod tests {

    use super::machine::*;
    use super::*;

    #[test]
    fn test_sum() {
        // calculates 1 + 2 + 3 + 4 + 5 + ... + n where n comes from inbox
        let program = r"
copyfrom 9
copyto 1
copyto 2
bump+ 1
inbox
copyto 2
label 1
sub 1
copyto 3
add 2
copyto 2
copyfrom 3
jumpzero 2
jump 1
label 2
copyfrom 2
outbox";

        let lines = string_to_lines(program);
        let mut machine = Machine::new(get_instructions(lines), 10, false);
        let outbox = machine.run(&[5]);
        assert_eq!(1, outbox.len());
        assert_eq!(15, outbox[0]);
    }
}
