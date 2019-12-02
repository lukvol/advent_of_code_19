use std::fs;

fn main() {
    let content = fs::read_to_string("./input.txt").unwrap();
    let mut program: Vec<usize> = content.split(',').map(|x| x.parse().unwrap()).collect();

    program[1] = 12;
    program[2] = 2;

    let solution_part_one = execute_program(program.clone());
    println!("Solution for part 1: {}", solution_part_one);
    
    let (noun, verb) = find_noun_and_verb(program.clone());
    println!("Solution for part 2: {}", 100 * noun + verb);
}

fn find_noun_and_verb(mut program: Vec<usize>) -> (usize, usize) {
    for i in 0..100 {
        for j in 0..100 {
            program[1] = i;
            program[2] = j;

            let solution = execute_program(program.clone());
            if solution == 19690720 {
                return (i, j)
            }
        }
    }
    return (0,0)
}

fn execute_program(mut program: Vec<usize>) -> usize {
    let mut i = 0;
    while i < program.len() {
        let instruction_code = program[i];
        if instruction_code == 99 {
            break;
        }

        if program.len() - i < 3 {
            println!("not enough arguments!");
        }
        i += 1; let source_position_1 = program[i];
        i += 1; let source_position_2 = program[i];
        i += 1; let target_position = program[i];

        if instruction_code == 1 {
            program[target_position] = program[source_position_1] + program[source_position_2];
        } else if instruction_code == 2 {
            program[target_position] = program[source_position_1] * program[source_position_2];
        }

        i += 1;
    }

    program[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input_for_second_part() {
        let mut program = vec![1,9,10,3,2,3,11,0,99,30,40,50];

        let result = execute_program(& mut program);
        
        assert_eq!(result, 3500);
    }
}