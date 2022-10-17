use time::Instant;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::{env,process::exit};
use std::io::{self,Write,stdin};

fn actual_run(goal:i8)
{
  let mut rng:ChaCha8Rng = ChaCha8Rng::from_entropy();
	let rgoal:i8 = goal+1;
	let mut total_highest:i8 = 0;
	let mut instance_highest:i8 = 0;
	let mut rolls:u128 = 0;
	let start:Instant = Instant::now();

	while instance_highest != goal
  {
		rolls += 1;
		if rng.gen_range(1..rgoal) == instance_highest+1
    {
			instance_highest += 1;
			if instance_highest > total_highest
      {
				total_highest = instance_highest;
				println!("{}/{} in {} seconds and {} rolls | {} rolls/s",total_highest,goal,start.elapsed().as_seconds_f32(),rolls,rolls as f64/start.elapsed().as_seconds_f64());
			}
		} else {instance_highest = 0;}
	}
}

fn benchmark(goal:i8,acc:i8)
{
  let mut rng:ChaCha8Rng = ChaCha8Rng::seed_from_u64(7);
	let rgoal:i8 = goal+1;
	let mut total_highest:i8 = 0;
	let mut instance_highest:i8 = 0;
	let mut rolls:u128 = 0;
	let start:Instant = Instant::now();

	while instance_highest != goal
  {
		rolls += 1;
		if rng.gen_range(1..rgoal) == instance_highest+1
    {
			instance_highest += 1;
			if instance_highest > total_highest
      {
				total_highest = instance_highest;
        print!("");
        if total_highest == acc
        {
          println!("benchmark complete: {} rolls/s",rolls as f64/start.elapsed().as_seconds_f64());
          exit(0);
        }
			}
		} else {instance_highest = 0;}
	}
}

fn main()
{
  let arg:String = env::args().nth(1).unwrap_or("".to_string());
	print!("d");
	io::stdout().flush().unwrap();
	let mut inp:String = String::new();
	stdin().read_line(&mut inp).expect("please enter number of sides");
	let goal:i8 = inp.trim().parse::<i8>().unwrap();

  if arg == "--benchmark"
  {
    println!("running benchmark...");
    benchmark(goal,7)
  }
  else if arg == "--high-acc-benchmark"
  {
    println!("running high accuracy benchmark...");
    benchmark(goal,8)
  }
  else {actual_run(goal)}
}