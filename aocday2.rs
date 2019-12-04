fn main() {
    let mut v = vec![1,0,0,3,1,1,2,3,1,3,4,3,1,5,0,3,2,6,1,19,1,19,10,23,2,13,23,27,1,5,27,31,2,6,31,35,1,6,35,39,2,39,9,43,1,5,43,47,1,13,47,51,1,10,51,55,2,55,10,59,2,10,59,63,1,9,63,67,2,67,13,71,1,71,6,75,2,6,75,79,1,5,79,83,2,83,9,87,1,6,87,91,2,91,6,95,1,95,6,99,2,99,13,103,1,6,103,107,1,2,107,111,1,111,9,0,99,2,14,0,0];
    let target = 19690720;
    
    for i in 0..99 {
        for j in 0..99 {
            v[1] = i;
            v[2] = j;
            if(process_intcode(&mut v) == target) {
                println!("answer: {}", 100*i+j);
            }
        }
    }
    
}

//refactored so it does a copy instead
fn process_intcode( intcode : &mut Vec<i32>) -> i32 {
    let mut copy = intcode.clone();

    for index in (0..copy.len()).step_by(4) {
        if copy[index] == 99 {
            return copy[0];
        }
        else {
            let operand_1_pos = copy[index + 1] as usize;
            let operand_1_real = copy[operand_1_pos];
            let operand_2_pos = copy[index + 2] as usize;
            let operand_2_real = copy[operand_2_pos];
            let out_pos = (index + 3) as usize;
            let out_real = copy[out_pos] as usize;
            match copy[index] {
                1 => {
                    copy[out_real] = operand_1_real + operand_2_real;
                },
                2 => {
                    copy[out_real] = operand_1_real * operand_2_real;
                },
                _ => println!("should never happen"),
            }
        }
    }
    return 0;
}
