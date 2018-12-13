use std::collections::HashSet;
use std::fs;

struct Device {
  
}

fn main() {

  // let mut frequency: i32 = 0;
  let input_file = fs::read_to_string("input.txt").unwrap();

  let mut frequencies = HashSet::new();
  let (frequency, frequencies) = input_file
    .lines()
    .fold((0, &mut frequencies), |acc, x| {
        let sum = acc.0 + x.trim().parse::<i32>().unwrap();
        acc.1.insert(sum);
        (sum, acc.1)
    });

  println!("frequency (in a cool functional way): {}", frequency);

  // 'outer: loop {
  //   for line in input_file.lines() {
  //     let delta: i32 = match line.trim().parse() {
  //       Ok(parsed) => parsed,
  //       Err(_) => {
  //         println!("Didn't get a number, just gonna keep going");
  //         continue;
  //       }
  //     };
  //     frequency += delta;

  //     //twice will be false if it's already in the set
  //     let twice = frequencies.insert(frequency);
  //     if !twice {
  //       println!("the thing we got twice is: {}", frequency);
  //       break 'outer;
  //     }
  //   }
  //   if first_time_through {
  //     println!("frequency: {}", frequency);
  //   }
  //   first_time_through = false;
  // }
}

