use crate::AdventError;


pub fn part1() -> Result<(), AdventError> {
    let card_pub = 10212254;
    let door_pub = 12577395;
    
    let (card_loop, door_loop) = break_code(7, card_pub, door_pub);

    println!("Card loop: {}", card_loop);
    println!("Door loop: {}", door_loop);

    let encryption_key = transform(door_pub, card_loop);
    println!("Key is {}", encryption_key);
    Ok(())
}

fn transform(subject_num: u64, loops: u64) -> u64 {
    let mut result = 1;
    let mut loops_remaining = loops;
    while loops_remaining > 0 {
        result = result * subject_num;
        result = result % 20201227;
        loops_remaining -= 1;
    }
    return result;
}
fn break_code(subject_num: u64, card_output: u64, door_output: u64) -> (u64, u64) {
    let mut current: u64 = 1;
    let mut loops = 0;

    let mut card_loops = 0;
    let mut door_loops = 0;

    loop {
        if loops > 10000000 {
            panic!("large number");
        }
        if current % 20201227 == card_output {
            card_loops = loops;
        }

        if current % 20201227 == door_output {
            door_loops = loops;
        }

        if card_loops > 0 && door_loops > 0 {
            return (card_loops, door_loops);
        }

        current = current * subject_num;
        if current > 20201227 {
            current = current % 20201227;
        }
        loops += 1;
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_example_one() {
        use super::*;
        let card_pub = 5764801;
        let door_pub = 17807724;

        assert_eq!((8, 11), break_code(7, card_pub, door_pub));
        assert_eq!(14897079, transform(door_pub, 8));
    }
}