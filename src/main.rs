use rand::Rng;
use time::Instant;

fn main()
{
	let mut rng = rand::thread_rng();
	let goal:i16 = 20; // has to be hard-coded, for some reason taking user input makes it ~33% slower
	let rgoal:i16 = goal+1; // for some reason, using 1..=goal in the random function makes it run ~55% slower
	let mut total_highest:i16 = 0;
	let mut instance_highest:i16 = 0;
	let mut iters:u128 = 0;
	let start:Instant = Instant::now();

	while instance_highest != goal {
		iters += 1;
		if rng.gen_range(1..rgoal) == instance_highest+1 {
			instance_highest += 1;
			if instance_highest > total_highest {
				total_highest = instance_highest;
				println!("{}/{} in {} seconds and {} iters | {} it/s",total_highest,goal,start.elapsed().as_seconds_f32(),iters,iters as f64/start.elapsed().as_seconds_f32() as f64);
			}
		} else {instance_highest = 0;}
	}
}