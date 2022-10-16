use rand::Rng;
use time::Instant;

fn main()
{
	let mut rng = rand::thread_rng();
	let goal:i16 = 20;
	let rgoal:i16 = goal+1;
	let mut p_highest:i16 = 0;
	let mut highest:i16 = 0;
	let mut iters:u128 = 0;
	let start: Instant = Instant::now();
	while highest != goal {
		iters += 1;
		if rng.gen_range(1..rgoal) == highest+1 {
			highest += 1;
			if highest > p_highest {
				p_highest = highest;
				println!("{}/{} in {} seconds and {} iters | {} it/s",p_highest,goal,start.elapsed().as_seconds_f32(),iters,iters as f64/start.elapsed().as_seconds_f32() as f64);
			}
		} else {highest = 0;}
	}
}