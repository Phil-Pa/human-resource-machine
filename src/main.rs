use std::env;

mod machine;
mod parser;

use crate::parser::InstructionParser;

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

    let parser = InstructionParser::new_from_file(&args[1]).unwrap();

    let mut machine = parser.parse().unwrap();
    machine.enable_logging = enable_logging;
    let outbox = machine.run(&inbox).unwrap();

    println!(
        "instructions: {}",
        machine.get_instruction_count() as usize + outbox.len()
    );

    println!("{:?}", outbox);
}

#[cfg(test)]
mod tests {
    use crate::parser::InstructionParser;

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

        let parser = InstructionParser::new_from_str(program);
        let mut machine = parser.parse().unwrap();
        let outbox = machine.run(&[5]).unwrap();
        assert_eq!(1, outbox.len());
        assert_eq!(15, outbox[0]);
    }
}
