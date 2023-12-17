use crate::prelude::*;
/**/
// https://adventofcode.com/2023/day/15
/**/

pub fn calculate_part1() -> Result<usize>{
    let instructions = include_str!("../input/day15.txt").strip_suffix('\n').unwrap().split(',');

    Ok( instructions.map(|str| hash_str(str) as usize).sum() )
}

fn hash_str(str: &str) -> u8 {
    let mut initial = 0;
    for char in str.chars() {
        initial = hash_char(initial, char);
    }
    initial
}

fn hash_char(mut initial: u8, chr: char) -> u8 {
    let ascii_num = chr as u8;
    initial = initial.wrapping_add(ascii_num);
    initial = initial.wrapping_mul(17);
    initial
}

pub fn calculate_part2() -> Result<usize>{
    let instructions = include_str!("../input/day15.txt").strip_suffix('\n').unwrap().split(',');

    const N_BOXES: usize = 256;
    let mut boxes: [Vec<Lens>; N_BOXES] = vec![Vec::new(); N_BOXES].try_into().unwrap();

    for instruction_str in instructions {
        let instr = parse_instruction(instruction_str)?;
        let boxx = &mut boxes[hash_str(instr.label) as usize];
        match instr.typ {
            InstructionType::Add => {
                let new_lens = Lens::new(instr.label, instr.value.unwrap());
                // Update if already exists, otherwise add new
                if let Some(pos) = boxx.iter().position(|current_lens| current_lens.label == new_lens.label ) {
                    boxx[pos] = new_lens;
                }
                else { boxx.push(new_lens); }
            },
            InstructionType::Remove => 
                boxx.retain(|lens|  lens.label != instr.label),
        };
    }

    let mut total = 0;
    for (box_idx, boxx) in boxes.into_iter().enumerate() {
        for (lens_idx, lens) in boxx.into_iter().enumerate() {
            let lens_value = (box_idx+1)*(lens_idx+1)*(lens.value) as usize;
            total += lens_value;
        }
    }

    Ok(total)
}

fn parse_instruction(str: &str) -> Result<Instruction> {
    println!("{str}");
    if let Some((label, _)) = str.split_once('-') {
        Ok( Instruction{label, typ: InstructionType::Remove, value: None} )
    }
    else if let Some((label, value_as_str)) = str.split_once('=') {
        Ok( Instruction{label, typ: InstructionType::Add, value: Some(value_as_str.parse()?)} )
    }
    else { bail!("Unexpected string: {str}") }
}

#[derive(Debug, Clone)]
struct Lens<'a> {
    label: &'a str,
    value: u8,
}
impl<'a> Lens<'a> {
    fn new(label: &'a str, value: u8) -> Self {
        Lens{label, value}
    }
}

struct Instruction<'a> {
    label: &'a str,
    typ: InstructionType,
    value: Option<u8>,
}

enum InstructionType {
    Add,
    Remove,
}