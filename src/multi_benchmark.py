from subprocess import Popen,PIPE
from os.path import exists


if not exists('target/release/stupid-die-roller'):
	print('file not found, please run `cargo build --release`')


def benchmark(high_acc:bool):
	proc = Popen(['target/release/stupid-die-roller',f'--{"high-acc-" if high_acc else ""}benchmark'],stdout=PIPE,stdin=PIPE)
	proc.stdin.write(b'20\n')
	proc.stdin.flush()
	while True:
		line = proc.stdout.readline().decode()
		if line.startswith('benchmark complete:'):
			result = float(line.split(' ')[2])
			print(f'{result} rolls/s')
			return result

def main():
	high_acc = input('run high accuracy benchmark? [y/N] ').lower() in ['y','yes','true']
	loops = int(input('how many times would you like to run the benchmark? [5]') or 5)
	benchmarks = [benchmark(high_acc) for i in range(loops)]
	print(f'{"[HIGH-ACC] " if high_acc else ""}average of {loops} benchmarks: {sum(benchmarks)/loops}')
	
if __name__ == '__main__':
	main()